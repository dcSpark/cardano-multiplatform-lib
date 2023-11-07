const fs = require('fs');
const path = require('path')

const crateName = process.argv.slice(2)[0];
const pathToRepo = path.join(__dirname, '..', crateName, 'wasm');
const hyphenRepoName = process.argv.slice(2)[1].replaceAll('_', '-');
const underscoreRepoName = hyphenRepoName.replaceAll('-', '_');

const inputFile = fs.readFileSync(`${pathToRepo}/pkg/${underscoreRepoName}.d.ts`, 'utf8').split(/\r?\n/);

//console.log(inputFile);
let currentClass = null;
for (let i = 0; i < inputFile.length; ++i) {
  let line = inputFile[i];
  //const r = /export class ([a-zA-Z]+){/.exec(line);
  const classDef = /export class(.*){/.exec(line);
  if (classDef != null && classDef.length > 1) {
    currentClass = classDef[1].trim();
    //console.log(`reading class ${currentClass}`);
    continue;
  }
  //const toJson = /\sto_json\(\): any;/.exec(line);
  //console.log(toJson);
  inputFile[i] = line.replace(/(\s?to_js_value\(\)\s?:\s?)(any)(;)/, `$1${currentClass}JSON$3`);
  if (line != inputFile[i]) {
    continue;
  }
  // TODO: we might want to make sure we don't have other cases where this would replace
  // things it shouldn't. We'd have to do some go-back-a-few-lines replace to only do this
  // for to_json() comments.
  inputFile[i] = line.replace(/(\s?\*\s?\@returns\s\{)(any)(\})/, `$1${currentClass}JSON$3`);
  //const m = /(\s?\*\s?\@returns\s\{)(any)(\})/.exec(line);
  //console.log(`${m} | ${line}`);
}
const jsonDefs = fs.readFileSync('./json-gen/output/json-types.d.ts', 'utf8');
fs.writeFile(
  `${pathToRepo}/pkg/${underscoreRepoName}_wasm.d.ts`,
  `${inputFile.join('\n')}\n${jsonDefs}`,
  (err) => {
    if (err != null) {
      console.log(`err writing file: ${err}`)
    }
  }
);
