# Architecture

## Purpose

Annoda standardizes the application-facing contract used by P2P messaging
clients while leaving transport, discovery, NAT traversal, relay, runtime, and
wire-protocol decisions to adapters.

```text
Application / client
        |
        v
Annoda public contract
        |
        v
Adapter SPI
        |
        +-- Hyperswarm / Pear / Bare
        +-- Iroh
        +-- libp2p
        +-- other suitable stacks
```

## Non-goals

Annoda is not:

- a universal wire protocol;
- a gateway translating unrelated P2P stacks;
- a new DHT;
- a NAT traversal implementation;
- a relay network;
- an E2EE protocol;
- a replacement for the networking stacks used by adapters.

Two communicating peers still need at least one mutually compatible underlying
wire stack/protocol.

## Core rule

**Annoda owns the contract, not the network.**

No adapter-specific concept should leak into the public application surface
unless multiple real adapters demonstrate that the concept is genuinely
portable.

## Runtime model

The first SPI is polling-based and synchronous at its boundary. This does not
require synchronous networking. An adapter may internally use Tokio, Bare,
native event loops, callbacks, threads, or another runtime and expose events
through Annoda's small polling surface.

This shape is intentionally friendly to C ABI, Swift wrappers, React Native
native modules, and future WASM bindings.

## Capability honesty

Adapters advertise explicit capabilities. Annoda must not flatten meaningful
differences between stacks merely to make the API look uniform.

## Evolution rule

The 0.0.x series exists to validate the contract against real, architecturally
different adapters before declaring the SPI stable.
