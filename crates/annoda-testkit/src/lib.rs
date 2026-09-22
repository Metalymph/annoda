#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Shared conformance helpers for Annoda adapter implementations.

use annoda::Adapter;

/// Verifies static invariants every adapter must satisfy.
///
/// This is intentionally small in 0.0.x. The testkit will grow only when real
/// adapters prove which behaviours are genuinely portable across stacks.
pub fn assert_static_contract<A: Adapter>(adapter: &A) {
    assert!(
        !adapter.name().trim().is_empty(),
        "adapter name must not be empty"
    );
    assert!(
        !adapter.local_peer_id().as_bytes().is_empty(),
        "local peer id must not be empty"
    );
}
