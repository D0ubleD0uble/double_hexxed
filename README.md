# Double_Hexxed

## Requirements
1. rustup target install wasm32-unknown-unknown
2. cargo install wasm-server-runner

## Run wasm-server locally
cargo run --target wasm32-unknown-unknown

## Automatic updating during development
cargo watch -cx run
