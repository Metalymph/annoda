# Contributing to Annoda

Annoda grows through explicit, independently testable interoperability
relationships between peer-to-peer protocols.

Contributions should preserve that model.

Annoda is not a universal P2P abstraction and new protocol support should not
widen the shared architecture merely to accommodate one protocol pair.

## Proposing a new node

Before implementing a new interoperability node, open a **Node proposal**
issue.

A node proposal represents a concrete protocol pair:

```text
Protocol A <-> Protocol B
```

The proposal should establish whether bidirectional interoperability is
technically possible before substantial implementation work begins.

## Required proposal information

A node proposal MUST identify:

- Protocol A and its upstream project;
- Protocol B and its upstream project;
- intended protocol versions or compatibility ranges;
- supported platforms and architectures;
- relevant runtime requirements;
- expected node placement;
- identity models;
- addressing models;
- discovery mechanisms;
- NAT traversal behaviour;
- relay requirements;
- bootstrap or rendezvous requirements;
- transport-security properties;
- wire framing;
- encoding and codec behaviour;
- stream or message semantics;
- expected translation strategy;
- dependency footprint;
- baseline messaging expectations;
- potential additional capabilities;
- known incompatibilities or unresolved questions.

The proposal MUST explain how an Annoda-enabled peer can communicate with an
unmodified native peer of the other protocol.

## Fundamental node requirements

A complete node MUST satisfy the normative requirements in
`docs/CONTRACT.md`.

In particular, it MUST:

- provide bidirectional `A <-> B` interoperability;
- preserve application payload bytes exactly;
- produce valid native protocol communication toward the remote peer;
- consume valid native protocol communication from the remote peer;
- perform required encoding and framing conversion locally;
- introduce no intermediate Annoda wire protocol;
- require no Annoda modification on the native remote peer;
- require no central Annoda translation service;
- declare its supported versions and capability profile;
- pass native-peer conformance testing.

A one-way implementation MAY be useful during development but is not a
complete Annoda node.

## Payload byte-stream parity

Payload byte-stream parity is not optional.

For every payload within the declared supported profile:

```text
send_A(payload) -> receive_B(payload)
send_B(payload) -> receive_A(payload)
```

the receiving application MUST observe the exact byte sequence sent by the
originating application.

Protocol-specific wire bytes may differ.

When protocols use different encodings, codecs, framing, envelopes, stream
representations, or other wire structures, conversion belongs inside the
node.

Do not solve protocol differences by requiring the remote peer to understand
an Annoda-specific representation.

## No intermediate Annoda protocol

Contributions MUST NOT introduce a universal Annoda network protocol between
peers.

An internal representation MAY be introduced when useful for implementing a
specific node or proven shared functionality.

Such a representation MUST remain internal.

It MUST NOT:

- appear as a required network protocol;
- require both peers to use Annoda;
- become a universal Annoda codec;
- require a central translation service.

## Native fast path

Same-protocol communication is native communication.

Contributions MUST NOT require an Annoda node for:

```text
A <-> A
```

merely to create a uniform abstraction.

Where protocol selection is statically known, implementations SHOULD permit
the Annoda interoperability path to be eliminated at compile time.

Where selection is dynamic, same-protocol communication MUST bypass
translation.

## Keep nodes isolated

A node should depend only on what is necessary for its own protocol pair.

Adding:

```text
A <-> B
```

must not force consumers of:

```text
C <-> D
```

to compile, link, ship, or initialize implementations belonging exclusively
to A or B.

Avoid global dependencies and shared abstractions until multiple real nodes
demonstrate that they are necessary.

## Shared architecture changes

Changes to Annoda's shared core require stronger justification than
protocol-specific node changes.

Do not widen the core because one protocol has a particular concept.

Before introducing a shared abstraction, ask whether it is:

1. required by the normative Annoda contract;
2. independently demonstrated by multiple protocol pairs; or
3. merely an implementation detail of one node.

Protocol-specific behaviour belongs in the node unless there is concrete
evidence that it belongs in shared infrastructure.

## Conformance

A complete node MUST be tested against native implementations of the protocols
it claims to support.

The essential topology is:

```text
native A -> Annoda node -> native B
native B -> Annoda node -> native A
```

Conformance must verify the declared guarantees rather than merely exercise
internal APIs.

At minimum, node tests should cover:

- connection establishment;
- A -> B payload exchange;
- B -> A payload exchange;
- simultaneous bidirectional traffic;
- exact payload byte-stream parity;
- arbitrary binary payloads;
- boundary conditions and supported payload limits;
- repeated messages;
- declared ordering behaviour;
- disconnect behaviour;
- reconnect behaviour where supported;
- malformed input;
- invalid or truncated protocol data;
- native protocol rejection;
- remote failure;
- local failure;
- security-relevant failure paths;
- native-peer transparency.

Mock-to-mock tests are useful during development but are insufficient for a
`supported` interoperability claim.

## Matrix updates

`docs/MATRIX.md` is the repository-level index of interoperability status.

Use matrix states according to their defined meaning:

```text
planned
experimental
supported
```

Do not mark a protocol pair `supported` until its complete bidirectional
conformance gate has passed.

If only one direction works, the pair remains experimental.

If a protocol update invalidates the verified compatibility range, update the
node documentation and matrix status accordingly.

## Security

Every node contribution MUST include a security review appropriate to the
protocol pair.

Document at least:

- peer identity;
- authentication;
- transport confidentiality;
- transport integrity;
- key handling;
- relay trust;
- bootstrap and discovery trust;
- metadata exposure;
- downgrade risks;
- malformed-input handling;
- parser and codec attack surface;
- dependency and runtime risks.

A node MUST NOT silently weaken a security property merely to obtain
interoperability.

Transport encryption MUST NOT be described as application-level end-to-end
encryption.

See `SECURITY.md` for the repository security policy.

## Dependencies

Keep dependencies deliberate and minimal.

A new dependency should have a concrete role in satisfying the node's
interoperability contract.

When adding substantial protocol implementations, runtimes, native libraries,
FFI layers, or other significant dependencies, document:

- why the dependency is required;
- where it is used;
- its platform implications;
- its security implications;
- whether it affects consumers of unrelated nodes.

Unrelated nodes must remain independently consumable.

## Rust pull request gate

Before submitting Rust changes, run:

```console
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
cargo package -p annoda
```

Additional node-specific tests and native interoperability suites MUST also
pass when applicable.

Public Rust APIs and public data structures MUST be documented.

## Pull request expectations

Keep pull requests focused and reviewable.

A node implementation should include, as applicable:

- implementation changes;
- unit tests;
- native-peer interoperability tests;
- conformance evidence;
- protocol/version documentation;
- capability documentation;
- security notes;
- dependency rationale;
- matrix updates.

Do not mix unrelated architectural refactors into a node contribution unless
they are required for the node and independently justified.

## Development states

Experimental work is welcome.

It must simply be labelled accurately.

A useful progression is:

```text
proposal
   |
   v
prototype
   |
   v
experimental node
   |
   v
native conformance
   |
   v
supported node
```

The purpose of this process is not bureaucracy.

It ensures that an Annoda compatibility claim has one precise meaning:
two otherwise incompatible native P2P protocols have a verified,
bidirectional interoperability path with exact application payload
byte-stream parity.