FROM mcr.microsoft.com/devcontainers/rust:1-bookworm

# Avoid warnings by switching to noninteractive
ENV DEBIAN_FRONTEND=noninteractive

# Install base development tools and dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    curl \
    git \
    jq \
    make \
    pkg-config \
    libssl-dev \
    protobuf-compiler \
    python3 \
    python3-pip \
    python3-venv \
    udev \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Setup Rust targets and tools for vscode user
USER vscode
ENV HOME=/home/vscode
ENV PATH="${HOME}/.cargo/bin:${HOME}/.local/bin:${PATH}"

# Install rust nightly toolchain, components, and wasm target
RUN rustup toolchain install nightly \
    && rustup default stable \
    && rustup component add rustfmt clippy \
    && rustup component add rustfmt --toolchain nightly \
    && rustup component add clippy --toolchain nightly \
    && rustup target add wasm32-unknown-unknown \
    && cargo install cargo-audit --locked

# Install buf CLI for Protobuf schema linting and code generation
RUN BUF_VERSION="1.30.0" \
    && curl -sSL "https://github.com/bufbuild/buf/releases/download/v${BUF_VERSION}/buf-Linux-x86_64" -o "${HOME}/.local/bin/buf" \
    && chmod +x "${HOME}/.local/bin/buf"

# Install PlatformIO Core in isolated virtual environment for ESP32-S3 firmware
RUN python3 -m venv "${HOME}/.platformio-venv" \
    && "${HOME}/.platformio-venv/bin/pip" install --upgrade pip platformio \
    && ln -s "${HOME}/.platformio-venv/bin/pio" "${HOME}/.local/bin/pio" \
    && ln -s "${HOME}/.platformio-venv/bin/platformio" "${HOME}/.local/bin/platformio"

USER root
ENV DEBIAN_FRONTEND=dialog
