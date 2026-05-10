//! Tooling contracts shared across rust families.

/// Rule implementation for `contract`.
mod contract;

#[cfg(feature = "api")]
pub use contract::{ToolingFamily, hook_contract};
