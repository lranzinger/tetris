#!/bin/bash

# Build the project with the wasm32-unknown-unknown target
cargo build --target wasm32-unknown-unknown

# Run wasm-bindgen to generate the bindings
wasm-bindgen --out-dir ./pkg --target web ./target/wasm32-unknown-unknown/debug/tetris.wasm
