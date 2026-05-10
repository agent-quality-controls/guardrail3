//! Architecture contracts shared across rust families.

/// Rule implementation for `contract`.
mod contract;

#[cfg(feature = "api")]
pub use contract::{ArchitectureFamily, hook_contract};
