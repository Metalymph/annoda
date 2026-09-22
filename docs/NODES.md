# Annoda interoperability nodes

An Annoda node provides explicit bidirectional interoperability between two
otherwise incompatible peer-to-peer protocols.

A complete node represents one edge in the Annoda interoperability graph:

```text
Protocol A <-> Protocol B
```

It allows a peer integrating Annoda to communicate with an unmodified native
peer using the other protocol.

## Fundamental requirements

A complete Annoda node MUST:

- support communication in both directions;
- preserve application payload bytes exactly;
- emit communication valid for the native remote protocol;
- consume communication produced by the native remote protocol;
- perform required encoding, framing, and protocol conversion locally;
- require no Annoda modification on the remote peer;
- introduce no intermediate Annoda wire protocol;
- require no central Annoda translation authority;
- declare its supported protocol versions;
- declare its supported platforms;
- declare its supported capability profile;
- declare its known limitations;
- pass native-peer conformance tests.

A node that does not satisfy these requirements MUST NOT be represented as
supported in the interoperability matrix.

## Bidirectional interoperability

A node declared as:

```text
A <-> B
```

MUST implement both:

```text
A -> B
B -> A
```

The bridge may be physically integrated on only one peer, but communication
through it is bidirectional.

For example, when an `A <-> B` node is integrated on peer A:

```text
                  Annoda-enabled peer          Native peer

A application
     |
Protocol A
     |
+-------------------+
| Annoda A <-> B    |
+-------------------+
     |
Protocol B wire  <-------------------------->  Protocol B
                                                  |
                                             B application
```

Outbound A traffic is converted into valid B communication.

Inbound B traffic is converted back into the representation expected by the A
side.

The B peer remains unmodified.

## Payload byte-stream parity

Payload byte-stream parity is mandatory.

For every payload within the node's declared supported profile:

```text
send_A(payload) -> receive_B(payload)
send_B(payload) -> receive_A(payload)
```

MUST preserve the exact application payload byte sequence.

If:

```text
payload = [b0, b1, b2, ..., bn]
```

then the receiving application MUST observe:

```text
[b0, b1, b2, ..., bn]
```

in the same order and without mutation.

This guarantee applies to arbitrary binary payloads. Annoda MUST NOT assume
that payloads contain text or any particular application serialization.

## Wire conversion

Payload parity does not require wire parity.

Protocols A and B may use completely different:

- encodings;
- codecs;
- framing;
- envelopes;
- handshakes;
- stream representations;
- multiplexing;
- addressing;
- discovery;
- connection lifecycle;
- transport security.

The node MUST perform whatever conversion is necessary for its declared
interoperability profile.

Conceptually:

```text
A payload
   |
A representation
   |
   v
+--------------------------+
| Annoda A <-> B node      |
|                          |
| decode A as necessary    |
| preserve payload         |
| encode B as necessary    |
+--------------------------+
   |
B representation
   |
B payload
```

The reverse path performs the corresponding B-to-A conversion.

## No Annoda wire format

A node MUST NOT require a native remote peer to understand an Annoda-specific
wire representation.

An internal representation MAY be used inside the node to make conversion
manageable, but it MUST remain an implementation detail.

The network-facing output of the node MUST be valid communication for the
native protocol being targeted.

Annoda therefore does not define a universal wire format shared by all nodes.

## Native-peer transparency

The native remote peer MUST NOT need:

- Annoda;
- an Annoda library;
- an Annoda runtime;
- an Annoda codec;
- an Annoda handshake;
- an Annoda framing layer;
- an Annoda gateway;
- an Annoda proxy;
- an Annoda translation service.

For every capability claimed by the node, the remote peer must be able to
behave as a normal native participant in its own protocol.

## Native fast path

A node exists only to bridge different protocols.

Same-protocol communication:

```text
A <-> A
```

does not require an Annoda node.

If both peers already use compatible implementations of A, they communicate
natively.

Where the protocol pair is statically known, implementations SHOULD allow the
Annoda path to be eliminated at compile time.

Where selection is dynamic, same-protocol communication MUST bypass node
translation.

## Node isolation

Every node MUST be independently consumable.

An application requiring:

```text
A <-> B
```

MUST NOT be required to include:

```text
A <-> C
B <-> C
D <-> E
```

or dependencies used exclusively by those unrelated nodes.

This requirement applies regardless of the eventual packaging mechanism.

A Rust implementation may eventually use one crate per protocol pair, while
other ecosystems may use their native package or library mechanisms.

The concrete packaging design is not part of the interoperability contract.

## Protocol versions

Protocol names alone are insufficient to establish compatibility.

