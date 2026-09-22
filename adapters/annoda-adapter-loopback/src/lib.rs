#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! In-memory reference adapter used to validate Annoda's SPI.

use annoda::{Adapter, AdapterCapabilities, AdapterEvent, PeerId};
use std::{
    collections::VecDeque,
    error::Error,
    fmt,
    sync::mpsc::{self, Receiver, Sender, TryRecvError},
};

/// An in-memory adapter connected to exactly one peer.
///
/// Use [`LoopbackAdapter::pair`] to create two compatible endpoints.
pub struct LoopbackAdapter {
    local: PeerId,
    remote: PeerId,
    sender: Sender<AdapterEvent>,
    receiver: Receiver<AdapterEvent>,
    local_events: VecDeque<AdapterEvent>,
    connected: bool,
}

impl LoopbackAdapter {
    /// Creates a connected pair of in-memory adapter endpoints.
    pub fn pair(first: PeerId, second: PeerId) -> (LoopbackAdapter, LoopbackAdapter) {
        let (first_tx, first_rx) = mpsc::channel();
        let (second_tx, second_rx) = mpsc::channel();

        let first_adapter = LoopbackAdapter {
            local: first.clone(),
            remote: second.clone(),
            sender: second_tx,
            receiver: first_rx,
            local_events: VecDeque::new(),
            connected: false,
        };

        let second_adapter = LoopbackAdapter {
            local: second,
            remote: first,
            sender: first_tx,
            receiver: second_rx,
            local_events: VecDeque::new(),
            connected: false,
        };

        (first_adapter, second_adapter)
    }

    fn ensure_remote(&self, peer: &PeerId) -> Result<(), LoopbackError> {
        if peer == &self.remote {
            Ok(())
        } else {
            Err(LoopbackError::UnknownPeer)
        }
    }
}

impl Adapter for LoopbackAdapter {
    type Error = LoopbackError;

    fn name(&self) -> &'static str {
        "loopback"
    }

    fn local_peer_id(&self) -> &PeerId {
        &self.local
    }

    fn capabilities(&self) -> AdapterCapabilities {
        AdapterCapabilities {
            direct_connections: true,
            relayed_connections: false,
            peer_discovery: false,
            reliable_delivery: true,
            ordered_delivery: true,
        }
    }

    fn connect(&mut self, peer: &PeerId) -> Result<(), Self::Error> {
        self.ensure_remote(peer)?;

        if !self.connected {
            self.connected = true;
            self.local_events.push_back(AdapterEvent::Connected {
                peer: self.remote.clone(),
            });
            self.sender
                .send(AdapterEvent::Connected {
                    peer: self.local.clone(),
                })
                .map_err(|_| LoopbackError::PeerClosed)?;
        }

        Ok(())
    }

    fn send(&mut self, peer: &PeerId, payload: &[u8]) -> Result<(), Self::Error> {
        self.ensure_remote(peer)?;

        if !self.connected {
            return Err(LoopbackError::NotConnected);
        }

        self.sender
            .send(AdapterEvent::Frame {
                peer: self.local.clone(),
                payload: payload.into(),
            })
            .map_err(|_| LoopbackError::PeerClosed)
    }

    fn poll_event(&mut self) -> Result<Option<AdapterEvent>, Self::Error> {
        if let Some(event) = self.local_events.pop_front() {
            return Ok(Some(event));
        }

        match self.receiver.try_recv() {
            Ok(event) => Ok(Some(event)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => Err(LoopbackError::PeerClosed),
        }
    }

    fn disconnect(&mut self, peer: &PeerId) -> Result<(), Self::Error> {
        self.ensure_remote(peer)?;

        if self.connected {
            self.connected = false;
            self.local_events.push_back(AdapterEvent::Disconnected {
                peer: self.remote.clone(),
            });
            self.sender
                .send(AdapterEvent::Disconnected {
                    peer: self.local.clone(),
                })
                .map_err(|_| LoopbackError::PeerClosed)?;
        }

        Ok(())
    }
}

/// Errors produced by [`LoopbackAdapter`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LoopbackError {
    /// The requested peer is not the endpoint paired with this adapter.
    UnknownPeer,
    /// A send was attempted before connectivity was established.
    NotConnected,
    /// The paired endpoint has been dropped.
    PeerClosed,
}

impl fmt::Display for LoopbackError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::UnknownPeer => "unknown loopback peer",
            Self::NotConnected => "loopback peer is not connected",
            Self::PeerClosed => "loopback peer has closed",
        };

        formatter.write_str(message)
    }
}

impl Error for LoopbackError {}
