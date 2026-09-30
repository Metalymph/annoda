# Idalion architecture

## Purpose

Idalion is an interoperability layer for peer-to-peer conversational messaging
protocols.

It allows applications and devices to communicate across P2P protocol
boundaries through explicit adapters while preserving the messaging semantics
that participating protocols actually share.

Participating protocols may use substantially different P2P architectures,
identity models, discovery mechanisms, transports, rendezvous systems, relay
strategies, and security models.

    Application / client
            |
            v
    Idalion public contract
            |
            +-- endpoints
            +-- conversational operations
            +-- directional capabilities
            +-- explicit results and constraints
            |
            v
    Adapter boundary
            |
            v
    Native P2P messaging protocol

**Idalion owns the interoperability contract, not the participating networks.**

## Domain boundary

Idalion is intentionally limited to peer-to-peer conversational messaging
interoperability.

Centralized, service-controlled, and federated messaging platforms are outside
the current architecture.

The rationale for that scope decision is recorded in
`CENTRALIZED-PROTOCOL-RESEARCH.md`.

Idalion is not a generic protocol converter, event bus, message broker, RPC
abstraction, webhook bridge, network tunnel, ETL system, centralized messaging
gateway, or application integration framework.

## Core model

The architecture separates four concerns:

- **Endpoint** — where conversational communication originates or terminates.
- **Operation** — what conversational action is requested or observed.
- **Capability** — whether and under which constraints that semantic operation
  is representable.
- **Adapter** — how those semantics map to one native P2P messaging protocol.

These concepts must not be collapsed merely because one protocol exposes them
through a single API, transport primitive, peer abstraction, or session model.

## Endpoint model

Endpoint is Idalion's neutral addressing concept.

Depending on the native P2P protocol, an endpoint may correspond to a peer,
device, public key, protocol-specific address, direct-message destination,
conversation, group, or another protocol-native messaging destination.

Endpoint identifiers remain opaque outside the adapter that owns their
interpretation.

Idalion does not define a global endpoint, peer, or identity namespace.

## Operation model

The public contract represents conversational intent rather than opaque
transport frames.

Operations may eventually include text, replies, reactions, edits, deletion,
attachments, recorded audio, and other verified conversational semantics.

The stable Rust representation is deliberately not fixed here. It evolves from
requirements demonstrated by real adapters.

An operation describes meaning. Wire representation belongs to the adapter.

## Adapter model

One adapter integrates one native P2P messaging protocol with the Idalion
contract.

Adapters own protocol-specific addressing, identity and authentication,
discovery, connection or session lifecycle, encoding, transport interaction,
bootstrap, rendezvous, NAT traversal, relay interaction, constraints, and error
translation where those concepts apply.

Adapter-specific dependencies remain isolated inside the adapter.

Protocol-specific implementation types must not leak into the common API merely
for adapter convenience.

## Semantic preservation

Idalion preserves supported conversational semantics, not identical wire bytes.

Envelopes, identifiers, timestamps, framing, encoding, transport bytes, and
protocol metadata may differ.

For a supported capability, the conversational meaning promised by the
capability contract must survive the mapping. When it cannot, Idalion reports
the limitation instead of manufacturing equivalence.

## Directional capabilities

Interoperability is capability-specific and directional.

A capability may be supported, supported with explicit constraints, or
unsupported.

Support from protocol A to protocol B does not imply support from B to A.
Support for one conversational operation does not imply support for another.

Boolean capability flags alone are insufficient when native protocols impose
meaningful constraints.

## Infrastructure asymmetry

P2P interoperability does not imply infrastructure symmetry.

One adapter may depend on a DHT and bootstrap nodes. Another may use rendezvous
services, hole punching, native relays, direct addressing, or a different
combination of P2P infrastructure.

A protocol may also use supporting servers without becoming equivalent to a
service-controlled messaging platform. The relevant question is whether the
native messaging architecture remains genuinely peer-to-peer rather than
requiring Idalion to operate through a centralized user-messaging API or
synthetic gateway identity.

Idalion documents these differences rather than hiding them.

## Discovery and identity

Discovery is a capability, not a universal assumption, and may be asymmetric.

Idalion does not own user identity and does not require an Idalion account,
global contact graph, universal cross-protocol identity, universal peer
identifier, or mandatory identity authority.

Cross-protocol identity association remains application policy unless multiple
real adapters later demonstrate a narrower shared requirement.

## No Idalion wire protocol or authority

Remote native endpoints communicate according to their native P2P protocols.

Idalion does not require a universal wire format, handshake, codec, transport,
central translation gateway, message store, relay network, discovery service,
or conversation authority.

Infrastructure required by a native P2P protocol remains infrastructure of that
protocol.

## Native fast path

When endpoints can already communicate correctly through the same compatible
native protocol, Idalion should avoid unnecessary adaptation.

Static configurations should permit unused interoperability machinery to
disappear at compile time. Dynamic configurations should bypass unnecessary
adaptation at runtime.

## Runtime boundary

Idalion remains runtime-agnostic.

Adapters may internally use Tokio, Bare or JavaScript runtimes, native event
loops, callbacks, threads, protocol SDKs, or platform APIs.

The common boundary should remain suitable for Rust applications, C ABI
exposure, Swift/native wrappers, Skip integration, React Native native modules
where required, and WASM-compatible environments where the native protocol
permits them.

Runtime neutrality must not force inherently asynchronous native behaviour into
incorrect synchronous semantics.

## Multi-endpoint conversations

Idalion may target endpoints backed by different P2P protocols.

Capability support is evaluated independently for each relevant destination and
direction, and per-destination results are preserved.

Success for one destination must not hide failure, rejection, or unsupported
semantics for another.

Fan-out may execute concurrently where safe, but Idalion must not invent
ordering guarantees absent from native protocols.

## Failure model

The architecture distinguishes unsupported capabilities, constraint violations,
invalid endpoints, native protocol rejection, identity or authentication
failures, connectivity and infrastructure failures, adapter failures, and
partial multi-endpoint failures.

The concrete Rust error representation is defined during API design.

## Security boundary

Interoperability does not imply security equivalence.

If crossing a protocol boundary terminates, changes, or makes impossible a
security property, that fact must remain visible to the embedding application.

Transport encryption must never be represented as application-level end-to-end
encryption.

Different P2P protocols may have materially different identity, key ownership,
relay trust, rendezvous trust, metadata exposure, persistence, and E2EE
properties. Idalion must preserve those distinctions.

## Conformance

Conformance is capability-specific and directional.

Adapters may claim only behaviour verified against the native protocol
implementation or authoritative interface.

Tests validate the semantics claimed, not merely that bytes crossed a boundary.
Mock-to-mock success alone is insufficient evidence of native interoperability.

## Dependency isolation

Applications should pay only for the adapters they use.

Protocol-specific SDKs, runtimes, and implementations belong to their
respective adapters. The Idalion core remains small and dependency-light.

## Evolution rule

The 0.0.x series exists to discover the smallest correct interoperability
contract through real, architecturally different P2P messaging adapters.

Idalion evolves from evidence: no speculative universal messaging ontology,
generic constraint language, global identity, central authority, or expansion
beyond P2P conversational messaging.

Shared abstractions enter Idalion only after real adapters demonstrate that they
are genuinely common.

Active implementation is currently paused. Architectural evolution should
resume when a concrete P2P-to-P2P integration provides requirements that can be
verified against native protocols.