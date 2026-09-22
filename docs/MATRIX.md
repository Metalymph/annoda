# Annoda interoperability matrix

This document is the canonical index of protocol interoperability provided by
Annoda.

Annoda compatibility is explicit and pairwise.

A protocol pair is supported only when a complete bidirectional node has been
implemented and verified according to the Annoda interoperability contract.

## Matrix semantics

Each off-diagonal matrix entry represents one explicit interoperability
relationship:

```text
Protocol A <-> Protocol B
```

A `supported` entry guarantees both:

```text
A -> B
B -> A
```

for the capability profile and protocol versions declared by that node.

The matrix does not represent inferred or transitive compatibility.

For example:

```text
A <-> B = supported
B <-> C = supported
```

does not imply:

```text
A <-> C = supported
```

An explicit `A <-> C` node is required.

## Status values

Matrix entries use the following states:

- `native*` — both peers use the same compatible protocol; Annoda is bypassed;
- `planned` — interoperability work is intended, but no compatibility
  guarantee exists;
- `experimental` — implementation work exists, but the complete Annoda
  conformance gate has not passed;
- `supported` — complete bidirectional interoperability has been verified
  against native implementations;
- `—` — no Annoda node currently exists for the protocol pair.

Only `supported` represents an Annoda interoperability guarantee.

## Current matrix

| Protocol | Hyperswarm | Iroh | libp2p |
|---|---|---|---|
| **Hyperswarm** | native* | planned | — |
| **Iroh** | planned | native* | — |
| **libp2p** | — | — | native* |

The matrix is symmetric because a complete Annoda node is bidirectional.

A partial one-way experiment does not create a supported matrix entry.

## Native fast path

Diagonal entries are marked:

```text
native*
```

because same-protocol communication does not require Annoda.

For example:

```text
Iroh <-> Iroh
```

uses Iroh natively.

Likewise:

```text
Hyperswarm <-> Hyperswarm
```

uses Hyperswarm natively.

Annoda MUST NOT introduce protocol translation into these paths.

Where both protocols are known at compile time, implementations SHOULD allow
the Annoda interoperability path to be eliminated entirely.

Where protocol selection is dynamic, same-protocol communication MUST bypass
node translation completely.

`native*` therefore means that Annoda contributes no interoperability layer to
the communication path.

## Planned first node

The first intended real interoperability investigation is:

```text
Hyperswarm <-> Iroh
```

This pair is useful because the two stacks are architecturally distinct and
provides a concrete test of Annoda's core premise.

Its current `planned` status is not a compatibility claim.

Before becoming `supported`, the node must demonstrate at least:

```text
native Hyperswarm -> Annoda node -> native Iroh
native Iroh       -> Annoda node -> native Hyperswarm
```

with exact application payload byte-stream parity in both directions.

The native peer on the side not hosting Annoda must remain unmodified.

## Matrix admission

A protocol pair MUST NOT be marked `supported` merely because:

- both protocols have implementations available;
- a conceptual mapping appears possible;
- their application APIs look similar;
- an Annoda mock can communicate with another Annoda mock;
- one direction works;
- a proof of concept exchanges one message;
- both protocols independently support arbitrary byte streams.

A `supported` entry requires the complete node conformance gate.

This includes native-peer testing, bidirectional communication, payload
byte-stream parity, declared lifecycle behaviour, capability validation, and
security review.

## Version scope

Matrix status alone does not imply compatibility with every version of a
protocol.

Each node must maintain its own explicit version or compatibility range.

For example, a `supported` matrix entry means:

```text
supported for the versions and capability profile declared by the node
```

not:

```text
supported for every past and future release of both protocols
```

A relevant wire-level change may require renewed conformance before a new
protocol version is included in the supported range.

## Capability scope

A matrix entry represents the node's declared capability profile.

Baseline Annoda support requires bidirectional messaging with exact application
payload byte-stream parity.

A node may additionally support capabilities such as:

- large payloads;
- chunked transfer;
- blob transfer;
- file transfer;
- recorded audio payload transfer.

These additional capabilities are node-specific and must be documented and
tested independently.

A `supported` matrix entry MUST NOT be interpreted as complete feature
equivalence between the two protocols.

## No intermediate protocol

No matrix relationship implies the existence of an Annoda wire protocol.

For an Annoda-enabled peer communicating with a native Protocol B peer, the
network-facing communication must be valid Protocol B communication.

Any representation used internally by the node remains local to that node.

## Adding a protocol pair

A new pair enters the matrix through the following lifecycle:

```text
no node
   |
   v
planned
   |
   v
experimental
   |
   v
supported
```

`planned` records an intended interoperability relationship.

`experimental` records implementation progress without promising complete
compatibility.

`supported` is reached only after the node satisfies the normative contract
and conformance requirements.

A node may return from `supported` to `experimental` if a relevant protocol
change invalidates its verified compatibility range.

## Source of truth

This matrix is the repository-level index of Annoda interoperability.

Detailed guarantees, versions, platforms, capabilities, limitations, and
security properties belong to each individual node's documentation.

The normative meaning of Annoda interoperability is defined by
`docs/CONTRACT.md`.