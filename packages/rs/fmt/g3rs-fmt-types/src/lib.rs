//! Shared types for the g3rs fmt family.

#[cfg(feature = "api")]
mod types;

#[cfg(feature = "api")]
pub use types::G3RsFmtCargoState;
#[cfg(feature = "api")]
pub use types::G3RsFmtConfigChecksInput;
#[cfg(feature = "api")]
pub use types::G3RsFmtConfigFileKind;
#[cfg(feature = "api")]
pub use types::G3RsFmtFileTreeChecksInput;
#[cfg(feature = "api")]
pub use types::G3RsFmtNestedConfigFile;
#[cfg(feature = "api")]
pub use types::G3RsFmtRustPolicyState;
#[cfg(feature = "api")]
pub use types::G3RsFmtRustfmtConfigState;
#[cfg(feature = "api")]
pub use types::G3RsFmtSourceChecksInput;
#[cfg(feature = "api")]
pub use types::G3RsFmtToolchainState;
