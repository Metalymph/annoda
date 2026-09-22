#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Transport-agnostic primitives and adapter SPI for peer-to-peer messaging.
//!
//! Annoda deliberately does not prescribe a concrete P2P stack. Adapters own
//! the translation between this small contract and a concrete implementation.

use std::{error::Error, fmt};

/// Stable identity of a remote or local peer as understood by an adapter.
///
/// Annoda treats peer identifiers as opaque bytes. Their concrete encoding and
/// cryptographic meaning belong to the adapter and its underlying stack.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PeerId(Box<[u8]>);

impl PeerId {
    /// Creates a peer identifier from non-empty opaque bytes.
    pub fn new(bytes: impl Into<Box<[u8]>>) -> Result<Self, PeerIdError> {
        let bytes = bytes.into();

        if bytes.is_empty() {
            return Err(PeerIdError);
        }

        Ok(Self(bytes))
    }

    /// Returns the opaque peer identifier bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// Error returned when a peer identifier is empty.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PeerIdError;

impl fmt::Display for PeerIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("peer identifiers must not be empty")
    }
}

impl Error for PeerIdError {}

/// Capabilities explicitly provided by an adapter.
///
/// Capability discovery prevents Annoda from pretending that every underlying
/// P2P stack has identical semantics.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AdapterCapabilities {
    /// The adapter can establish direct peer-to-peer connections.
    pub direct_connections: bool,
    /// The adapter can fall back to relayed connectivity.
    pub relayed_connections: bool,
    /// The adapter can discover peers without an already-known endpoint.
    pub peer_discovery: bool,
    /// Delivered frames are reliable.
    pub reliable_delivery: bool,
    /// Delivered frames preserve send order.
    pub ordered_delivery: bool,
}

/// An event emitted by an [`Adapter`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AdapterEvent {
    /// Connectivity to a peer became usable.
    Connected {
        /// The peer that became connected.
        peer: PeerId,
    },
    /// Connectivity to a peer ended.
    Disconnected {
        /// The peer that disconnected.
        peer: PeerId,
    },
    /// A framed application payload arrived from a peer.
    Frame {
        /// The peer that sent the payload.
        peer: PeerId,
        /// Opaque application bytes.
        payload: Box<[u8]>,
    },
}

/// Minimal SPI implemented by concrete P2P adapters.
///
/// The polling model is intentional: an adapter may internally use Tokio,
/// JavaScript/Bare, native threads, callbacks, or another runtime while Annoda
/// itself remains runtime-agnostic and FFI-friendly.
pub trait Adapter {
    /// Adapter-specific error type.
    type Error: Error + Send + Sync + 'static;

    /// Returns a stable, human-readable adapter name.
    fn name(&self) -> &'static str;

    /// Returns the local peer identity exposed by this adapter.
    fn local_peer_id(&self) -> &PeerId;

    /// Returns the capabilities supported by this adapter.
    fn capabilities(&self) -> AdapterCapabilities;

    /// Requests connectivity to `peer`.
    fn connect(&mut self, peer: &PeerId) -> Result<(), Self::Error>;

    /// Sends one opaque application frame to `peer`.
    fn send(&mut self, peer: &PeerId, payload: &[u8]) -> Result<(), Self::Error>;

    /// Polls the next adapter event without blocking.
    fn poll_event(&mut self) -> Result<Option<AdapterEvent>, Self::Error>;

    /// Requests disconnection from `peer`.
    fn disconnect(&mut self, peer: &PeerId) -> Result<(), Self::Error>;
}

#[cfg(test)]
mod tests {
    use super::PeerId;

    #[test]
    fn peer_id_rejects_empty_bytes() {
        assert!(PeerId::new(Vec::<u8>::new()).is_err());
    }

    #[test]
    fn peer_id_preserves_bytes() {
        let peer = PeerId::new(vec![1_u8, 2, 3]).expect("valid peer id");
        assert_eq!(peer.as_bytes(), &[1, 2, 3]);
    }
}
