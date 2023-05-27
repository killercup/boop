#! /bin/bash

set -ex

name=boop

mkdir -pv wasm
rm -rf wasm/assets
cp -r assets wasm
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --out-name bevy_game --out-dir wasm --target web $CARGO_TARGET_DIR/wasm32-unknown-unknown/release/$name.wasm
# wasm-opt -Oz -o boop.wasm boop.wasm
