fmt:
    cargo +nightly fmt

clippy:
    cargo clippy --fix --allow-dirty

release:
    cargo build --release --bin kt2j2t
    mv target/release/kt2j2t .