#!/bin/bash
set -e

cargo install wasm-bindgen-cli

rm -rf dist/
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --target web --out-dir dist/pkg target/wasm32-unknown-unknown/release/*.wasm

cp index.html dist/
cp -r assets dist/
