#!/bin/bash
set -e

# Clean previous output
rm -rf dist/

# Build with wasm-pack (outputs to ./pkg by default)
wasm-pack build --release --target web --out-dir dist/

# Copy static files to dist
cp index.html dist/
cp -r assets dist/
