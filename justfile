# Diple development tasks — run `just <task>` (https://github.com/casey/just)

default:
    @just --list

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all --check

lint:
    cargo clippy --all-targets --all-features -- -D warnings

test:
    cargo test --all-features

build:
    cargo build

run:
    cargo run

install:
    cargo install --path . --force

smoke-edit:
    cargo build --release
    python3 scripts/smoke_edit_file.py --binary target/release/diple

ci: fmt-check lint test
    cargo build --release
