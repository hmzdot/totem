# 1. This tells docker to use the Rust official image
FROM rust:1.80-alpine

# 2. Copy the files in your machine to the Docker image
COPY ./ ./

RUN apk add --no-cache musl-dev

# Build your program for release
RUN cargo build --release
