# Contributing

## Building

If you need to install Rust, do the following:

```bash
curl https://sh.rustup.rs -sSf | sh -s -- -y
echo 'export PATH=$HOME/.cargo/bin/:$PATH' >> $BASH_ENV
rustup install stable
rustup target add wasm32-unknown-unknown --toolchain stable
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

To build this repository, do the following:

```bash
git submodule update --init --recursive
nvm use
npm install
npm run rust:build-nodejs
```

## Testing

```bash
npm run rust:test
```

## Publishing \(only needed if you are an admin of this project\)

To publish a new version to [crates.io](https://crates.io)

```bash
npm run rust:publish
```

To publish new versions to NPM

```bash
npm run js:publish-nodejs
npm run js:publish-browser
npm run js:publish-asm
```
