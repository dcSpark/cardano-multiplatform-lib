const fs = require('fs');
const json2ts = require('json-schema-to-typescript');
const path = require('path');

const SCRIPT = 'run-json2ts.js';

// LOCKSTEP (argument convention + root/wasm-dir resolution): the `--name=value` parser and the
// `resolveRoot`/`resolveWasmDir` pair below are mirrored verbatim in `json-ts-types.js`. The
// generator ships exactly these two self-contained files (`--package-json --json-schema-export`, or
// `--json-schema-scripts` on its own), so neither may `require` the other — a change to one must be
// mirrored into the other in the same commit.
function parseArgs(script, argv, known) {
  const out = {};
  for (const arg of argv) {
    const m = /^--([\w-]+)=(.*)$/.exec(arg);
    if (m == null || !known.includes(m[1])) {
      // A typo'd override must not silently fall back to the default and generate against the
      // wrong tree, so an unrecognized argument is fatal rather than ignored.
      console.error(
        `${script}: unrecognized argument ${JSON.stringify(arg)}. Supported: ` +
        known.map(k => `--${k}=<value>`).join(', '));
      process.exit(1);
    }
    out[m[1]] = m[2];
  }
  return out;
}

// Both shipped layouts place the scripts at `<root>/scripts/*.js`, so the root is derivable from
// the script's own location — no dependence on the caller's cwd.
function resolveRoot(override) {
  return override != null ? path.resolve(override) : path.resolve(__dirname, '..');
}

// `--package-json` nests the crates one level deeper (`<root>/rust/wasm`); the bare
// `--json-schema-scripts` layout puts the wasm crate at `<root>/wasm`.
function resolveWasmDir(script, root, override) {
  if (override != null) {
    const dir = path.resolve(override);
    if (!fs.existsSync(dir)) {
      console.error(`${script}: --wasm-dir=${dir} does not exist`);
      process.exit(1);
    }
    return dir;
  }
  const candidates = [path.join(root, 'rust', 'wasm'), path.join(root, 'wasm')];
  const found = candidates.find(candidate => fs.existsSync(candidate));
  if (found == null) {
    console.error(
      `${script}: could not find the wasm crate directory. Looked for:\n  ` +
      candidates.join('\n  ') +
      `\n(root resolved to ${root}). Pass --wasm-dir=<dir>, or --root=<dir>, to override.`);
    process.exit(1);
  }
  return found;
}
// end LOCKSTEP block

const args = parseArgs(SCRIPT, process.argv.slice(2), ['root', 'wasm-dir']);
const root = resolveRoot(args.root);
const wasmDir = resolveWasmDir(SCRIPT, root, args['wasm-dir']);

// Fail loudly and leave the last-good output alone; never write a partial `.d.ts`.
function fail(message) {
  console.error(`${SCRIPT}: ${message}`);
  process.exit(1);
}

const schemasDir = path.join(wasmDir, 'json-gen', 'schemas');
if (!fs.existsSync(schemasDir)) {
  fail(
    `no schema directory at ${schemasDir}. Run the json-gen crate ` +
    `(\`cd ${path.join(wasmDir, 'json-gen')} && cargo run\`) first, which writes the schema ` +
    `document this script compiles.`);
}
const outputFile = path.join(wasmDir, 'json-gen', 'output', 'json-types.d.ts');

// The json-gen crate writes exactly ONE document per crate — `<crate>.schema.json`, a pure `$defs`
// bundle threaded through a single schemars generator — so every type any other type references is
// a definition in the same file. That is what makes one `compile()` call correct: each definition
// becomes a top-level declaration, and the declared set cannot diverge from the referenced set.
const jsonFiles = fs.readdirSync(schemasDir).filter(file => path.extname(file) === '.json');
const documents = jsonFiles.filter(file => file.endsWith('.schema.json'));
const strays = jsonFiles.filter(file => !file.endsWith('.schema.json'));

if (documents.length !== 1) {
  fail(
    `expected exactly one *.schema.json document in ${schemasDir}, found ${documents.length}` +
    (documents.length > 0 ? `:\n  ${documents.join('\n  ')}` : '') +
    `\nAll .json files present:${jsonFiles.length > 0 ? `\n  ${jsonFiles.join('\n  ')}` : ' (none)'}`);
}
if (strays.length > 0) {
  // A leftover per-type schema from a pre-merge generator run. Ignoring it would keep a deleted
  // type shipping, so it is fatal and the fix is named.
  fail(
    `${strays.length} stale per-type schema file(s) in ${schemasDir}:\n  ${strays.join('\n  ')}\n` +
    `The json-gen crate writes one ${documents[0]} document and never deletes anything, so these ` +
    `are left over from an older generator run. Delete them and re-run the json-gen crate.`);
}

