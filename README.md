# Idalion

**Interoperability for conversational messaging protocols.**

Idalion allows applications and devices using different messaging protocols to communicate when their respective capabilities admit a meaningful mapping.

It does not make protocols identical. It makes explicitly supported conversational semantics interoperable.

```text
Application / device
        |
        v
     Idalion
        |
        +-- protocol adapter A --> native protocol A
        |
        +-- protocol adapter B --> native protocol B
        |
        +-- protocol adapter C --> native protocol C
```

Participating protocols may be peer-to-peer, server-mediated, or use another architecture suitable for conversational messaging.

## Principles

- **Semantic interoperability.** Idalion preserves supported conversational meaning, not identical wire bytes.
- **Capability honesty.** Unsupported semantics remain unsupported instead of being silently approximated.
- **Directional support.** A capability working from protocol A to B does not imply that B to A is equivalent.
- **Infrastructure honesty.** P2P and centralized systems remain architecturally different.
- **No Idalion authority.** Idalion does not require global accounts, identities, discovery, message storage, relays, or a central translation service.
- **Native protocols stay native.** Remote endpoints do not need an Idalion wire format, codec, or handshake.
- **Small core.** Protocol-specific dependencies and behaviour remain inside adapters.

## Scope

Idalion is specifically for **conversational messaging interoperability**.

The contract may cover semantics such as text, replies, reactions, edits, deletion, attachments, recorded audio, delivery/read state, groups, addressing, and discovery as real adapters demonstrate shared requirements.

Idalion is not a generic integration framework, event bus, RPC layer, webhook bridge, network tunnel, ETL system, or universal protocol converter.

## Current status

Idalion is currently **0.0.x** and its public SPI is experimental.

The initial foundation establishes:

- opaque protocol-owned endpoints;
- semantic text operations;
- inbound and outbound capability declarations;
- explicit adapter acceptance distinct from delivery state;
- runtime-agnostic adapter event polling;
- an in-memory semantic reference adapter.

The contract will grow from verified native adapters rather than from a speculative universal messaging model.

## Workspace

```text
crates/
  idalion/                    core interoperability contract
  idalion-testkit/            reusable adapter conformance helpers

adapters/
  idalion-adapter-loopback/   in-memory semantic reference adapter
```

Protocol adapters may live in this repository or externally.

## Architecture and contract

The normative design is documented in:

- `docs/CONTRACT.md` — interoperability guarantees and boundaries;
- `docs/ARCHITECTURE.md` — architectural model;
- `docs/ADAPTERS.md` — adapter requirements and conformance expectations.

The central rule is:

> Idalion preserves supported messaging semantics across protocol boundaries without pretending that the protocols themselves are equivalent.

## Development

Requires Rust 1.98 or newer.

```console
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

See `CONTRIBUTING.md` for contribution requirements and `SECURITY.md` for security reporting.

## License

MIT.