Every node MUST explicitly document the versions or compatibility ranges
against which it has been implemented and verified.

For example, support for:

```text
A <-> B
```

does not automatically imply support for every historical or future version of
A and B.

A protocol upgrade that changes relevant wire behaviour may require new
conformance evidence before the node can claim compatibility with that version.

## Capability profile

A node guarantees only its explicitly declared capability profile.

Baseline Annoda interoperability is messaging-oriented and requires opaque
application payload exchange with byte-stream parity.

A node MAY additionally declare capabilities such as:

- large payloads;
- chunked transfer;
- blob transfer;
- file transfer;
- recorded audio payload transfer.

Optional capabilities MUST have their own conformance coverage.

A node MUST NOT imply complete feature equivalence between its two protocols.

Unsupported native features remain unsupported unless explicitly bridged.

## Protocol-specific infrastructure

A node MAY use infrastructure belonging to either native protocol when that
infrastructure is part of normal protocol operation.

Examples include:

- DHT infrastructure;
- bootstrap nodes;
- relays;
- rendezvous services;
- discovery systems.

Such infrastructure does not become Annoda infrastructure merely because a
node uses it.

Every node MUST document:

- which infrastructure it requires;
- which protocol owns it;
- whether it is optional or mandatory;
- relevant trust assumptions;
- relevant metadata exposure.

## Required node documentation

Every node MUST document at least:

- protocol A;
- protocol B;
- supported versions or compatibility ranges;
- supported operating systems;
- supported architectures where relevant;
- supported runtimes where relevant;
- node integration requirements;
- baseline messaging support;
- additional capabilities;
- unsupported capabilities;
- payload limits;
- framing and encoding differences;
- translation strategy;
- identity assumptions;
- addressing assumptions;
- discovery requirements;
- NAT traversal behaviour;
- relay requirements;
- bootstrap requirements;
- connection lifecycle behaviour;
- reconnect behaviour;
- security properties;
- metadata exposure;
- dependencies;
- known limitations.

The documentation MUST distinguish verified guarantees from implementation
plans or expected behaviour.

## Conformance requirements

A complete node MUST be validated against native implementations of both
protocols.

At minimum, conformance MUST cover:

- connection establishment with a native A peer where applicable;
- connection establishment with a native B peer where applicable;
- A -> B payload exchange;
- B -> A payload exchange;
- simultaneous bidirectional exchange;
- exact payload byte-stream parity;
- arbitrary binary payloads;
- empty payload behaviour where supported;
- minimum and boundary-sized payloads;
- large payload limits;
- repeated payloads;
- ordering guarantees claimed by the node;
- disconnect behaviour;
- reconnect behaviour where supported;
- malformed input;
- truncated or invalid protocol data;
- native protocol rejection;
- remote failure;
- local failure;
- security-relevant failure paths;
- native-peer transparency.

Conformance MUST exercise real native implementations.

A test in which both endpoints are Annoda mocks or both sides use the same
translation implementation is insufficient to establish supported matrix
compatibility.

## Deterministic matrix admission

The canonical interoperability matrix has a deliberately strong meaning.

A protocol pair may be marked `supported` only when its complete node has
passed the required conformance gate for the versions and capability profile
being claimed.

Development states may include:

```text
planned
experimental
supported
```

`planned` means no interoperability guarantee exists.

`experimental` means implementation work exists but the complete Annoda
guarantee has not yet been established.

`supported` means the declared bidirectional interoperability guarantee has
been verified.

There is no implicit compatibility between protocol pairs.

## Security requirements

Every node MUST document security properties independently of functional
interoperability.

At minimum, review must consider:

- peer identity;
- peer authentication;
- transport confidentiality;
- transport integrity;
- key creation;
- key persistence;
- protocol downgrade risks;
- relay trust;
- bootstrap trust;
- discovery trust;
- metadata exposure;
- malformed-input handling;
- parser and codec attack surface;
- dependency and runtime risks.

A node MUST NOT silently disable or weaken a security mechanism merely to make
the protocols communicate.

If a security property cannot be preserved across the declared capability
profile, that limitation MUST be explicit and MUST NOT be represented as a
stronger guarantee.

## Contribution model

New interoperability relationships should be contributed as new nodes rather
than by widening a universal Annoda transport abstraction.

A node proposal should therefore answer a concrete question:

```text
How can native Protocol A and native Protocol B communicate bidirectionally
while modifying only the peer that integrates Annoda?
```

Shared abstractions should be introduced only after multiple real nodes prove
that the abstraction is genuinely common.

Annoda grows by adding verified edges to its interoperability graph, not by
pretending that all P2P protocols share one universal transport model.