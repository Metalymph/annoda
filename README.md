# Idalion

**Semantic interoperability for peer-to-peer conversational messaging protocols.**

Idalion allows applications and devices using different P2P messaging protocols
to communicate when their respective capabilities admit a meaningful mapping.

It does not make protocols identical. It makes explicitly supported
conversational semantics interoperable.

    Application / device
            |
            v
         Idalion
            |
            +-- P2P adapter A --> native P2P protocol A
            |
            +-- P2P adapter B --> native P2P protocol B
            |
            +-- P2P adapter C --> native P2P protocol C

Idalion is deliberately limited to peer-to-peer conversational protocols.
Centralized, service-controlled, and federated messaging platforms are outside
the current interoperability contract.

## Principles

- **Semantic interoperability.** Idalion preserves supported conversational
  meaning, not identical wire bytes.
- **Capability honesty.** Unsupported semantics remain unsupported instead of
  being silently approximated.
- **Directional support.** A capability working from protocol A to B does not
  imply that B to A is equivalent.
- **Infrastructure honesty.** Different P2P systems retain their native
  addressing, discovery, connectivity, relay, persistence, and security models.
- **No Idalion authority.** Idalion does not require global accounts,
  identities, discovery, message storage, relays, or a central translation
  service.
- **Native protocols stay native.** Remote endpoints do not need an Idalion
  wire format, codec, or handshake.
- **Small core.** Protocol-specific dependencies and behaviour remain inside
  adapters.

## Scope

Idalion is specifically for **P2P conversational messaging interoperability**.

The contract may cover semantics such as text, replies, reactions, edits,
deletion, attachments, recorded audio, delivery/read state, groups, addressing,
and discovery as real P2P adapters demonstrate shared requirements.

Potential protocol families include Hyperswarm/Holepunch, Iroh, libp2p, and
other P2P conversational stacks. These are examples, not a fixed compatibility
list. A protocol is supported only when a real adapter implements and verifies
the corresponding capability profile.

Centralized, service-controlled, and federated messaging platforms are not part
of the current contract. The investigation and rationale behind that decision
are recorded in `docs/CENTRALIZED-PROTOCOL-RESEARCH.md`.

Idalion is not a generic integration framework, event bus, RPC layer, webhook
bridge, network tunnel, ETL system, centralized messaging gateway, or universal
protocol converter.

## Current status

Idalion is currently **0.0.x** and its public SPI is experimental.

The initial foundation establishes:

- opaque protocol-owned P2P endpoints;
- semantic text operations;
- inbound and outbound capability declarations;
- explicit adapter acceptance distinct from delivery state;
- runtime-agnostic adapter event polling;
- an in-memory semantic reference adapter.

The contract will grow only from verified native P2P adapters rather than from
a speculative universal messaging model.

**Active development is currently paused.**

The repository remains public and the existing contract is retained for future
P2P interoperability work. Development should resume when a concrete
P2P-to-P2P integration provides real requirements against which the contract
can evolve.

The project is paused, not deprecated or abandoned.

## Workspace

    crates/
      idalion/                    core interoperability contract
      idalion-testkit/            reusable adapter conformance helpers

    adapters/
      idalion-adapter-loopback/   in-memory semantic reference adapter

Protocol adapters may live in this repository or externally.

## Architecture and contract

The design is documented in:

- `docs/CONTRACT.md` — normative interoperability guarantees and boundaries;
- `docs/ARCHITECTURE.md` — architectural model;
- `docs/ADAPTERS.md` — adapter requirements and conformance expectations;
- `docs/CENTRALIZED-PROTOCOL-RESEARCH.md` — research and rationale for excluding
  centralized messaging platforms from the current scope.

The central rule is:

> Idalion preserves supported conversational semantics across P2P protocol
> boundaries without pretending that the protocols themselves are equivalent.

## Development

Requires Rust 1.98 or newer.

    cargo test --workspace
    cargo clippy --workspace --all-targets -- -D warnings

See `CONTRIBUTING.md` for contribution requirements and `SECURITY.md` for
security reporting.

## License

MIT.