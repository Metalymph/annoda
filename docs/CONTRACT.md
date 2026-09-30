# Idalion interoperability contract

## Purpose

Idalion enables conversational messaging across peer-to-peer protocol
boundaries.

It allows endpoints using otherwise incompatible P2P messaging protocols to
communicate when their respective capabilities admit a meaningful mapping.

Idalion does not make protocols identical. It makes explicitly supported
messaging semantics interoperable.

Participating protocols remain responsible for their own native identity,
addressing, discovery, connectivity, transport, relay, persistence, and
security models.

## Scope

Idalion is specifically an interoperability layer for **peer-to-peer
conversational messaging protocols**.

Its scope includes messaging capabilities such as:

- text messages;
- replies;
- reactions;
- edits;
- deletion;
- attachments;
- recorded audio messages;
- delivery and read state;
- conversational groups;
- addressing and discovery where exposed by the participating protocols.

Support for any capability is explicit and protocol-dependent.

Centralized, service-controlled, and federated messaging platforms are outside
the current Idalion interoperability contract.

This boundary is intentional. Research into centralized messaging platforms and
the reasons for excluding them from the current scope are recorded separately
in `CENTRALIZED-PROTOCOL-RESEARCH.md`.

Idalion is not a generic integration framework, event bus, message broker,
network tunnel, RPC abstraction, webhook bridge, ETL system, centralized
messaging gateway, or universal protocol converter.

## Endpoints, not universal peers

Idalion models communicating parties as endpoints.

An endpoint is a protocol-native P2P messaging destination or source as
represented by an adapter.

Depending on the native protocol, an endpoint may correspond to a peer, device,
public key, protocol-specific address, conversation destination, or another
P2P-native addressing concept.

Idalion MUST NOT require every protocol to expose the same peer identifier,
address representation, discovery mechanism, or device model.

Protocol-specific identities and addresses remain opaque outside the adapter
that owns their interpretation.

## Explicit adapters

Each supported protocol is integrated through an explicit adapter.

An adapter owns the protocol-specific behaviour required to participate in its
native P2P messaging system and to expose supported semantics through the
Idalion contract.

This may include, where applicable:

- addressing;
- discovery;
- identity and authentication;
- connection or session establishment;
- native message encoding;
- transport interaction;
- attachment handling;
- bootstrap or rendezvous;
- NAT traversal;
- relay interaction;
- protocol-native infrastructure;
- lifecycle behaviour;
- error translation.

Idalion MUST NOT infer support for a protocol merely because another protocol
has superficially similar APIs, topology, transports, or message types.

## Capability-based interoperability

Interoperability is defined per capability.

Support for one capability does not imply support for another.

For a source protocol A and destination protocol B, a capability may be:

- supported;
- supported with explicit constraints;
- unsupported.

Capability support MAY differ by direction.

For example:

                             A -> B       B -> A
    text                     supported    supported
    reply                    supported    supported
    reaction                 constrained  supported
    discovery                supported    unsupported
    feature X                unsupported  unsupported

Idalion MUST NOT fabricate a capability that the destination protocol cannot
meaningfully represent.

## Asymmetric interoperability

P2P protocol interoperability does not imply infrastructure symmetry or
capability symmetry.

Protocols may differ fundamentally in:

- identity models;
- endpoint addressing;
- discovery;
- authentication;
- connection establishment;
- NAT traversal;
- rendezvous and bootstrap;
- relay requirements;
- transport behaviour;
- delivery semantics;
- persistence;
- group semantics;
- message mutation;
- metadata exposure;
- encryption;
- connectivity and lifecycle.

These differences are part of the interoperability model and MUST remain
explicit.

An interoperability relationship MAY therefore support:

    A -> B

for a capability without supporting:

    B -> A

for that same capability.

Idalion MUST NOT describe such a relationship as symmetric.

## Semantic preservation

Idalion preserves the semantics that participating protocols actually share.

Exact wire representation is not preserved and exact application payload bytes
are not a universal Idalion invariant.

For a supported capability, an adapter MUST preserve the meaning and observable
behaviour required by the declared Idalion capability contract to the extent
promised by its capability profile.

For example, a text message may acquire different protocol-native envelopes,
identifiers, timestamps, framing, encoding, or transport representation while
remaining the same conversational message for the purpose of the declared
mapping.

Protocol-specific metadata MUST NOT be presented as preserved when the
destination protocol cannot represent it faithfully.

## No silent semantic degradation

Idalion MUST NOT silently convert an unsupported semantic operation into a
materially different operation merely to make delivery appear successful.

For example, an unsupported reaction MUST NOT automatically become a plain text
message unless an application explicitly requests such a policy outside the
baseline Idalion contract.

When faithful representation is unavailable, the operation MUST produce an
explicit unsupported or constrained result according to the capability
contract.

Lossy mappings, when intentionally supported in the future, MUST be explicit,
inspectable, and opt-in. They MUST NOT be baseline behaviour.

## Protocol-native representation

Communication emitted toward a native endpoint MUST be valid for that
endpoint's native protocol.

Idalion MAY use internal semantic representations while adapting operations,
but those representations MUST remain implementation details.

Idalion MUST NOT require remote native endpoints to understand an
Idalion-specific wire format.

## No Idalion wire protocol

Idalion does not define a universal network protocol between messaging
endpoints.

It MUST NOT require:

- an Idalion wire format;
- an Idalion handshake between remote peers;
- an Idalion codec on remote native clients;
- an Idalion transport shared by all adapters.

An application embedding Idalion communicates through each P2P system according
to that system's native protocol and infrastructure requirements.

## No Idalion authority

Idalion MUST NOT require a central authority for interoperability.

