#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Shared conformance helpers for Idalion adapter implementations.

use annoda::{Adapter, AdapterDirection, Capability, CapabilitySupport};

/// Verifies static invariants every adapter must satisfy.
///
/// The testkit remains intentionally small while real native adapters establish
/// which interoperability behaviours are genuinely portable.
pub fn assert_static_contract<A: Adapter>(adapter: &A) {
    assert!(
        !adapter.name().trim().is_empty(),
        "adapter name must not be empty"
    );
}

/// Verifies that an adapter declares text support in both boundary directions.
///
/// This helper is intended only for adapters that explicitly claim symmetric
/// text support. Idalion itself does not require such symmetry.
pub fn assert_bidirectional_text_support<A: Adapter>(adapter: &A) {
    assert_eq!(
        adapter.capability(Capability::Text, AdapterDirection::Outbound),
        CapabilitySupport::Supported,
        "adapter must support outbound text"
    );
    assert_eq!(
        adapter.capability(Capability::Text, AdapterDirection::Inbound),
        CapabilitySupport::Supported,
        "adapter must support inbound text"
    );
}
