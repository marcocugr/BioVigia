FROM rust:slim-trixie

ENV HOME=/app \
    CARGO_HOME=/app/cargo \
    CARGO_TARGET_DIR=/app/target \
    PATH="/app/cargo/bin:${PATH}"

RUN useradd -m -d /app biovigia \
    && mkdir -p /app/cargo /app/target /app/test /app/deps \
    && chown -R biovigia:biovigia /app

USER biovigia

RUN cargo install --locked just cargo-nextest

COPY --chown=biovigia:biovigia Cargo.toml Cargo.lock /app/deps/
COPY --chown=biovigia:biovigia src /app/deps/src
COPY --chown=biovigia:biovigia tests /app/deps/tests

RUN cargo fetch --locked --manifest-path /app/deps/Cargo.toml \
    && rm -rf /app/deps \
    && chmod -R a+rwX /app/cargo /app/target

WORKDIR /app/test

CMD ["just", "test"]