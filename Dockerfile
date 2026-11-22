FROM rust:slim-trixie

COPY rust-toolchain.toml /tmp/rust-toolchain.toml

RUN set -eux; \
    channel="$(sed -n 's/^channel = "\(.*\)"/\1/p' /tmp/rust-toolchain.toml)"; \
    test -n "$channel"; \
    rustup toolchain install "$channel" --profile minimal; \
    cargo "+$channel" install --locked just; \
    cargo "+$channel" install --locked cargo-nextest; \
    rm /tmp/rust-toolchain.toml

ENV HOME=/tmp \
    CARGO_HOME=/tmp/cargo \
    CARGO_TARGET_DIR=/tmp/target

WORKDIR /app/test

USER 1001

CMD ["sh", "-c", "just install && just test"]
