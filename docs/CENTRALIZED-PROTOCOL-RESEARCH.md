# Centralized messaging protocol research

## Status

**Research complete. Centralized messaging platforms are outside Idalion's
current scope.**

This document records the investigation that led Idalion to narrow its scope to
peer-to-peer conversational protocols.

It is a decision record, not a claim that interoperability with centralized
messaging platforms is universally or permanently impossible.

The conclusions below describe the public APIs, platform models, and policies
evaluated in September 2026. They should be revalidated before being relied on
in the future.

## Original objective

Idalion originally investigated interoperability between arbitrary
conversational messaging protocols, including both peer-to-peer and
service-controlled systems.

The important requirement was stronger than merely being able to send data to
another platform.

Idalion was intended to be present on only one side of a conversation.

Conceptually:

    Application A
        |
     Idalion
        |
     protocol boundary
        |
        v
    Native client B

The user on endpoint B should continue using the native, unmodified client for
their protocol.

Endpoint B must not need:

- Idalion;
- the embedding application used by endpoint A;
- a synthetic Idalion account;
- an Idalion-operated messaging gateway;
- a replacement client;
- a second identity created solely for interoperability.

This requirement is referred to in this document as **single-sided
interoperability**.

## Acceptance criteria

A centralized protocol would have been suitable only if its supported public
integration model could provide all of the following properties.

1. **Native addressing or discovery**

   Endpoint A can identify or address endpoint B using a protocol-native
   identity or destination.

2. **Native identity for A**

   Messages sent through Idalion are represented by an identity that endpoint B
   can legitimately address using the native protocol.

3. **Conversation initiation**

   Endpoint A can initiate a conversation with an eligible endpoint B rather
   than only responding to conversations initiated elsewhere.

4. **Native reply path**

   Endpoint B can reply using the official or otherwise normal native client.

5. **Return delivery**

   The reply can return to endpoint A through the supported integration
   mechanism.

6. **Single-sided deployment**

   Endpoint B does not need Idalion or an Idalion-aware client.

7. **No synthetic gateway identity**

   Idalion does not manufacture a central identity, telephone number, account,
   proxy user, or equivalent gateway merely to make the protocols appear
   interoperable.

8. **Supported platform use**

   The architecture is compatible with the platform's documented APIs and
   published policies. Idalion must not depend on impersonation, unsupported
   automation, private APIs, policy ambiguity, or behaviour likely to be
   withdrawn as abuse.

Failure of a fundamental criterion is sufficient to reject the protocol for the
centralized interoperability scope.

## Why discoverability alone is insufficient

The investigation initially focused heavily on discoverability.

That is necessary, but it is not sufficient.

Suppose A can discover B and send a message. A usable conversation still
requires B to possess a native destination representing A.

The complete requirement is therefore bidirectional addressability:

    A + Idalion  ---------->  B
    A + Idalion  <----------  B

Both directions must remain valid within the destination protocol's supported
identity and messaging model.

A system that can deliver the first message but cannot provide a legitimate
native return path is not conversationally interoperable.

This distinction eliminated several apparently promising integrations.

## Telegram

### Result

**Technically viable through the Telegram Client API, but incompatible with
Idalion's intended integration scope under the platform terms evaluated in
September 2026.**

Telegram was the strongest centralized candidate and therefore became the
decisive feasibility test.

Three distinct Telegram integration models were considered.

### Bot API

The Bot API provides a supported and powerful automation surface, but a bot is
a distinct Telegram identity rather than the Telegram user represented by the
embedding application.

Bots also operate under Telegram's bot-specific conversation model.

This does not satisfy Idalion's requirement for a general user-to-user
single-sided interoperability path.

**Verdict: incompatible identity and conversation model.**

### Telegram Business connected bots

Telegram Business allows an account to connect a bot that can process selected
private conversations and perform supported actions on behalf of the business
connection.

This solves an important part of the return path: eligible incoming Telegram
messages can be exposed to the connected integration and replies can be sent
through the business connection.

However, the model is not an unrestricted user-level messaging session.

