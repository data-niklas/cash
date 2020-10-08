install:
    cp ./target/debug/cash $SCRIPTS/bin/

build:
    cargo build

release:
    cargo build --release

run:
    cargo run

test:
    ./test/test

ftest: build install test