# Security Policy

Annoda is security-sensitive interoperability infrastructure.

A node participates directly in communication between otherwise incompatible
peer-to-peer protocols. Functional interoperability must therefore never be
treated as sufficient evidence of secure interoperability.

## Reporting a vulnerability

Do not open a public issue for a suspected vulnerability.

Use GitHub's private vulnerability reporting or Security Advisory flow for
this repository.

When reporting a vulnerability, include enough information to reproduce and
understand the issue without unnecessarily exposing sensitive data.

## Security invariant

An Annoda node MUST NOT silently weaken a security property merely to make two
protocols communicate.

If a security property cannot be preserved, reproduced, or meaningfully mapped
across a protocol pair, the limitation MUST be explicit.

A node MUST fail rather than claim a security guarantee that it does not
actually provide.

## Security properties are distinct

The following properties are separate and MUST NOT be conflated:

- peer identity;
- peer authentication;
- transport confidentiality;
- transport integrity;
- application message authenticity;
- application-level end-to-end encryption;
- metadata privacy.

For example, encrypted transport between peers does not by itself provide
application-level end-to-end encryption.

Annoda documentation and node capability claims MUST describe these properties
independently.

## Node security review

Every Annoda node MUST receive a security review appropriate to both protocols
and to the bridge between them.

At minimum, the review MUST consider:

- identity semantics on Protocol A;
- identity semantics on Protocol B;
- authentication on both protocols;
- transport confidentiality;
- transport integrity;
- key generation;
- key storage and persistence;
- key exchange where relevant;
- handshake behaviour;
- downgrade opportunities;
- relay trust;
- bootstrap trust;
- discovery trust;
- rendezvous trust where applicable;
- metadata exposure;
- addressing information exposure;
- malformed-input handling;
- parser and codec attack surface;
- framing conversion;
- encoding conversion;
- resource-exhaustion risks;
- dependency risks;
- runtime risks;
- FFI or native-library risks where applicable.

The review MUST distinguish properties inherited from native protocols from
properties introduced, transformed, or affected by Annoda.

## Protocol conversion boundary

Encoding, decoding, framing, handshake adaptation, and other required protocol
conversion happen on the peer hosting Annoda.

The native remote peer MUST receive only communication valid for its own
protocol.

For an `A <-> B` node hosted on the A peer:

```text
A-side representation
        |
        v
+-------------------+
| Annoda A <-> B    |
+-------------------+
        |
        v
valid Protocol B communication
        |
        v
native B peer
```

The reverse path consumes native Protocol B communication and converts it into
the representation required by the A side.

Protocol conversion MUST NOT require the native remote peer to disable or
bypass its normal security checks merely to interoperate with Annoda.

## No intermediate security domain

Annoda does not define an intermediate network protocol and MUST NOT create an
intermediate network security domain.

An internal representation MAY exist inside a node for implementation
purposes, but it MUST remain local to the node.

It MUST NOT become:

- an Annoda wire protocol;
- an Annoda network handshake;
- a shared Annoda encryption layer required by both peers;
- a central translation boundary;
- a central trust authority.

The network-facing side of the node must participate in the security model of
the native protocol it is speaking.

## Native-peer security

The remote native peer MUST NOT require Annoda-specific security behaviour.

It MUST NOT need:

- Annoda-specific credentials;
- Annoda-specific keys;
- an Annoda-specific certificate;
- an Annoda-specific authentication protocol;
- an Annoda-specific encryption scheme;
- an Annoda-specific trusted intermediary.

Protocol-native credentials, identities, keys, relays, bootstrap systems, and
other security mechanisms may still be required when they are part of the
native protocol.

Those requirements MUST be documented by the node.

## Identity mapping

Protocols may use different identity models.

An Annoda node MUST NOT imply that identities from Protocol A and Protocol B
are equivalent unless the node explicitly establishes and verifies such a
mapping.

