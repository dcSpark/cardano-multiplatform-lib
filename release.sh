set -eu
if [ $1 = "prod" ];
then RELEASE_TYPE="prod"
elif [ $1 = "beta" ];
then RELEASE_TYPE="beta"
else
  echo "First parameter is expected 'prod' or 'beta'"
  return 1
fi

echo "Preparing ${RELEASE_TYPE} release"

. build-and-test.sh 

# publish on crates.io
cargo publish -p cml-core
cargo publish -p cml-crypto
cargo publish -p cml-chain
cargo publish -p cml-cip25
cargo publish -p cml-cip36
cargo publish -p cml-multi-era
cargo publish -p cml-core-wasm
cargo publish -p cml-crypto-wasm
cargo publish -p cml-chain-wasm
cargo publish -p cml-cip25-wasm
cargo publish -p cml-cip36-wasm
cargo publish -p cml-multi-era-wasm
cargo publish -p cardano-multiplatform-lib

# pubish on NPM
npm run js:publish-nodejs:${RELEASE_TYPE}
npm run js:publish-browser:${RELEASE_TYPE}
# asmjs builds broken for now
# npm run js:publish-asm:${RELEASE_TYPE}
