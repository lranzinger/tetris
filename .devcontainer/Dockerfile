# Start from a minimal Debian base image
FROM debian:bullseye-slim

# Set user
ARG USERNAME=dev
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# Create the user
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME

# Set up environment
ENV DEBIAN_FRONTEND=noninteractive

# Install essential tools for building Rust and WebAssembly
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    curl \
    ca-certificates \
    git \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

USER $USERNAME
WORKDIR /home/$USERNAME

# Set environment variables for Rust installation
ENV CARGO_HOME=/home/$USERNAME/.cargo
ENV RUSTUP_HOME=/home/$USERNAME/.rustup
ENV PATH="$CARGO_HOME/bin:$PATH"

# Install Rust for the user
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y

# Add WebAssembly target and install wasm-bindgen
RUN rustup target add wasm32-unknown-unknown \
    && cargo install wasm-bindgen-cli

WORKDIR /workspace

# Default command
CMD [ "bash" ]
