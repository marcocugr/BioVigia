FROM rust:slim-trixie

ENV HOME=/app \
    CARGO_HOME=/app/cargo \
    CARGO_TARGET_DIR=/app/target \
    PATH="/app/cargo/bin:${PATH}"

RUN useradd -m -u 1000 -d /app biovigia \
    && mkdir -p /app/cargo /app/target /app/test /app/deps \
    && cargo install --locked just cargo-nextest

COPY Cargo.toml Cargo.lock /app/deps/
COPY src /app/deps/src
COPY tests /app/deps/tests

RUN cargo fetch --locked --manifest-path /app/deps/Cargo.toml \
    && rm -rf /app/deps \
    && chown -R biovigia:biovigia /app \
    && chmod -R 777 /app/cargo /app/target

WORKDIR /app/test

USER biovigia

CMD ["just", "test"]