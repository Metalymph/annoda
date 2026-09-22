## Summary

<!-- Describe the change and why it is needed. -->

## Annoda contract

- [ ] This change preserves the invariants in `docs/CONTRACT.md`.
- [ ] No intermediate Annoda wire protocol is introduced.
- [ ] Native same-protocol communication remains outside the Annoda interoperability path.
- [ ] Shared architecture changes are required and justified by more than one real protocol pair, or directly required by the normative contract.

## Node changes

<!-- Check these when this PR adds or changes an interoperability node. -->

- [ ] The protocol pair and supported versions are documented.
- [ ] Both interoperability directions are implemented or the node remains explicitly experimental.
- [ ] Application payload byte-stream parity is covered by tests.
- [ ] Native-peer conformance evidence is included where required.
- [ ] Capability, limitation, dependency, and security documentation is updated.
- [ ] `docs/MATRIX.md` is updated when interoperability status changes.

## Validation

- [ ] `cargo fmt --all --check`
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] `cargo test --workspace --all-targets --all-features`
- [ ] `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`
- [ ] Relevant node-specific/native interoperability tests pass.

## Security

<!-- Describe security implications, or explain why there are none. -->

## Notes

<!-- Optional reviewer context, limitations, follow-up work, or intentionally deferred changes. -->