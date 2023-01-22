const fs = require('fs');
const path = require('path')

const pkgModName = process.argv.slice(2)[0]; // hyphen names
const buildType /* : '-browser' | '-asmjs' | '-nodejs' */ = process.argv.slice(2)[1];

const pathToRepo = path.join(__dirname, '..', pkgModName, 'wasm');
const oldPkg = require(`${pathToRepo}/publish/package.json`);

const repoName = `cml`;
const baseHyphenName = repoName.replaceAll('_', '-');
const underscorePkgModName = pkgModName.replaceAll('-', '_');

if (oldPkg.name === `${baseHyphenName}-${pkgModName}-wasm`) {
  oldPkg.name = '@dcspark/' + oldPkg.name + buildType;
}
if (buildType === '-browser' || buildType === '-asmjs') {
  // due to a bug in wasm-pack, this file is missing from browser builds
  const missingFile = `${repoName}_${underscorePkgModName}_bg.js`;
  if (oldPkg.files.find(entry => entry === missingFile) == null) {
    oldPkg.files.push(missingFile);
  }
}
if (buildType === '-asmjs') {
  // need to replace WASM with ASM package 
  const missingFile = `${repoName}_${underscorePkgModName}_bg.wasm`;
  oldPkg.files = [
    `${repoName}_${underscorePkgModName}.asm.js`,
    ...oldPkg.files.filter(file => file !== `${repoName}_${underscorePkgModName}_bg.wasm`)
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
