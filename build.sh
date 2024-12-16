#!/bin/bash

# Build the project with the wasm32-unknown-unknown target
cargo build --target wasm32-unknown-unknown

mkdir -p /workspace/pkg

cp /workspace/target/wasm32-unknown-unknown/debug/tetris.wasm /workspace/