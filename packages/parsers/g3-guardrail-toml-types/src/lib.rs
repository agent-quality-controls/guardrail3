//! Shared TOML policy types for guardrail3 parser crates.

/// Shared waiver schema and exact matching helpers.
mod waiver;

#[cfg(feature = "api")]
pub use waiver::{WaiverConfig, WaiverMatch, WaiverReason, find_waiver_reason, has_waiver};
