default: build

build:
    cargo build

test:
    cargo test

wasm_build: 
    wasm-pack build --target web

wasm_publish: wasm_build
    wasm-pack publish