const documentFile = documents[0];
const document = JSON.parse(fs.readFileSync(path.join(schemasDir, documentFile), 'utf8'));
const sourceDefs = document.$defs || document.definitions;
if (sourceDefs == null || Object.keys(sourceDefs).length === 0) {
  // An empty JSON surface must not produce a green build: the wasm classes' `to_json_value()` would
  // silently stay `any` and nothing would say why.
  fail(`${documentFile} has no definitions ($defs). Refusing to write an empty ${outputFile}.`);
}

// `JSON` suffix so these names cannot collide with the wasm classes they describe. Applied to the
// `$defs` KEYS (and titles) BEFORE compiling, so json2ts emits the final names itself — renaming
// identifiers in its output afterwards also rewrites matching words inside doc comments and string
// literal unions.
const suffixed = (name) => `${name}JSON`;

// The document needs a root that references every definition, or json2ts prunes the ones nothing
// else points at (a schema root that no other type embeds — a large fraction of a real corpus).
// Stripped from the output again below.
const ROOT_TITLE = '__AllSchemas';

// Annotations sitting beside a `$ref` are legal in 2020-12, but json2ts reads the combination as a
// schema distinct from its target and emits a near-duplicate type (`FooJSON1`). They are pure
// documentation, so drop them and keep the reference bare. Known cost, accepted deliberately: a
// field-level doc comment on a `$ref`-typed field does not reach the emitted TypeScript.
const ANNOTATION_KEYWORDS =
  ['description', 'title', 'default', 'examples', 'deprecated', 'readOnly', 'writeOnly'];

