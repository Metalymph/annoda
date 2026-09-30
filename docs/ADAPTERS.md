# Writing an Idalion adapter

An Idalion adapter integrates one peer-to-peer conversational messaging protocol
with the Idalion interoperability contract.

Adapters are protocol-specific. They preserve the native P2P architecture
rather than forcing unrelated protocols into one universal peer, transport,
discovery, or runtime model.

Centralized, service-controlled, and federated messaging integrations are
outside the current adapter contract.

## Responsibilities

An adapter owns the protocol-specific subset of:

- endpoint representation and native addressing;
- inbound and outbound semantic mapping;
- identity and authentication;
- discovery where supported;
- connection or session lifecycle where applicable;
- native message encoding and decoding;
- attachments and other supported conversational media;
- bootstrap and rendezvous integration;
- NAT traversal where applicable;
- relay integration where applicable;
- native P2P infrastructure integration;
- protocol limits and capability constraints;
- error translation;
- security-boundary documentation.

Not every protocol requires every responsibility.

A protocol must not be forced to expose another protocol's peer identity,
connection model, discovery mechanism, transport, relay strategy, or session
semantics merely to fit Idalion.

## Endpoint ownership

The adapter owns the interpretation of its endpoint identifiers.

An endpoint may represent a peer, device, public key, protocol-specific address,
conversation, group, or another native P2P messaging destination.

Endpoint identifiers remain opaque outside the responsible adapter.

Adapters must not create a synthetic global Idalion identity merely to make
unrelated protocols appear uniform.

## Semantic mapping

Adapters operate on conversational semantics rather than arbitrary transport
frames.

A supported operation must preserve the conversational meaning promised by its
Idalion capability contract.

Native envelopes, framing, identifiers, timestamps, protocol messages, and
transport bytes may differ.

An adapter must not silently replace an unsupported operation with a materially
different conversational operation. Unsupported or constrained behaviour must
remain explicit.

## Directional capabilities

Capability declarations are directional.

Support in one direction does not imply support in the reverse direction.

Adapters must report meaningful native constraints rather than advertising
unconditional support.

Concrete constraint types are added only when real protocols require them.

## Native behaviour

Communication emitted toward a native system must be valid native protocol
behaviour.

Remote native endpoints must not require an Idalion-specific codec, handshake,
runtime, wire format, or client modification merely to participate.

An adapter must not introduce an Idalion-operated gateway identity merely to
manufacture interoperability.

## Infrastructure honesty

Adapters may use DHTs, bootstrap nodes, rendezvous services, relays, NAT
traversal infrastructure, discovery mechanisms, signaling infrastructure, or
other facilities belonging to their native P2P protocol.

Using supporting servers does not by itself make a protocol non-P2P and does
not make that infrastructure Idalion infrastructure.

Adapters must document the actual architecture rather than hiding whether
communication is direct, relayed, rendezvous-assisted, or dependent on other
native infrastructure.

Every adapter must document relevant infrastructure dependencies and trust
implications.

## Runtime and dependency isolation

Adapters may internally use the runtime appropriate to their native
implementation.

Protocol-specific dependencies remain inside the adapter. Applications must not
be required to include dependencies belonging exclusively to unrelated
adapters.

The common Idalion boundary remains runtime-agnostic and dependency-light.

## Security requirements

Every adapter documents relevant:

- endpoint identity;
- authentication;
- transport confidentiality;
- application-level E2EE;
- message authenticity;
- key ownership;
- bootstrap trust;
- rendezvous trust;
- relay trust;
- metadata exposure;
- persistence;
- downgrade properties.

An adapter must not silently weaken a security property merely to obtain
interoperability.

Transport encryption must not be represented as application-level E2EE.

If interoperability changes or terminates a native end-to-end security
property, that boundary must remain explicit to the embedding application.

## Conformance

Conformance is directional and capability-specific.

An adapter may claim only behaviour verified against the native protocol
implementation or authoritative interface.

Tests should cover the operations, constraints, lifecycle, failure paths, and
security properties actually claimed by that adapter.

Where applicable, native validation should include addressing, discovery,
connection establishment, reconnect behaviour, relay behaviour, duplicate
handling, ordering, and malformed input.

Mock-to-mock success alone is insufficient evidence of native interoperability.

The Idalion testkit will provide reusable conformance helpers as the real
adapter model becomes established.

## Required documentation

Every adapter documents at least:

- native protocol and relevant versions;
- supported platforms and runtimes where relevant;
- endpoint representation;
- inbound and outbound capabilities;
- capability constraints and unsupported operations;
- addressing and discovery behaviour;
- connection and lifecycle behaviour where relevant;
- bootstrap, rendezvous, NAT traversal, and relay requirements where relevant;
- known semantic differences;
- security properties and metadata exposure;
- dependencies and known limitations.

Documentation must distinguish verified behaviour from planned behaviour.

An adapter must not claim general Idalion compatibility merely because it can
exchange arbitrary bytes with another endpoint. Compatibility is established
through the conversational capabilities actually implemented and verified.

## Adapter layout

    adapters/
      idalion-adapter-<protocol>/
        Cargo.toml
        README.md
        src/
        tests/

External adapters may live in separate repositories. In-tree inclusion is not
required for ecosystem compatibility.

## Evolution rule

New adapters validate the common contract rather than forcing premature
generalization.

When P2P protocols differ, Idalion represents the difference honestly.

A common abstraction enters the core only after multiple real adapters
demonstrate that it is genuinely shared.

Adapter work is currently paused with the rest of active Idalion development.
New adapters should be introduced when a concrete P2P interoperability use case
justifies resuming implementation.