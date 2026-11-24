install:
    cargo fetch --locked

test:
    cargo nextest run

check:
    cargo check