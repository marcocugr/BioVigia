FROM rust:slim-trixie

ENV HOME=/app \
    CARGO_HOME=/app/cargo \
    CARGO_TARGET_DIR=/app/target \
    PATH="/app/cargo/bin:${PATH}"

RUN mkdir -p /app/cargo /app/target /app/test /app/deps \
    && cargo install --locked just cargo-nextest

COPY Cargo.toml Cargo.lock* /app/deps/

RUN cargo fetch --manifest-path /app/deps/Cargo.toml \
    && rm -rf /app/deps

WORKDIR /app/test

USER 1001

CMD ["sh", "-c", "just install && just test"]
