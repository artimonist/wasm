# Artimonist-wasm
Artimonist - A tool for generating mnemonics based on diagrams.

### Compile:

**set envirment:**  
`export RUSTFLAGS='--cfg getrandom_backend="wasm_js"'`  

**compile for module usage:**  
`wasm-pack build --target web`  
_(`cargo build --lib --release --target wasm32-unknown-unknown`)_  


**compile for web worker:**  
`wasm-pack build --target no-modules` for web worker  


