# Artimonist-wasm
Artimonist - A tool for generating mnemonics based on diagrams.

### Compile:
`export RUSTFLAGS='--cfg getrandom_backend="wasm_js"'`  
`wasm-pack build --target web`  
or `cargo build --lib --release --target wasm32-unknown-unknown`  