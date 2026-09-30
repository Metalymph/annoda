#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Semantic interoperability primitives for peer-to-peer conversational messaging protocols.
//!
//! Idalion models conversational semantics rather than transport frames. Concrete
//! adapters translate this contract to native messaging protocols.

use std::{error::Error, fmt};

/// Opaque identity of a messaging endpoint as understood by an adapter.
///
/// Depending on the native protocol, an endpoint may represent a peer, account,
/// user, device, conversation, group, channel, or another native destination.
/// Idalion does not interpret the identifier bytes.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EndpointId(Box<[u8]>);

impl EndpointId {
    /// Creates an endpoint identifier from non-empty opaque bytes.
    pub fn new(bytes: impl Into<Box<[u8]>>) -> Result<Self, EndpointIdError> {
        let bytes = bytes.into();

        if bytes.is_empty() {
            return Err(EndpointIdError);
        }

        Ok(Self(bytes))
    }

    /// Returns the opaque endpoint identifier bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// Error returned when an endpoint identifier is empty.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EndpointIdError;

impl fmt::Display for EndpointIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("endpoint identifiers must not be empty")
    }
}

impl Error for EndpointIdError {}

/// Conversational capability understood by the current Idalion contract.
///
/// Capabilities are added only when real adapters demonstrate a shared semantic
/// operation. The initial contract intentionally starts with text messaging.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum Capability {
    /// Sending and receiving textual conversational content.
    Text,
}

/// Support declared by an adapter for one capability in one boundary direction.
///
/// Constraint-bearing support will be introduced when concrete native adapters
/// establish the first constraints that Idalion must represent.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum CapabilitySupport {
    /// The capability is supported without a currently modelled constraint.
    Supported,
    /// The capability is not supported.
    Unsupported,
}

/// Direction of a capability relative to the adapter/application boundary.
///
/// This is deliberately not a protocol-A-to-protocol-B direction. Cross-protocol
/// interoperability is derived by composing the relevant adapter capabilities.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AdapterDirection {
    /// Operations emitted by the application toward the native protocol.
    Outbound,
    /// Operations observed from the native protocol by the application.
    Inbound,
}

/// UTF-8 textual conversational content.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextContent(Box<str>);

impl TextContent {
    /// Creates textual content.
    ///
    /// Empty text is currently rejected because the initial semantic contract has
    /// no native-protocol evidence requiring an empty text operation.
    pub fn new(text: impl Into<Box<str>>) -> Result<Self, TextContentError> {
        let text = text.into();

        if text.is_empty() {
            return Err(TextContentError);
        }

        Ok(Self(text))
    }

    /// Returns the textual content.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Error returned when textual content is empty.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TextContentError;

impl fmt::Display for TextContentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("text content must not be empty")
    }
}

impl Error for TextContentError {}

/// Immediate acceptance of an outbound conversational operation.
///
/// Acceptance means only that the adapter accepted responsibility for the
/// operation. It does not imply remote delivery, persistence, or read state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum OperationAcceptance {
    /// The adapter accepted the operation for native processing.
    Accepted,
}

/// Event emitted by an [`Adapter`] from its native protocol boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum AdapterEvent {
    /// Textual conversational content was received from a native endpoint.
    TextReceived {
        /// Native source endpoint as represented by this adapter.
        source: EndpointId,
        /// Received textual content.
        content: TextContent,
    },
}

/// Minimal protocol adapter boundary for conversational interoperability.
///
/// The SPI is intentionally runtime-agnostic. An implementation may internally
/// use async runtimes, native event loops, callbacks, worker threads, service
/// SDKs, JavaScript/Bare, or another protocol-appropriate execution model.
pub trait Adapter {
    /// Adapter-specific operational error.
    type Error: Error + Send + Sync + 'static;

    /// Returns a stable human-readable adapter name.
    fn name(&self) -> &'static str;

    /// Reports support for a conversational capability in one adapter direction.
    fn capability(&self, capability: Capability, direction: AdapterDirection) -> CapabilitySupport;

    /// Requests transmission of textual conversational content to an endpoint.
    ///
    /// A successful return reports immediate adapter acceptance only. It does not
    /// imply delivery or read state.
    fn send_text(
        &mut self,
        destination: &EndpointId,
        content: &TextContent,
    ) -> Result<OperationAcceptance, Self::Error>;

    /// Polls the next native conversational event without blocking.
    fn poll_event(&mut self) -> Result<Option<AdapterEvent>, Self::Error>;
}

#[cfg(test)]
mod tests {
    use super::{EndpointId, TextContent};

    #[test]
    fn endpoint_id_rejects_empty_bytes() {
        assert!(EndpointId::new(Vec::<u8>::new()).is_err());
    }

    #[test]
    fn endpoint_id_preserves_opaque_bytes() {
        let endpoint = EndpointId::new(vec![1_u8, 2, 3]).expect("valid endpoint id");
        assert_eq!(endpoint.as_bytes(), &[1, 2, 3]);
    }

    #[test]
    fn text_content_rejects_empty_text() {
        assert!(TextContent::new("").is_err());
    }

    #[test]
    fn text_content_preserves_utf8() {
        let content = TextContent::new("ciao 👋").expect("valid text");
        assert_eq!(content.as_str(), "ciao 👋");
    }
}
