//! Shared waiver schema and result application for guardrail3.

#[cfg(feature = "api")]
/// Shared waiver matching and application primitives.
mod waiver;

#[cfg(feature = "api")]
pub use waiver::{WaiverConfig, WaiverKey, WaiverReason, apply_waivers, find_waiver_reason};
