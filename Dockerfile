FROM rust:slim-trixie

COPY rust-toolchain.toml /tmp/rust-toolchain.toml

RUN cargo install --locked just cargo-nextest

ENV HOME=/tmp \
    CARGO_HOME=/tmp/cargo \
    CARGO_TARGET_DIR=/tmp/target

WORKDIR /app/test

USER 1001

CMD ["sh", "-c", "just install && just test"]
