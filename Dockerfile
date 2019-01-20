FROM rustlang/rust:nightly-slim

WORKDIR /usr/src/

COPY ./ /usr/src/
RUN cargo build --release --target-dir ./dist
