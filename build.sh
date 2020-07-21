set RUSTFLAGS=-C link-arg=-s
cargo build --target wasm32-unknown-unknown --release
mkdir -p output
cp target/wasm32-unknown-unknown/release/UID_discovery_rust.wasm output/
wasm2wat output/UID_discovery_rust.wasm -o output/temp.wat
wat2wasm output/temp.wat -o output/UID_discovery_rust_optimized.wasm