Idalion does not inherently provide or own:

- global user accounts;
- universal Idalion identities;
- a global contact directory;
- a central message store;
- a mandatory translation gateway;
- a mandatory relay network;
- a global discovery service;
- a central conversation authority.

Infrastructure required by a native P2P protocol remains infrastructure of that
protocol.

An adapter may legitimately depend on its protocol's native DHT, bootstrap
nodes, rendezvous mechanisms, relays, discovery services, NAT traversal
facilities, or other P2P infrastructure.

Idalion MUST document these dependencies rather than pretending different P2P
protocols use equivalent infrastructure.

## Native fast path

When two endpoints can communicate natively through the same compatible
protocol, Idalion SHOULD NOT insert interoperability work merely for
architectural uniformity.

Where protocol selection is statically known, implementations SHOULD allow
unnecessary Idalion machinery to disappear at compile time.

Where protocol selection is dynamic, implementations SHOULD bypass adaptation
when no adaptation is required.

## Identity and addressing

Idalion does not define one universal P2P identity model.

Protocol-native identities, endpoint addresses, public keys, conversation
identifiers, device identifiers, and peer identifiers remain owned by their
respective protocols.

Adapters MUST expose only the information required by the Idalion contract and
MUST NOT force unrelated P2P protocols into a synthetic global identity scheme.

Cross-protocol identity association, when required by an application, is
application policy unless a future Idalion capability explicitly specifies
otherwise.

## Conversations and groups

Conversational groups MAY span endpoints backed by different P2P protocols when
the required operations can be represented by all participating destinations.

Capability availability for a multi-endpoint operation is determined by the
relevant directional capability profiles of its destinations.

Idalion MUST support explicit partial-failure reporting when an operation
targets multiple endpoints and succeeds for only a subset.

Idalion MUST NOT report a cross-protocol group operation as universally
successful when one or more required destinations rejected or could not
represent it.

Ordering and concurrency guarantees MUST be declared rather than assumed to be
identical across protocols.

## Protocol-native infrastructure

Adapters MAY depend on infrastructure belonging to their native P2P protocols.

Examples include:

- DHTs;
- bootstrap nodes;
- relays;
- rendezvous services;
- NAT traversal infrastructure;
- peer discovery mechanisms;
- protocol-specific signaling infrastructure.

The existence of relays, bootstrap services, rendezvous servers, or other
supporting infrastructure does not by itself place a protocol outside Idalion's
P2P scope.

The relevant distinction is architectural: the protocol must expose a genuine
P2P messaging model rather than requiring Idalion to integrate through a
service-controlled user messaging API or synthetic gateway identity.

Every adapter MUST document infrastructure dependencies relevant to its
supported capability profile, including trust and metadata implications where
applicable.

## Security boundary

Interoperability MUST NOT be represented as preserving a security property that
does not survive the protocol boundary.

Adapters MUST document relevant differences in:

- endpoint identity;
- authentication;
- transport confidentiality;
- application-level end-to-end encryption;
- message authenticity;
- key ownership;
- relay trust;
- bootstrap or rendezvous trust;
- metadata exposure;
- persistence;
- protocol downgrade risk.

Transport encryption MUST NOT be described as application-level end-to-end
encryption.

If crossing a protocol boundary necessarily terminates or changes an end-to-end
security property, that boundary MUST be explicit to the embedding application
and MUST NOT be hidden by Idalion.

## Capability honesty

Every adapter MUST declare:

- the protocol and relevant versions or compatibility range;
- supported platforms and runtimes where relevant;
- supported inbound capabilities;
- supported outbound capabilities;
- capability constraints;
- addressing and discovery requirements;
- infrastructure requirements;
- security assumptions;
- known semantic differences;
- known limitations.

Idalion MUST NOT advertise interoperability that has not been implemented and
verified against native protocol behaviour.

## Conformance

A capability becomes supported only after its declared behaviour has been
verified against native protocol implementations or authoritative protocol
interfaces.

Conformance MUST test the actual direction and semantics being claimed.

Where applicable, this includes:

- outbound operation mapping;
- inbound operation mapping;
- message content preservation;
- reply relationships;
- reaction semantics;
- attachment handling;
- mutation behaviour;
- addressing;
- discovery;
- ordering;
- duplicate behaviour;
- disconnect and reconnect behaviour;
- malformed input;
- native rejection;
- partial failure;
- security-relevant failure paths.

Mock-to-mock success alone is insufficient to establish interoperability with a
native protocol.

## Non-goals

Idalion does not aim to:

- support centralized, service-controlled, or federated messaging platforms
  under the current contract;
- replace native P2P messaging protocols;
- create a new universal messaging protocol;
- normalize every protocol feature into one lowest-common-denominator API;
- guarantee complete feature equivalence;
- hide meaningful infrastructure differences;
- create a universal identity system;
- provide a mandatory central gateway;
- become a generic network abstraction;
- bridge arbitrary event systems or application APIs;
- silently approximate unsupported semantics.

## Evolution rule

Idalion grows from verified P2P messaging interoperability, not speculative
generalization.

New abstractions SHOULD be introduced only when multiple real P2P adapters
demonstrate that the abstraction is genuinely shared.

The project MUST remain focused on peer-to-peer conversational messaging
interoperability.

Centralized messaging interoperability MUST NOT be reintroduced merely because
a platform exposes an API. Any future scope expansion requires a supported,
bidirectional, protocol-native integration model that satisfies Idalion's
architectural constraints and a concrete application requirement that justifies
expanding the contract.

If a useful abstraction is discovered outside the P2P conversational domain, it
should be evaluated separately rather than expanding Idalion into a generic
integration framework.