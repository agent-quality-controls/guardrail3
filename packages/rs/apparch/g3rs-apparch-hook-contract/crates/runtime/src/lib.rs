//! Runtime crate that exposes the apparch hook contract data.

/// Embedded hook contract data and its accessor.
mod contract;

#[cfg(feature = "api")]
pub use contract::hook_contract;
