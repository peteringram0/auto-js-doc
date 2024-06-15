#
# Build stage
#
FROM ubuntu:latest as builder

# Setup
RUN apt-get -y update && apt-get -y install build-essential curl pkg-config clang

# Install Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

COPY ./ ./usr/src/app
WORKDIR /usr/src/app

# Build the rust project
RUN cargo build --release

#

#
# Host Stage
#
FROM ubuntu

# Install deps
RUN apt-get -y update && apt-get -y install curl unzip

# Download and install Bun
RUN curl -fsSL https://bun.sh/install | bash

# Add Bun to PATH
ENV PATH="/root/.bun/bin:$PATH"

# Copy in bun app
COPY ./site ./usr/local/bin

# Copy in rust app
COPY --from=builder /usr/src/app/target/release/auto-js-doc /usr/local/bin

WORKDIR /usr/local/bin

EXPOSE 3000

# Run bun server
CMD ["bun", "run", "index.ts"]
