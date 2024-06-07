FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye
RUN rustup target add thumbv7em-none-eabihf
RUN rustup component add llvm-tools
RUN cargo install cargo-binutils
