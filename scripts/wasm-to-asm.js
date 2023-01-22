const fs = require('fs')

const pkgModName = process.argv.slice(2)[0];

const repoName = `cml`;

const paths = [
  `./pkg/${repoName}_${pkgModName}_bg.js`,
  `./pkg/${repoName}_${pkgModName}.js`
]

paths.forEach((path) => {
  fs.readFile(path, 'utf8', (err,data) => {
    if (err) {
      return console.log(err);
    }

    const  result = data.replace(/_bg.wasm/g, '.asm.js');

    fs.writeFile(path, result, 'utf8', (err) => {
      if (err) return console.log(err);
    });
  });
})

fs.unlinkSync(`./pkg/${repoName}_${pkgModName}_bg.wasm`)