In particular, business messaging rights are constrained by Telegram's
business conversation eligibility and recent-contact rules. It therefore
cannot be treated as a general mechanism by which an Idalion endpoint discovers
an arbitrary Telegram user and initiates a new conversation with that user.

**Verdict: useful conversation-management API, but insufficient for
general-purpose single-sided interoperability.**

### Telegram Client API / MTProto

The Telegram Client API is fundamentally different.

An authenticated user session can perform native Telegram operations as that
Telegram user. Technically this provides the primitives Idalion requires:

    Application A
        |
     Idalion
        |
    Telegram user session
        |
        +----------> Telegram user B
        |
        <----------+

It can provide native identity, conversation initiation, native replies, and
return delivery without requiring Idalion on endpoint B.

From a purely technical perspective, this was the first centralized mechanism
found that satisfied the core bidirectional model.

The blocker is product and policy scope rather than protocol mechanics.

Telegram publishes the Client API as a facility for building third-party
Telegram clients and its API terms impose requirements on such clients,
including correct support for the basic functionality expected of Telegram
applications and Telegram-specific behaviour.

Idalion's intended embedding model is deliberately narrower.

An application using Idalion should not be required to become a general
Telegram client merely because it wants to interoperate with Telegram
conversations.

For example, an application whose primary messaging system is Hyperswarm should
not need to reproduce Telegram's broader client surface, platform behaviours,
and evolving Telegram-specific requirements in order to expose a small
interoperability subset.

Depending on an optimistic interpretation of those requirements would make a
core interoperability feature vulnerable to policy enforcement or later
clarification.

**Verdict: technically viable, but rejected because the supported platform
contract does not match Idalion's intended partial-integration model.**

### Telegram conclusion

Telegram is deliberately **not** classified as technically impossible.

The distinction matters:

    Bot API                    incompatible model
    Business connected bots    insufficient initiation model
    Client API / MTProto       technically suitable
    Client API policy/scope     unsuitable for Idalion's intended product model

If Telegram's supported integration model or terms materially change, this
decision is worth revisiting.

## WhatsApp

### Result

**Incompatible with Idalion's required single-sided user-to-user model under
the supported integration surfaces evaluated in September 2026.**

The important problem is not whether an Idalion user can learn another person's
telephone number.

Out-of-band exchange of an address is acceptable.

The problem is the reverse conversational identity.

For a native WhatsApp user B to participate normally, B must receive a message
from a legitimate WhatsApp identity representing A and must be able to reply to
that identity through WhatsApp.

An architecture such as:

    A learns B's WhatsApp number

    A + Idalion ----------------> B

is incomplete unless this also exists:

    A + Idalion <---------------- B

with a supported WhatsApp-native identity for A.

Introducing an Idalion-owned telephone number, business account, proxy identity,
or central relay would change the architecture into a gateway service. That is
explicitly outside Idalion's model.

WhatsApp's supported business integration surfaces do not turn an arbitrary
external P2P identity into a normal WhatsApp user identity.

**Verdict: incompatible identity/addressability model for Idalion.**

## Signal

### Result

**Incompatible with the supported integration model required by Idalion.**

Signal publishes substantial protocol and client implementation material, but
open implementation availability is not equivalent to a supported public
user-level interoperability API.

Idalion would require a supported mechanism through which an external
application identity could participate in normal Signal conversations while the
remote user remained on an unmodified Signal client.

No such supported general-purpose integration surface was established during
this research.

Building around unofficial automation, emulating a Signal client, or operating
an Idalion-controlled Signal identity would violate the architectural
constraints of this investigation.

**Verdict: no supported single-sided integration model established.**

## Messenger

### Result

**Incompatible with Idalion's intended user-to-user interoperability model.**

Meta's supported Messenger integration surfaces are oriented around the
platform's supported business, Page, application, and messaging models rather
than providing arbitrary external applications with a transparent
user-session-level bridge between unrelated messaging identities.

A Page, business identity, application identity, or Idalion-operated proxy
would not represent the external application's user as a normal native
Messenger peer.