Where identity translation or binding is necessary, the node MUST document:

- what identifies a peer on each protocol;
- how identities are obtained;
- whether identities are persistent or ephemeral;
- whether any mapping is authenticated;
- where the mapping is stored;
- what trust assumptions the mapping introduces;
- what happens when identity verification fails.

Application-level user identity remains outside the baseline Annoda contract.

## Key material

Nodes SHOULD minimize access to secret key material.

Where a node must create, import, retain, transform, or otherwise access key
material, its documentation MUST describe:

- why access is necessary;
- which protocol owns the key material;
- where it is generated;
- where it is stored;
- its lifetime;
- whether it can be exported;
- how failure or corruption is handled.

A node MUST NOT unnecessarily copy protocol secrets into shared Annoda state.

## Protocol-native infrastructure

Annoda does not require central Annoda infrastructure.

A node MAY depend on infrastructure belonging to its native protocols,
including:

- relays;
- DHT infrastructure;
- bootstrap nodes;
- rendezvous services;
- discovery services.

Every such dependency MUST be attributed to the protocol that requires it.

Node documentation MUST describe relevant trust and privacy consequences,
including metadata exposure where known.

## Payload integrity

Annoda's functional contract requires exact application payload byte-stream
parity.

Security-sensitive implementations MUST ensure that protocol conversion does
not accidentally mutate, reinterpret, truncate, extend, normalize, or
otherwise alter opaque application payload bytes.

Payloads MUST remain opaque to Annoda unless a future explicitly declared
capability requires additional semantics.

Text normalization, Unicode normalization, serialization changes, or
application-level transcoding MUST NOT be applied implicitly.

## Malformed and hostile input

Network input MUST be treated as untrusted.

Nodes MUST handle malformed, truncated, oversized, unexpected, and
protocol-invalid input without violating memory safety or silently producing
valid-looking application payloads.

Where possible, invalid protocol input SHOULD be rejected before it reaches
application-level payload handling.

Protocol conversion MUST NOT turn invalid input from one protocol into
apparently valid trusted input on the other side without an explicit and
justified rule.

## Resource exhaustion

Nodes SHOULD consider denial-of-service and resource-exhaustion risks,
including:

- unbounded frame sizes;
- unbounded buffering;
- excessive concurrent streams;
- connection churn;
- reconnect loops;
- decompression or decoding amplification;
- expensive malformed inputs;
- queue growth;
- memory retention.

Limits that affect interoperability MUST be documented as part of the node's
declared profile.

## Dependency boundary

Protocol interoperability may require substantial third-party protocol
implementations, runtimes, native libraries, or FFI.

Every node MUST keep this dependency footprint isolated from unrelated nodes
where practical.

Security review SHOULD consider:

- dependency provenance;
- maintenance status;
- unsafe code;
- native code;
- FFI boundaries;
- runtime permissions;
- update strategy;
- known vulnerability response.

Adding one protocol pair MUST NOT silently expand the attack surface of
applications that do not consume that node.

## Application-level encryption

Application-level end-to-end encryption remains above the baseline Annoda
interoperability layer.

Annoda MAY transport already encrypted application payloads as opaque bytes.

A node MUST preserve those bytes exactly according to the payload byte-stream
parity guarantee.

The existence of encrypted native transports on both sides MUST NOT be
described as equivalent to application-level E2EE.

## Security and matrix support

A node MUST NOT be marked `supported` solely because functional payload
exchange succeeds.

Security review is part of the conformance evidence required for a supported
interoperability claim.

If a newly discovered security issue invalidates a node's declared guarantees,
its support status SHOULD be reconsidered until the affected guarantee is
restored or accurately narrowed.

## Scope

This policy defines repository-level security expectations.

Each protocol-pair node remains responsible for documenting its concrete
security model, limitations, dependencies, and trust assumptions.

The normative interoperability requirements are defined in
`docs/CONTRACT.md`.