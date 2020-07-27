#!/usr/bin/env bash
set RUSTFLAGS=-C link-arg=-s
cargo build --target wasm32-unknown-unknown --release
mkdir -p output
cp target/wasm32-unknown-unknown/release/DID_NEAR_rust.wasm output/
wasm2wat output/DID_NEAR_rust.wasm -o output/temp.wat
wat2wasm output/temp.wat -o output/DID_NEAR_rust_optimized.wasm