That would produce gateway or business messaging integration rather than the
single-sided user-to-user interoperability Idalion was investigating.

**Verdict: incompatible platform identity/integration model.**

## Discord

### Result

**Not pursued after the Telegram gate failed.**

Discord was initially considered because it has strong account addressing,
direct messaging, OAuth, bot infrastructure, and a large user base.

However, Discord's supported automation model distinguishes bots/applications
from normal user accounts, while automation of ordinary user accounts is not a
supported replacement for that model.

A bot identity would not satisfy Idalion's intended transparent user-to-user
identity model.

More importantly, Telegram was intentionally selected as the decisive
centralized feasibility gate because it offered a substantially stronger
candidate architecture.

Once Telegram failed the product/policy requirement, proving a narrower Discord
integration could not justify retaining centralized platforms in Idalion's
scope.

**Verdict: research intentionally stopped; insufficient strategic value to
change the centralized-scope decision.**

This is not a claim that every conceivable Discord integration is technically
impossible.

## Matrix

### Result

**Not pursued; outside the resulting product scope.**

Matrix differs substantially from the proprietary centralized platforms above.
It is an open, federated messaging ecosystem and has architectural properties
that make interoperability research considerably more plausible.

It was nevertheless not a useful deciding case for Idalion.

The original motivation for centralized interoperability was to reach users of
widely deployed messaging services without requiring those users to adopt the
embedding application's P2P client.

Matrix did not provide enough product value for the intended applications to
justify retaining a broad server-mediated/federated scope after the Telegram
gate failed.

Matrix should therefore not be classified as technically incompatible.

**Verdict: technically interesting, but deliberately not pursued.**

## Rejected gateway alternatives

Several designs could superficially make centralized interoperability appear
possible:

- an Idalion-operated telephone number;
- Idalion-owned accounts on destination services;
- business identities acting as shared proxies;
- protocol-specific bridge servers;
- impersonated or automated user accounts;
- replacement clients installed by the remote participant;
- requiring Idalion on both endpoints.

These designs solve a different problem.

They introduce an Idalion messaging authority, synthetic identity, mandatory
gateway, unsupported automation, or second-sided deployment.

Idalion deliberately rejects those approaches.

The project should not claim protocol interoperability by hiding a centralized
bridge behind the adapter interface.

## Decision

The centralized messaging investigation is closed for the current project
scope.

Idalion is narrowed to:

> **Semantic interoperability between peer-to-peer conversational messaging
> protocols.**

This is a product and architecture decision, not merely an implementation
shortcut.

The P2P domain provides a substantially cleaner fit for Idalion's principles:

- protocol-native endpoint identities;
- explicit addressing and discovery mechanisms;
- adapter-owned native infrastructure;
- no requirement for an Idalion identity authority;
- no dependency on third-party centralized-platform product policy for the
  basic existence of an interoperability path;
- the possibility of validating semantics against multiple independently
  implemented P2P stacks.

Potential future adapters include Hyperswarm/Holepunch, Iroh, libp2p, and other
P2P conversational stacks where real application requirements justify them.

## Revisit conditions

Centralized interoperability should not be periodically re-investigated without
a concrete reason.

This decision should be reopened only if at least one of the following occurs:

- a major messaging platform introduces a supported user-level interoperability
  API;
- Telegram materially changes the Client API terms or introduces an integration
  model suitable for partial embedded messaging;
- a standardized interoperability mechanism gains meaningful deployment across
  major messaging platforms;
- an Idalion consumer develops a concrete requirement that changes the
  cost/benefit analysis;
- a previously rejected platform changes its identity or bidirectional
  addressing model materially.

Any future investigation must re-evaluate both technical capability and current
platform policy.

## Project consequence

The centralized scope was the primary reason to pursue Idalion aggressively as
an independent project.

With that scope removed, Idalion remains architecturally useful for P2P-to-P2P
interoperability but is no longer an immediate development priority.

The repository remains public and the P2P contract remains available for future
work.

Active implementation is paused until a concrete P2P interoperability need
justifies continuing development.