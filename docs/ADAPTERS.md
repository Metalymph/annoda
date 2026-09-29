# Writing an Idalion adapter

An Idalion adapter integrates one conversational messaging protocol with the Idalion interoperability contract.

The underlying system may be peer-to-peer, server-mediated, service-backed, or use another architecture appropriate to conversational messaging.

## Responsibilities

An adapter owns the protocol-specific subset of:

- endpoint representation and native addressing;
- inbound and outbound semantic mapping;
- authentication;
- discovery where supported;
- connection or session lifecycle where applicable;
- native message encoding and decoding;
- attachments and other supported conversational media;
- native infrastructure integration;
- protocol limits and capability constraints;
- error translation;
- security-boundary documentation.

Not every protocol requires every responsibility.

A server-mediated protocol must not be forced to expose artificial P2P connection semantics, and a P2P protocol must not be forced into an account/server model.

## Endpoint ownership

The adapter owns the interpretation of its endpoint identifiers.

An endpoint may represent a peer, account, user, device, conversation, group, channel, or another native messaging destination.

Endpoint identifiers remain opaque outside the responsible adapter.

Adapters must not create a synthetic global Idalion identity merely to make unrelated protocols appear uniform.

## Semantic mapping

Adapters operate on conversational semantics rather than arbitrary transport frames.

A supported operation must preserve the conversational meaning promised by its Idalion capability contract.

Native envelopes, framing, identifiers, timestamps, service requests, and transport bytes may differ.

An adapter must not silently replace an unsupported operation with a materially different conversational operation. Unsupported or constrained behaviour must remain explicit.

## Directional capabilities

Capability declarations are directional.

Support in one direction does not imply support in the reverse direction.

Adapters must report meaningful native constraints rather than advertising unconditional support.

Concrete constraint types are added only when real protocols require them.

## Native behaviour

Communication emitted toward a native system must be valid native protocol behaviour.

Remote native endpoints must not require an Idalion-specific codec, handshake, runtime, wire format, or client modification merely to participate.

## Infrastructure honesty

Adapters may use provider servers, APIs, authentication services, push infrastructure, DHTs, bootstrap nodes, rendezvous services, relays, or other infrastructure belonging to their native protocol.

Using such infrastructure does not make it Idalion infrastructure.

Every adapter must document relevant infrastructure dependencies and trust implications.

## Runtime and dependency isolation

Adapters may internally use the runtime appropriate to their native implementation.

Protocol-specific dependencies remain inside the adapter. Applications must not be required to include dependencies belonging exclusively to unrelated adapters.

The common Idalion boundary remains runtime-agnostic and dependency-light.

## Security requirements

Every adapter documents relevant identity, authentication, confidentiality, E2EE, authenticity, key ownership, server or relay trust, metadata exposure, persistence, and downgrade properties.

An adapter must not silently weaken a security property merely to obtain interoperability.

Transport encryption must not be represented as application-level E2EE.

## Conformance

Conformance is directional and capability-specific.

An adapter may claim only behaviour verified against the native protocol implementation or authoritative interface.

Tests should cover the operations, constraints, lifecycle, failure paths, and security properties actually claimed by that adapter.

Mock-to-mock success alone is insufficient evidence of native interoperability.

The Idalion testkit will provide reusable conformance helpers as the real adapter model becomes established.

## Required documentation

Every adapter documents at least:

- native protocol and relevant versions;
- supported platforms and runtimes where relevant;
- endpoint representation;
- inbound and outbound capabilities;
- capability constraints and unsupported operations;
- addressing and discovery behaviour;
- infrastructure and authentication requirements;
- known semantic differences;
- security properties and metadata exposure;
- dependencies and known limitations.

Documentation must distinguish verified behaviour from planned behaviour.

## Adapter layout

```text
adapters/
  idalion-adapter-<protocol>/
    Cargo.toml
    README.md
    src/
    tests/
```

External adapters may live in separate repositories. In-tree inclusion is not required for ecosystem compatibility.

## Evolution rule

New adapters validate the common contract rather than forcing premature generalization.

When protocols differ, Idalion represents the difference honestly.

A common abstraction enters the core only after multiple real adapters demonstrate that it is genuinely shared.
