# idalion

Core Rust contract for Idalion conversational messaging interoperability.

The crate provides protocol-neutral endpoint, capability, operation-acceptance, event, and adapter primitives.

It deliberately does not provide a universal wire protocol, transport, identity system, message store, relay network, or generic networking abstraction.

Protocol-specific implementations belong in adapter crates.

The public SPI is experimental during the 0.0.x series and evolves from requirements demonstrated by real native adapters.

See the repository `docs/CONTRACT.md` and `docs/ARCHITECTURE.md` for the normative interoperability model.
