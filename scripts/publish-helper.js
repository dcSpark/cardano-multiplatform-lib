const fs = require('fs');
const oldPkg = require('../publish/package.json');

const flowFile = 'cardano_multiplatform_lib.js.flow';
if (oldPkg.files.find(entry => entry === flowFile) == null) {
  oldPkg.files.push(flowFile);
}
if (oldPkg.name === 'cardano-multiplatform-lib') {
  oldPkg.name = '@dcSpark/' + oldPkg.name + process.argv.slice(2)[0];
}
if (process.argv.slice(2)[0] === '-browser' || process.argv.slice(2)[0] === '-asmjs') {
  // due to a bug in wasm-pack, this file is missing from browser builds
  const missingFile = 'cardano_multiplatform_lib_bg.js';
  if (oldPkg.files.find(entry => entry === missingFile) == null) {
    oldPkg.files.push(missingFile);
  }
}
if (process.argv.slice(2)[0] === '-asmjs') {
  // need to replace WASM with ASM package 
  const missingFile = 'cardano_multiplatform_lib_bg.wasm';
  oldPkg.files = [
    'cardano_multiplatform_lib.asm.js',
    ...oldPkg.files.filter(file => file !== 'cardano_multiplatform_lib_bg.wasm')
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
