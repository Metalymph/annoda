# Annoda interoperability contract

## Purpose

Annoda enables two peers using otherwise incompatible peer-to-peer protocols
to communicate through an explicit interoperability node.

Annoda does not require both peers to use Annoda.

For a protocol pair `A <-> B`, only one peer needs to integrate the Annoda
node. The other peer remains a native, unmodified participant in its own
protocol.

Example:

```text
Peer A                                  Peer B
Hyperswarm                              Iroh
    +                                     |
Annoda Hyperswarm <-> Iroh node           |
    |                                     |
    +----------- Iroh wire ---------------+
```

The Iroh peer does not need to know that Annoda exists.

## Fundamental guarantee

A published Annoda node MUST provide bidirectional interoperability between
the two protocols it declares.

A node declared as:

```text
A <-> B
```

MUST support both:

```text
A -> B
B -> A
```

A one-way implementation MAY exist during development, experimentation, or
validation, but MUST NOT be published in the interoperability matrix as a
complete Annoda node.

Without verified bidirectional interoperability, there is no Annoda
compatibility guarantee.

## Payload byte-stream parity

Payload byte-stream parity is a fundamental Annoda invariant.

For every payload supported by the node:

```text
payload bytes sent by A == payload bytes delivered to B
payload bytes sent by B == payload bytes delivered to A
```

This equality refers to the application payload byte sequence.

Protocol-specific wire bytes are not required to be identical.

If protocols A and B use different framing, encoding, codecs, envelopes,
handshakes, stream representations, or other wire-level representations, the
Annoda node MUST perform the transformations necessary to preserve the
application payload exactly.

Conceptually:

```text
A -> B

A application bytes
        |
        v
A protocol representation
        |
        v
Annoda node
  decode / normalize
  encode for B
        |
        v
B protocol representation
        |
        v
B application bytes

A application bytes == B application bytes
```

And symmetrically:

```text
B -> A

B application bytes
        |
        v
B protocol representation
        |
        v
Annoda node
  decode / normalize
  encode for A
        |
        v
A protocol representation
        |
        v
A application bytes

B application bytes == A application bytes
```

The node performs all required conversion on the peer that integrates Annoda.

Annoda MUST NOT expose an Annoda-specific encoding to the native remote peer.

Each native peer MUST observe only communication valid for its own protocol.

## No intermediate wire protocol

Annoda MUST NOT introduce a universal intermediate network protocol.

There is no Annoda wire protocol between peers.

For an `A <-> B` node, communication visible to an unmodified native B peer
MUST be valid B communication.

Communication visible to an unmodified native A peer MUST be valid A
communication.

A node MAY use an internal intermediate representation while translating
between protocols, but that representation:

- MUST remain internal to the node;
- MUST NOT become a third network protocol;
- MUST NOT require the remote peer to understand Annoda.

## Node placement

Only the peer integrating Annoda is modified.

If peer A integrates an `A <-> B` node in order to communicate with native
peer B, all interoperability work happens on peer A.

The native B peer MUST NOT require:

- Annoda;
- an Annoda runtime;
- an Annoda library;
- an Annoda gateway;
- an Annoda-specific codec;
- an Annoda-specific wire protocol.

The same node MUST provide the corresponding reverse communication path so
that data received from native B is converted back into communication valid
for A.

Which peer hosts the node is an application and deployment decision, subject
to the platforms supported by the node implementation.

## Native fast path

When both peers already use the same compatible protocol, Annoda has no
interoperability work to perform.

```text
A <-> A
```

MUST use the native A path.

An Annoda node MUST NOT be inserted into a same-protocol communication path
merely for architectural uniformity.

Where protocol selection is statically known, an implementation SHOULD allow
the Annoda interoperability layer to be eliminated at compile time.

Where protocol selection is dynamic, the native path MUST bypass node
translation completely.

The interoperability matrix represents same-protocol communication as
`native`, not as an Annoda node.

## Explicit protocol pairs

Annoda does not claim universal interoperability.

Every supported relationship is explicit.

Examples:

```text
Hyperswarm <-> Iroh
Hyperswarm <-> libp2p
Iroh <-> libp2p
```

