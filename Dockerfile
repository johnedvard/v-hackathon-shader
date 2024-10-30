# Use official rust image as base
FROM rust:slim

# Install necessary packages
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libssl-dev \
    pkg-config \
    make \
    curl \
    && rm -rf /var/lib/apt/lists

# Install Trunk.rs
RUN cargo install trunk

# Add target for wasm
RUN rustup target add wasm32-unknown-unknown


# Install WABT
RUN curl -L https://github.com/WebAssembly/wabt/releases/download/1.0.36/wabt-1.0.36-ubuntu-20.04.tar.gz | tar xz && \
mv wabt-1.0.36/bin/* /usr/local/bin/ && \
rm -rf wabt-1.0.36

# Install Binaryen
RUN curl -L https://github.com/WebAssembly/binaryen/releases/download/version_119/binaryen-version_119-x86_64-linux.tar.gz | tar xz && \
mv binaryen-version_119/bin/* /usr/local/bin/ && \
rm -rf binaryen-version_119

WORKDIR /usr/src/app

# Copy local project to container
COPY . .

# Build project into wasm
CMD ["make", "build"]