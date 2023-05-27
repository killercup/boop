#! /bin/bash

set -ex

name=boop

mkdir -pv wasm
rm -rf wasm/assets
cp -r assets wasm
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --out-name "$name" --out-dir wasm --target web "${CARGO_TARGET_DIR}/wasm32-unknown-unknown/release/${name}.wasm"
pushd wasm
wasm-opt -Oz -o "${name}_bg.wasm" "${name}_bg.wasm"
popd

