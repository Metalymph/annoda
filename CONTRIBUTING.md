# Contributing

Contributions are welcome, but Annoda intentionally keeps a narrow public
surface.

## Before opening code

For a new adapter, open an **Adapter proposal** issue first. Describe:

- underlying stack and upstream project;
- supported platforms;
- identity model;
- discovery/bootstrap model;
- NAT traversal and relay behaviour;
- transport security;
- metadata exposure;
- runtime requirements;
- dependency footprint;
- which Annoda capabilities can be implemented honestly.

## Pull request gate

Before submitting:

```console
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
cargo package -p annoda
```

Public APIs and data must be documented.

Adapter changes must include tests and security notes. Avoid widening the core
SPI merely to accommodate one implementation.
