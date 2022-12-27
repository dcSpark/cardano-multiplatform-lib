const fs = require('fs');
const oldPkg = require('./publish/package.json');

const pkgModName = process.argv.slice(2)[0];
const modName = process.argv.slice(2)[1];
const buildType = process.argv.slice(2)[2];

if (oldPkg.name === `cardano-multiplatform-lib-${modName}`) {
  oldPkg.name = '@dcspark/' + oldPkg.name + buildType;
}
if (buildType === '-browser' || buildType === '-asmjs') {
  // due to a bug in wasm-pack, this file is missing from browser builds
  const missingFile = `cardano_multiplatform_lib_${pkgModName}_bg.js`;
  if (oldPkg.files.find(entry => entry === missingFile) == null) {
    oldPkg.files.push(missingFile);
  }
}
if (buildType === '-asmjs') {
  // need to replace WASM with ASM package 
  const missingFile = `cardano_multiplatform_lib_${pkgModName}_bg.wasm`;
  oldPkg.files = [
    `cardano_multiplatform_lib_${pkgModName}.asm.js`,
    ...oldPkg.files.filter(file => file !== `cardano_multiplatform_lib_${pkgModName}_bg.wasm`)
  ];
}

oldPkg.repository = {
  type: "git",
  url: "git+https://github.com/dcSpark/cardano-multiplatform-lib.git"
};
oldPkg.author = "dcSpark";
oldPkg.license = "MIT";
console.log(oldPkg);
fs.writeFileSync('./publish/package.json', JSON.stringify(oldPkg, null, 2));
