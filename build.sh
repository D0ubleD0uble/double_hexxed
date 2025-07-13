#!/bin/bash
set -e

# Clean previous output
rm -rf dist/

# Build the project to WebAssembly
cargo build --target wasm32-unknown-unknown --release

# Run wasm-bindgen on the output binary
wasm-bindgen target/wasm32-unknown-unknown/release/double_hexxed.wasm \
  --out-dir dist/pkg \
  --target web

# Copy static files to dist
cp index.html dist/
cp -r assets dist/
