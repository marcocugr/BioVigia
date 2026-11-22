# Uso la imagen acordada en docs/objetivos/objetivo-5/justificacion-imagen-base.md
FROM rust:1.99-slim-trixie

# Leo la versión de rust definida en el proyecto
COPY rust-toolchain.toml /tmp/rust-toolchain.toml

# Hago set de esa version de rust y me bajo just, nextest y assertor (pillando su version del Cargo.toml)
RUN set -eux; \
    channel="$(sed -n 's/^channel = "\(.*\)"/\1/p' /tmp/rust-toolchain.toml)"; \
    test -n "$channel"; \
    rustup toolchain install "$channel" --profile minimal; \
    cargo "+$channel" install --locked just; \
    cargo "+$channel" install --locked cargo-nextest; \
    rm /tmp/rust-toolchain.toml

# El código se monta desde fuera; los artefactos van a /tmp
ENV HOME=/tmp \
    CARGO_HOME=/tmp/cargo \
    CARGO_TARGET_DIR=/tmp/target

# Todo este directorio de BioVigia se pone como "una ventana" en /app/test como pide el guion
WORKDIR /app/test

# Sin privilegios
USER 1001

# Descargo las dependencias y ejecuto los test
CMD ["sh", "-c", "just install && just test"]
