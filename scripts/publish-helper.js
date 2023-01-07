const fs = require('fs');
const path = require('path')

const pkgModName = process.argv.slice(2)[0];
const modName = process.argv.slice(2)[1];
const buildType = process.argv.slice(2)[2];

const pathToRepo = path.join(__dirname, '..', modName.replaceAll('_', '-'));
const oldPkg = require(`${pathToRepo}/publish/package.json`);

const repoName = `cardano_multiplatform_lib_`;
const baseHypenName = repoName.replaceAll('_', '-');

if (oldPkg.name === `${baseHypenName}${modName}`) {
  oldPkg.name = '@dcspark/' + oldPkg.name + buildType;
}
if (buildType === '-browser' || buildType === '-asmjs') {
  // due to a bug in wasm-pack, this file is missing from browser builds
  const missingFile = `${repoName}${pkgModName}_bg.js`;
  if (oldPkg.files.find(entry => entry === missingFile) == null) {
    oldPkg.files.push(missingFile);
  }
}
if (buildType === '-asmjs') {
  // need to replace WASM with ASM package 
  const missingFile = `${repoName}${pkgModName}_bg.wasm`;
  oldPkg.files = [
    `${repoName}${pkgModName}.asm.js`,
    ...oldPkg.files.filter(file => file !== `${repoName}${pkgModName}_bg.wasm`)
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
