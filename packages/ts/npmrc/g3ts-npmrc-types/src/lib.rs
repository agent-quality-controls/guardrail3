/// Conversion helpers from raw npmrc parser output into family snapshots.
#[cfg(feature = "api")]
mod convert;

/// Public type definitions for the npmrc family contract.
#[cfg(feature = "api")]
mod types;

#[cfg(feature = "api")]
pub use convert::root_snapshot;
#[cfg(feature = "api")]
pub use types::G3TsNpmrcChecksInput;
#[cfg(feature = "api")]
pub use types::G3TsNpmrcRootSnapshot;
#[cfg(feature = "api")]
pub use types::G3TsNpmrcRootState;
#[cfg(feature = "api")]
pub use types::G3TsNpmrcSetting;
