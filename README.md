# Annoda

Annoda provides explicit interoperability between otherwise incompatible
peer-to-peer protocols.

A peer integrates a protocol-pair-specific Annoda node and gains the ability
to communicate with an unmodified native peer using another supported P2P
protocol.

```text
Peer A                                  Peer B
Protocol A                              Protocol B
    +                                      |
Annoda A <-> B node                        |
    |                                      |
    +---------- Protocol B wire -----------+
```

The remote peer does not need Annoda.

## Core guarantees

A complete Annoda node guarantees:

- bidirectional `A <-> B` interoperability;
- application payload byte-stream parity;
- protocol-specific encoding, framing, and wire conversion when necessary;
- native protocol behaviour toward the unmodified remote peer;
- no intermediate Annoda wire protocol;
- no central Annoda gateway or translation authority.

The node performs the interoperability work on the peer that integrates
Annoda. The native remote peer receives and emits only communication valid for
its own protocol.

## Native fast path

When both peers already use the same compatible protocol, Annoda is not part
of the communication path.

```text
Protocol A <-> Protocol A
```

uses Protocol A natively.

Where protocol selection is known at compile time, implementations should
allow the Annoda interoperability path to disappear entirely.

Where protocol selection is dynamic, same-protocol communication bypasses node
translation completely.

Annoda therefore adds no interoperability layer to native-to-native
same-protocol communication.

## Explicit nodes

Annoda is not a universal protocol translator.

Every supported protocol relationship is implemented and verified explicitly.

Examples:

```text
Hyperswarm <-> Iroh
Hyperswarm <-> libp2p
Iroh <-> libp2p
```

Each relationship is an independent Annoda node.

Support for one protocol pair does not imply support for another, and
compatibility is not transitive.

The canonical support matrix lives in `docs/MATRIX.md`.

## Payload parity

Annoda treats application payloads as opaque bytes.

For every payload supported by a node:

```text
bytes sent by A == bytes delivered to B
bytes sent by B == bytes delivered to A
```

The wire representation may differ completely between protocols.

When required, the node converts protocol-specific encoding, framing, codecs,
handshakes, stream representations, and other wire behaviour while preserving
the application payload byte sequence exactly.

There is no universal Annoda encoding or Annoda wire protocol between peers.

## Modular by design

Nodes are independently consumable.

An application that requires only:

```text
Hyperswarm <-> Iroh
```

must not be required to include unrelated protocol nodes or their dependencies.

The concrete packaging model may differ by language and platform, but this
isolation is an architectural requirement.

## Scope

Annoda's baseline scope is peer-to-peer messaging interoperability.

The fundamental application data unit is an opaque byte payload. Text, Unicode,
structured messages, encrypted application data, and arbitrary binary payloads
remain application concerns.

Nodes may explicitly provide additional capabilities such as chunked payloads,
blob or file transfer, and recorded audio transfer.

Real-time audio/video streaming, generic network tunnelling, and universal
feature equivalence between P2P protocols are outside the baseline contract.

## No Annoda authority

Annoda does not require a central gateway, proxy, relay network, rendezvous
authority, or translation service.

Protocol-native infrastructure such as relays, DHTs, bootstrap nodes,
rendezvous services, or discovery systems may still be required by the
protocols participating in a node.

Those requirements and their security assumptions must be documented by each
node.

## Status

**0.0.x — experimental interoperability foundation.**

The interoperability contract is being established before the first real
protocol-pair node is declared supported.

The existing Rust foundation predates the finalized interoperability model and
remains experimental while the node architecture and conformance model are
validated.

## Documentation

- `docs/CONTRACT.md` — normative interoperability contract
- `docs/ARCHITECTURE.md` — architecture and responsibility boundaries
- `docs/NODES.md` — requirements for interoperability nodes
- `docs/MATRIX.md` — canonical protocol interoperability matrix
- `SECURITY.md` — security policy

## License

MIT