Each relationship requires its own implemented and verified node.

Support for:

```text
A <-> B
```

and:

```text
B <-> C
```

MUST NOT be interpreted as support for:

```text
A <-> C
```

unless an explicit `A <-> C` node exists.

Annoda compatibility is therefore represented by a deterministic
interoperability matrix.

## Independent consumption

Every Annoda node MUST be independently consumable.

A peer requiring only:

```text
A <-> B
```

MUST NOT be required to include implementations or dependencies for unrelated
protocol pairs.

The concrete packaging mechanism is language and platform specific, but node
isolation is an architectural requirement.

## Messaging scope

The baseline Annoda contract targets peer-to-peer messaging.

The fundamental application data unit is an opaque byte payload.

Annoda assigns no application semantics to those bytes.

A payload MAY contain, for example:

- UTF-8 text;
- Unicode emoji;
- JSON;
- protobuf;
- MessagePack;
- encrypted application data;
- arbitrary binary application data.

Annoda MUST preserve the payload byte sequence regardless of its application
meaning.

## Optional capabilities

Protocols may expose capabilities beyond baseline messaging.

A node MAY additionally support capabilities such as:

- large payloads;
- chunked transfer;
- blob transfer;
- file transfer;
- recorded audio payload transfer.

Such capabilities MUST be declared explicitly and tested.

A node MUST NOT claim equivalence between the complete feature sets of two
protocols.

Annoda guarantees interoperability only for the capability profile declared
by the node.

Real-time audio/video media streaming and generic network tunnelling are not
part of the baseline Annoda contract.

## No Annoda authority

Annoda MUST NOT require a central Annoda service to translate communication.

Annoda MUST NOT require:

- a central gateway;
- a translation proxy;
- an Annoda relay network;
- an Annoda rendezvous authority.

This restriction does not prohibit infrastructure belonging to either native
protocol, such as:

- protocol-native relays;
- DHT infrastructure;
- bootstrap nodes;
- rendezvous services;
- discovery infrastructure.

A node MUST document any such dependency and its trust assumptions.

## Protocol responsibility

An Annoda node is responsible for implementing, reusing, wrapping, translating,
or otherwise satisfying whatever parts of the two protocols are necessary for
its declared interoperability guarantee.

This MAY include:

- discovery;
- addressing;
- handshake behaviour;
- authentication;
- encryption;
- framing;
- stream establishment;
- message encoding;
- connection lifecycle;
- relay behaviour;
- reconnect behaviour.

The exact mechanism is node-specific.

Annoda MUST NOT pretend that incompatible protocols become interoperable
without implementing the protocol behaviour necessary to make them so.

## Capability honesty

A node MUST declare:

- its two protocols;
- supported protocol versions or compatibility ranges;
- supported platforms;
- baseline messaging support;
- additional supported capabilities;
- known limitations;
- security assumptions;
- infrastructure requirements.

A node MUST NOT advertise compatibility that has not been implemented and
verified against native peers.

## Conformance

A complete node MUST be tested against native implementations of both
protocols.

At minimum, conformance MUST verify:

- A -> B connection establishment;
- A -> B payload exchange;
- B -> A payload exchange;
- simultaneous bidirectional payload exchange;
- payload byte-stream parity in both directions;
- empty and boundary-sized payload behaviour where supported;
- arbitrary binary payloads;
- repeated messages;
- ordering behaviour declared by the node;
- disconnect behaviour;
- reconnect behaviour where supported;
- malformed input and protocol failure paths;
- native-peer transparency.

Mock-to-mock interoperability alone is insufficient to establish matrix
compatibility.

## Security boundary

Annoda interoperability MUST NOT silently weaken security guarantees merely
to obtain compatibility.

Every node MUST document, where relevant:

- peer identity;
- authentication;
- transport encryption;
- integrity protection;
- key material;
- relay trust;
- bootstrap trust;
- metadata exposure;
- persistence;
- runtime risks.

Transport encryption MUST NOT be represented as application-level end-to-end
encryption.

Application-level message encryption and application message semantics remain
outside the baseline Annoda contract unless a future capability explicitly
defines otherwise.