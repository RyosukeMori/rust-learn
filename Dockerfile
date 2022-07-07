FROM rust:latest

RUN mkdir app
WORKDIR /app

RUN cargo install trunk
RUN cargo install wasm-bindgen-cli

RUN cargo install cargo-watch

RUN rustup target add wasm32-unknown-unknown
RUN rustup component add rustfmt clippy