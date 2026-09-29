#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! In-memory semantic reference adapter for the Idalion SPI.

use annoda::{
    Adapter, AdapterDirection, AdapterEvent, Capability, CapabilitySupport, EndpointId,
    OperationAcceptance, TextContent,
};
use std::{
    error::Error,
    fmt,
    sync::mpsc::{self, Receiver, Sender, TryRecvError},
};

/// In-memory adapter paired with one destination endpoint.
///
/// The adapter models semantic text exchange only. It deliberately has no
/// connection lifecycle because connection semantics are not universal to
/// Idalion adapters.
pub struct LoopbackAdapter {
    local: EndpointId,
    remote: EndpointId,
    sender: Sender<AdapterEvent>,
    receiver: Receiver<AdapterEvent>,
}

impl LoopbackAdapter {
    /// Creates a pair of in-memory semantic endpoints.
    pub fn pair(first: EndpointId, second: EndpointId) -> (LoopbackAdapter, LoopbackAdapter) {
        let (first_tx, first_rx) = mpsc::channel();
        let (second_tx, second_rx) = mpsc::channel();

        let first_adapter = LoopbackAdapter {
            local: first.clone(),
            remote: second.clone(),
            sender: second_tx,
            receiver: first_rx,
        };

        let second_adapter = LoopbackAdapter {
            local: second,
            remote: first,
            sender: first_tx,
            receiver: second_rx,
        };

        (first_adapter, second_adapter)
    }

    fn ensure_destination(&self, endpoint: &EndpointId) -> Result<(), LoopbackError> {
        if endpoint == &self.remote {
            Ok(())
        } else {
            Err(LoopbackError::UnknownEndpoint)
        }
    }
}

impl Adapter for LoopbackAdapter {
    type Error = LoopbackError;

    fn name(&self) -> &'static str {
        "loopback"
    }

    fn capability(
        &self,
        capability: Capability,
        _direction: AdapterDirection,
    ) -> CapabilitySupport {
        match capability {
            Capability::Text => CapabilitySupport::Supported,
            _ => CapabilitySupport::Unsupported,
        }
    }

    fn send_text(
        &mut self,
        destination: &EndpointId,
        content: &TextContent,
    ) -> Result<OperationAcceptance, Self::Error> {
        self.ensure_destination(destination)?;

        self.sender
            .send(AdapterEvent::TextReceived {
                source: self.local.clone(),
                content: content.clone(),
            })
            .map_err(|_| LoopbackError::RemoteClosed)?;

        Ok(OperationAcceptance::Accepted)
    }

    fn poll_event(&mut self) -> Result<Option<AdapterEvent>, Self::Error> {
        match self.receiver.try_recv() {
            Ok(event) => Ok(Some(event)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => Err(LoopbackError::RemoteClosed),
        }
    }
}

/// Errors produced by [`LoopbackAdapter`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LoopbackError {
    /// The requested destination is not paired with this adapter.
    UnknownEndpoint,
    /// The paired in-memory endpoint has been dropped.
    RemoteClosed,
}

impl fmt::Display for LoopbackError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::UnknownEndpoint => "unknown loopback endpoint",
            Self::RemoteClosed => "loopback remote endpoint has closed",
        };

        formatter.write_str(message)
    }
}

impl Error for LoopbackError {}
