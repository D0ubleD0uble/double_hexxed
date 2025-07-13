# Double_Hexxed

## Status
Currently this project is still considered to be in alpha status.

## Development setup
1. rustup target install wasm32-unknown-unknown
2. cargo install wasm-server-runner

## Run wasm-server locally
cargo watch -cx "run --target wasm32-unknown-unknown" --env "WASM_SERVER_RUNNER_CUSTOM_INDEX_HTML=[path]\double_hexxed\index.html"