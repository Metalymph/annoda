# Architecture

## Purpose

Annoda provides explicit interoperability between otherwise incompatible
peer-to-peer protocols through protocol-pair-specific nodes.

Annoda does not replace native P2P stacks and does not introduce a common
network protocol.

Instead, a node adds the protocol behaviour required to one peer so that it can
communicate with an unmodified native peer using another protocol.

## Core architecture

For a protocol pair `A <-> B`, with Annoda integrated on the A peer:

```text
Peer A                                      Peer B
Application                                Application
    |                                          |
Protocol A                                Protocol B
    |                                          |
Annoda A <-> B node                           |
    |                                          |
    +----------- Protocol B wire --------------+
```

The B peer remains native and unmodified.

The node is responsible for making communication emitted toward B valid
according to Protocol B and for converting communication received from B back
into the representation required by the A side.

The same interoperability relationship may be integrated on the opposite peer
when the node implementation supports that deployment.

## Bidirectional bridge

A complete node is bidirectional.

For a declared:

```text
A <-> B
```

the node must provide both:

```text
A -> B
B -> A
```

This is a single interoperability relationship, not two independent
compatibility claims.

A partial one-way implementation may be useful while developing or validating
a node, but it is not a complete Annoda node and cannot establish supported
matrix compatibility.

## Payload byte-stream invariant

Application payload bytes are invariant across the bridge.

For supported payloads:

```text
payload_A == payload_B
```

in both directions.

Wire representation is not invariant.

Protocols may differ in:

- framing;
- encoding;
- codecs;
- envelopes;
- handshakes;
- stream representation;
- connection semantics;
- transport security;
- other protocol-specific wire behaviour.

The node performs whatever conversion is required while preserving the
application payload byte sequence exactly.

Conceptually:

```text
A application payload
        |
        v
A protocol representation
        |
        v
+-----------------------+
|   Annoda A <-> B      |
|                       |
| decode / normalize    |
| translate as required |
| encode for B          |
+-----------------------+
        |
        v
B protocol representation
        |
        v
B application payload
```

The reverse path performs the corresponding B-to-A conversion.

## No intermediate Annoda protocol

Annoda has no universal network protocol.

The architecture is explicitly not:

```text
Protocol A
    |
Annoda wire protocol
    |
Protocol B
```

Such a design would introduce a third protocol and require both peers to
understand it, defeating Annoda's purpose.

Instead, when the node is hosted by the A peer:

```text
Protocol A + Annoda A <-> B node
                |
                v
        native Protocol B wire
                |
                v
        native Protocol B peer
```

Any canonical or intermediate representation used internally by a node is an
implementation detail.

It must remain local to the node and must never become a network protocol
required by the remote peer.

## Native-peer transparency

The peer that does not integrate Annoda must remain native.

For an A-hosted `A <-> B` node, the B peer must not require:

- Annoda;
- an Annoda runtime;
- an Annoda library;
- an Annoda codec;
- an Annoda handshake;
- an Annoda wire format;
- an Annoda gateway;
- an Annoda translation service.

From the B peer's perspective, communication must be valid Protocol B
communication for every capability claimed by the node.

The corresponding rule applies when Annoda is hosted on the B side.

## Explicit pairwise interoperability

Annoda models interoperability as explicit relationships between protocol
pairs.

For example:

```text
Hyperswarm <-> Iroh
Hyperswarm <-> libp2p
Iroh <-> libp2p
```

Each relationship requires its own implementation and verification.

Compatibility is not transitive.

If these nodes exist:

```text
A <-> B
B <-> C
```

Annoda does not infer:

```text
A <-> C
```

An explicit `A <-> C` node is required.

This produces a deterministic interoperability graph whose supported edges are
recorded in the canonical matrix.

## Native fast path

No interoperability node is required when both peers already use the same
compatible protocol.

```text
A <-> A
```

uses A directly.

Annoda must not place translation, framing, encoding, dispatch, or other
interoperability work in a same-protocol path merely for architectural
uniformity.

When protocol selection is statically known, implementations should allow the
Annoda interoperability path to be eliminated at compile time.

Conceptually:

```text
compile-time protocols

A + A
  |
  +--> native A path

A + B
  |
  +--> Annoda A <-> B node
```

When protocol selection is dynamic, the same rule applies at runtime:
same-protocol communication bypasses node translation completely.

The interoperability matrix therefore represents same-protocol relationships
as `native`, not as Annoda nodes.

## Node isolation

Nodes are modular and independently consumable.

