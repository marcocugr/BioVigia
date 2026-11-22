install:
    cargo fetch --locked

test: install
    cargo nextest run

check:
    cargo check