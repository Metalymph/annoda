# Writing an Annoda adapter

An adapter translates Annoda's small SPI to one concrete P2P stack.

## Required implementation surface

An adapter must provide:

- a stable adapter name;
- a local opaque peer identity;
- explicit capability reporting;
- connect;
- send;
- non-blocking event polling;
- disconnect.

## Required design constraints

1. Do not expose stack-specific types through Annoda's public API.
2. Keep peer identifiers opaque to Annoda.
3. Do not claim capabilities the underlying stack cannot guarantee.
4. Do not silently introduce a central gateway to emulate interoperability.
5. Document any relay, rendezvous, bootstrap, or authority requirements.
6. Document metadata visible to relays or bootstrap infrastructure.
7. Document transport encryption separately from application-level E2EE.
8. Keep adapter-specific dependencies inside the adapter crate.

## Conformance expectations

Every adapter PR must include:

- deterministic unit tests where possible;
- connection lifecycle tests;
- bidirectional frame tests;
- disconnect/reconnect tests when supported;
- malformed-input/error-path tests;
- capability assertions;
- security notes;
- dependency rationale;
- platform support matrix.

As Annoda matures, `annoda-testkit` will become the canonical reusable
conformance suite.

## Proposed adapter layout

```text
adapters/
  annoda-adapter-<stack>/
    Cargo.toml
    README.md
    src/
    tests/
```

External adapters may live in separate repositories. In-tree inclusion is not
a requirement for ecosystem compatibility.
