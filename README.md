# Annoda

Annoda is a transport-agnostic contract for peer-to-peer messaging.

It owns the application-facing messaging/peer contract, not the underlying
network stack. Concrete adapters translate Annoda's deliberately small SPI to
stacks such as Hyperswarm/Pear, Iroh, libp2p, or other suitable P2P engines.

## Status

**0.0.x — experimental foundation.** The public surface is intentionally small
and may change while the first real adapters validate the design.

## Design rule

```text
Application
    |
    v
Annoda contract
    |
    v
Adapter SPI
    |
    +-- Hyperswarm / Pear
    +-- Iroh
    +-- libp2p
    +-- ...
```

Annoda is **not** a cross-protocol gateway. Two peers still need at least one
mutually compatible wire stack/protocol. Annoda standardizes the application
surface and adapter contract.

See `docs/ARCHITECTURE.md` and `docs/ADAPTERS.md`.

## License

MIT
