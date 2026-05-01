FROM rust:1.95-slim-trixie
RUN apt-get update && apt-get install -y pkg-config libssl-dev git build-essential && rm -rf /var/lib/apt/lists/*
WORKDIR /workspace