Conceptually:

```text
nodes/
    hyperswarm-iroh/
    hyperswarm-libp2p/
    iroh-libp2p/
```

An application requiring only one protocol pair must not be required to include
implementations or dependencies belonging exclusively to unrelated nodes.

The exact crate, package, FFI, native-library, or platform packaging model is
an implementation decision that will be validated by real nodes.

Isolation itself is an architectural requirement.

## Node implementation strategy

Annoda does not prescribe one universal implementation technique.

Depending on the protocols involved, a node may:

- reuse an existing native implementation;
- embed part of another protocol implementation;
- implement the required target protocol behaviour directly;
- translate framing or encoding;
- bridge stream abstractions;
- integrate protocol-native discovery;
- participate in protocol-native handshakes;
- use protocol-native relays;
- combine multiple techniques.

The implementation is correct only if the resulting behaviour satisfies the
Annoda contract and the native remote peer remains unmodified.

## Responsibility boundaries

### Annoda node owns

A node owns the interoperability required for its declared protocol pair and
capability profile.

This includes, where necessary:

- bidirectional protocol bridging;
- application payload byte-stream parity;
- encoding conversion;
- framing conversion;
- stream representation conversion;
- handshake compatibility;
- addressing adaptation;
- discovery integration;
- authentication integration;
- transport-security integration;
- connection lifecycle mapping;
- relay integration;
- reconnect behaviour;
- error mapping;
- conformance evidence.

Not every node will require every item.

The protocol pair determines the necessary work.

### Native protocols own

The underlying protocols retain their own semantics and infrastructure.

Depending on the protocol, this may include:

- peer identity;
- addressing;
- discovery;
- DHT behaviour;
- NAT traversal;
- relays;
- rendezvous;
- bootstrap infrastructure;
- transport encryption;
- connection establishment;
- stream multiplexing;
- native framing.

Annoda does not redefine these mechanisms globally.

A node interacts with or implements the required subset for its specific
protocol pair.

### Application owns

Application-level concerns remain outside the baseline Annoda contract,
including:

- user and account identity;
- contacts;
- conversations;
- application message schemas;
- message identifiers;
- persistence and history;
- delivery and read receipts;
- moderation;
- content policy;
- application-level retries;
- application-level E2EE;
- key management above the transport;
- UI;
- application lifecycle policy.

Annoda transports opaque application payload bytes and does not assign them
application semantics.

## No central Annoda authority

Interoperability happens on the peer hosting the node.

Annoda does not require:

- a central translation gateway;
- a proxy;
- an Annoda relay network;
- an Annoda rendezvous authority;
- an Annoda discovery service.

This does not prohibit infrastructure that belongs to one of the native
protocols.

For example, a node may legitimately use a target protocol's:

- relays;
- DHT;
- bootstrap nodes;
- rendezvous infrastructure;
- discovery services.

Such requirements and their trust implications must be documented by the node.

## Capability model

Annoda guarantees declared interoperability, not complete feature equivalence
between protocols.

Baseline interoperability is messaging-oriented and requires opaque payload
byte preservation.

A node may additionally declare support for capabilities such as:

- large payloads;
- chunked transfer;
- blob transfer;
- file transfer;
- recorded audio payload transfer.

Capabilities beyond the baseline must be explicit and independently tested.

Real-time audio/video streaming and generic network tunnelling are outside the
baseline architecture.

## Conformance model

A protocol pair becomes supported only through evidence against native
implementations.

A complete node must demonstrate:

```text
native A -> Annoda node -> native B
native B -> Annoda node -> native A
```

with exact application payload byte-stream parity.

Testing only internal Annoda mocks or two sides of the same bridge
implementation is insufficient to establish interoperability.

The conformance model must also validate the lifecycle, limits, capabilities,
and security properties claimed by the node.

## Security boundary

Protocol interoperability does not imply application-level end-to-end
encryption.

Transport encryption, peer authentication, message authenticity, application
E2EE, and metadata privacy are distinct properties.

A node must not silently weaken protocol security merely to obtain
interoperability.

Each node documents its concrete security assumptions and any unavoidable
differences between the two protocols.

## Evolution

The 0.0.x series exists to validate this architecture against real,
architecturally different P2P protocols.

The first real protocol-pair node is an architectural proof.

Shared abstractions should be extracted only after multiple real nodes
demonstrate that they are genuinely common.

Annoda must remain a graph of explicit interoperability relationships rather
than evolving into a universal transport abstraction or a new P2P protocol.