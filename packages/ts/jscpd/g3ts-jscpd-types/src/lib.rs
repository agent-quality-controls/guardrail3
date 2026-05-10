//! Shared types and conversions for g3ts .jscpd family hooks.

/// Conversion logic from parsed jscpd JSON into normalized snapshots.
#[cfg(feature = "api")]
mod convert;

/// Public input/state/snapshot types for g3ts .jscpd hooks.
#[cfg(feature = "api")]
mod types;

#[cfg(feature = "api")]
pub use convert::root_snapshot;
#[cfg(feature = "api")]
pub use types::G3TsJscpdChecksInput;
#[cfg(feature = "api")]
pub use types::G3TsJscpdRootSnapshot;
#[cfg(feature = "api")]
pub use types::G3TsJscpdRootState;
