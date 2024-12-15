# Start from a minimal Debian base image
FROM debian:bullseye-slim

# Set up environment
ENV DEBIAN_FRONTEND=noninteractive
WORKDIR /workspace

# Install essential tools for building Rust and WebAssembly
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    curl \
    ca-certificates \
    git \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Add WebAssembly target and install wasm-bindgen
RUN rustup target add wasm32-unknown-unknown \
    && cargo install wasm-bindgen-cli

# Clean up unused files to reduce image size
RUN apt-get remove -y curl \
    && apt-get autoremove -y \
    && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

# Default command
CMD [ "bash" ]
