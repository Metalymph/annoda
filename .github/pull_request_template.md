## Summary

<!-- What changes, and why? -->

## Contract impact

- [ ] No Annoda public SPI change.
- [ ] Public SPI change is required and justified by more than one real adapter.

## Quality gate

- [ ] `cargo fmt --all --check`
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] `cargo test --workspace --all-targets --all-features`
- [ ] `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`
- [ ] Adapter changes include capability and security documentation.
