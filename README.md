# Double_Hexxed

## Status
Currently this project is still considered to be in alpha status.

## Development setup
1. rustup target install wasm32-unknown-unknown
2. cargo install wasm-pack
3. cargo install basic-http-server

## Local Testing
1. wasm-pack build --target web --release
2. basic-http-server .