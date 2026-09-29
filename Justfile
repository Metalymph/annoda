set dotenv-load := false

default:
    @just --list

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all --check

check:
    cargo check --workspace --all-targets --all-features

clippy:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
    cargo test --workspace --all-targets --all-features

docs:
    RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps

package:
    cargo package -p idalion

ci: fmt-check check clippy test docs package