const knownNames = new Set(Object.keys(sourceDefs));
const rewriteRefs = (node) => {
  if (Array.isArray(node)) {
    node.forEach(rewriteRefs);
    return;
  }
  if (node === null || typeof node !== 'object') return;
  if (typeof node.$ref === 'string') {
    const bare = node.$ref.replace(/^#\/(?:\$defs|definitions)\//, '');
    if (knownNames.has(bare)) {
      node.$ref = `#/$defs/${suffixed(bare)}`;
    }
    for (const keyword of ANNOTATION_KEYWORDS) delete node[keyword];
  }
  for (const [key, value] of Object.entries(node)) {
    if (key !== '$ref') rewriteRefs(value);
  }
};
// Ref rewriting runs BEFORE the per-definition title is written, so the annotation strip above can
// never delete the suffixed title this script is about to set.
rewriteRefs(sourceDefs);

const defs = Object.create(null);
for (const [name, body] of Object.entries(sourceDefs)) {
  // json2ts names a type from its `title` in preference to its `$defs` key, so the title has to
  // carry the suffix too — otherwise a definition that arrived with a title (from a Rust doc
  // comment, say) emits an unsuffixed name that merges with the wasm class of the same name.
  const def = { ...body, title: suffixed(name) };
  // Suppresses `[k: string]: unknown` on the emitted type, unless the definition really is a map.
  // Per-definition, because every definition becomes a top-level declaration here.
  if (typeof def.additionalProperties !== 'object') def.additionalProperties = false;
  defs[suffixed(name)] = def;
}

// TypeScript requires every named property to be assignable to the type's index signature, which
// is what json2ts renders `patternProperties` as. A schema declaring BOTH — named keys beside a
// pattern catch-all, the shape a CDDL open-map rest row (`* k => v`) flattened into a record
// produces — is valid JSON Schema, since the two key spaces are disjoint, but has no exact
// TypeScript equivalent: json2ts emits the named properties beside the index signature and every
// one of them fails TS2411 against it. Widen each catch-all to also accept the named property
// types: the emitted declaration is legal and loses only the disjointness. Recursive, because the
// combination is just as expressible (and just as uncompilable) on a nested object schema.
//
// An OPTIONAL named property needs one member more than its own schema. Under `strictNullChecks`
// the type checked against the index signature for `a?: T` is `T | undefined`, so the named schemas
// alone still leave TS2411 on `? foo: uint` beside a rest row. `tsType` is json2ts's escape hatch
// for emitting a raw TypeScript type verbatim, and is what puts `undefined` in the union. A
// property counts as optional unless `required` positively lists it, since `required` may be
// absent, not an array, or name keys that `properties` does not have.
//
// The wrap decision must only ever be evaluated on a node that IS a schema. The recursion stays
// generic (as `rewriteRefs` above is), but two keyword classes are handled by name so that every
// node it reaches is one. A schema MAP (`properties`, `patternProperties`, `$defs`, `definitions`,
// `dependentSchemas`, `dependencies`) is entered by its VALUES — the map itself is not a schema, and
// a consumer whose record has fields literally named `properties` and `patternProperties` would
// otherwise have that map satisfy the wrap condition and be silently rewritten. A data-bearing
// keyword (`enum`, `const`, `default`, `examples`) holds instance values, never schemas, so the walk
// stops there rather than treating an object literal as a schema. Everything else is a schema, an
// array of schemas, or a primitive that falls out at the top of the recursion — so no
// schema-position flag is threaded: by construction there is no other way in.
//
// The wrap runs AFTER the node's children are walked, so the `anyOf` built here — which holds the
// very objects `node.properties` holds — is never re-entered. That is what keeps a nested
// named-beside-pattern property from being wrapped a second time through its parent's catch-all.
const SCHEMA_MAP_KEYWORDS =
  ['properties', 'patternProperties', '$defs', 'definitions', 'dependentSchemas', 'dependencies'];
const DATA_KEYWORDS = ['enum', 'const', 'default', 'examples'];
const widenPatternCatchAlls = (node) => {
  if (Array.isArray(node)) {
    node.forEach(widenPatternCatchAlls);
    return;
  }
  if (node === null || typeof node !== 'object') return;
  for (const [key, value] of Object.entries(node)) {
    if (DATA_KEYWORDS.includes(key)) continue;
    if (SCHEMA_MAP_KEYWORDS.includes(key)) {
      if (value !== null && typeof value === 'object' && !Array.isArray(value)) {
        Object.values(value).forEach(widenPatternCatchAlls);
      }
    } else {
      widenPatternCatchAlls(value);
    }
  }
  if (node.patternProperties == null) return;
  const named = Object.entries(node.properties || {});
  if (named.length === 0) return;
  const members = named.map(([, schema]) => schema);
  if (named.some(([key]) => !Array.isArray(node.required) || !node.required.includes(key))) {
    members.push({ tsType: 'undefined' });
  }
  for (const [pattern, patternSchema] of Object.entries(node.patternProperties)) {
    node.patternProperties[pattern] = { anyOf: [patternSchema, ...members] };
  }
};
// `defs` is a map OF schemas, not a schema, so each definition is what enters the walk.
Object.values(defs).forEach(widenPatternCatchAlls);

const merged = {
  $schema: document.$schema || 'https://json-schema.org/draft/2020-12/schema',
  title: ROOT_TITLE,
  type: 'object',
  additionalProperties: false,
  properties: Object.fromEntries(
    Object.keys(defs).map((name) => [name, { $ref: `#/$defs/${name}` }])
  ),
  $defs: defs,
};

json2ts
  .compile(merged, ROOT_TITLE, { bannerComment: '', cwd: schemasDir })
  .then((ts) => {
    // Drop the synthetic root, keeping every real declaration.
    const withoutRoot = ts.replace(
      new RegExp(`^export interface ${ROOT_TITLE} \\{[\\s\\S]*?\\n\\}\\n?`, 'm'),
      ''
    );
    if (withoutRoot.includes(ROOT_TITLE)) {
      throw new Error(`${ROOT_TITLE} survived into the output; refusing to write it`);
    }
    const declared = (withoutRoot.match(/^export (?:type|interface) /gm) || []).length;
    if (declared === 0) {
      throw new Error(`no types produced from ${Object.keys(defs).length} definitions`);
    }
    fs.mkdirSync(path.dirname(outputFile), { recursive: true });
    fs.writeFileSync(outputFile, withoutRoot);
    console.log(`${SCRIPT}: ${declared} types from ${Object.keys(defs).length} definitions`);
  })
  .catch((e) => {
    // A document that fails to compile is a hard failure of the whole run, never a silently-dropped
    // type: the last-good `.d.ts` stays on disk and the build goes red.
    console.error(
      `${SCRIPT}: ${documentFile} failed to compile (schemas dir: ${schemasDir}). ` +
      `${outputFile} was left untouched.`);
    console.error(`  ${e && e.stack ? e.stack : e}`);
    process.exitCode = 1;
  });
