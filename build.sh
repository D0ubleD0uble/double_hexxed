#!/bin/bash
set -e

# Clean previous output
rm -rf dist/

# Build the project to WebAssembly
wasm-pack build --target web --out-dir dist/pkg

# Copy static files to dist
cp index.html dist/
cp -r assets dist/
