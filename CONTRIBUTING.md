# Contributing

Contributions are welcome, but Idalion intentionally keeps a narrow public
surface.

Active implementation is currently paused. Contributions that change the
interoperability model should be motivated by a concrete P2P-to-P2P messaging
use case rather than speculative generalization.

## Before opening code

For a new P2P adapter, open an **Adapter proposal** issue first. Describe:

- underlying protocol or stack and upstream project;
- supported platforms;
- peer identity model;
- addressing model;
- discovery and bootstrap model;
- connection or session lifecycle where applicable;
- NAT traversal and relay behaviour where applicable;
- transport and application-level security properties;
- metadata exposure;
- runtime requirements;
- dependency footprint;
- which Idalion capabilities can be implemented honestly.

Centralized, service-controlled, and federated messaging integrations are
outside the current Idalion scope.

## Pull request gate

Before submitting:

```console
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
cargo package -p idalion --locked
```

Public APIs and data must be documented.

Adapter changes must include tests and security notes. Avoid widening the core
SPI merely to accommodate one implementation.