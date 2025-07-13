#!/bin/bash

# Clean previous output
rm -rf dist/

# Build the project to WebAssembly
wasm-pack build --target web --out-dir dist/pkg

# Copy static assets (HTML, CSS, JS) to dist
cp index.html dist/
cp -r assets dist/

# Make it executable
chmod +x build.sh