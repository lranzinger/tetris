#!/bin/bash

# Parse build type argument
BUILD_TYPE=${1:-debug}  # Default to debug if no argument

# Help text
if [[ "$1" == "-h" || "$1" == "--help" ]]; then
    echo "Usage: ./build.sh [debug|release]"
    echo "  debug   - Build with debug symbols"
    echo "  release - Build with optimizations"
    exit 0
fi

# Validate build type
if [[ "$BUILD_TYPE" != "debug" && "$BUILD_TYPE" != "release" ]]; then
    echo "Error: Invalid build type. Use 'debug' or 'release'"
    exit 1
fi

# Build based on type
if [[ "$BUILD_TYPE" == "debug" ]]; then
    echo "Building debug version..."
    cargo build --target wasm32-unknown-unknown
    cp target/wasm32-unknown-unknown/debug/blocks.wasm .
else
    echo "Building release version..."
    cargo build --target wasm32-unknown-unknown --release
    
    echo "Optimizing with wasm-opt..."
    wasm-opt -O3 \
        --strip-debug \
        --strip-producers \
        -o blocks.wasm \
        target/wasm32-unknown-unknown/release/blocks.wasm
fi

echo "Build complete: $BUILD_TYPE"