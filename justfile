run:
    cargo watch -q -c -w src/ -x run | bunyan

test:
    cargo watch -x test | bunyan

build:
    cargo build --release
