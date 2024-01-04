const fs = require('fs');
const path = require('path')

const crateName = process.argv.slice(2)[0];
const hyphenRepoName = process.argv.slice(2)[1].replaceAll('_', '-');
const buildType /* : '-browser' | '-asmjs' | '-nodejs' */ = process.argv.slice(2)[2];

const underscoreRepoName = hyphenRepoName.replaceAll('-', '_');
const pathToRepo = path.join(__dirname, '..', crateName, 'wasm');
const oldPkg = require(`${pathToRepo}/publish/package.json`);

const packageNameRoot = hyphenRepoName.split("-wasm")[0];
oldPkg.name = '@dcspark/' + packageNameRoot + buildType;
if (buildType === '-browser' || buildType === '-asmjs') {
  // due to a bug in wasm-pack, this file is missing from browser builds
  const missingFile = `${underscoreRepoName}_bg.js`;
  if (oldPkg.files.find(entry => entry === missingFile) == null) {
    oldPkg.files.push(missingFile);
  }
}
if (buildType === '-asmjs') {
  // need to replace WASM with ASM package 
  const missingFile = `${underscoreRepoName}_bg.wasm`;
  oldPkg.files = [
    `${underscoreRepoName}.asm.js`,
    ...oldPkg.files.filter(file => file !== `${underscoreRepoName}_bg.wasm`)
  ];
}

oldPkg.repository = {
  type: "git",
  url: "git+https://github.com/dcSpark/cardano-multiplatform-lib.git"
};
oldPkg.author = "dcSpark";
oldPkg.license = "MIT";
console.log(oldPkg);
fs.writeFileSync(`${pathToRepo}/publish/package.json`, JSON.stringify(oldPkg, null, 2));
