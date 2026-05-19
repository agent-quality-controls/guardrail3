//! Shared types for the g3rs code family.

/// Internal type definitions exposed via the facade re-exports.
#[cfg(feature = "api")]
mod types;

#[cfg(feature = "api")]
pub use types::G3RsCodeConfigChecksInput;
#[cfg(feature = "api")]
pub use types::G3RsCodeConfigFile;
#[cfg(feature = "api")]
pub use types::G3RsCodeConfigFileKind;
#[cfg(feature = "api")]
pub use types::G3RsCodeExceptionComment;
#[cfg(feature = "api")]
pub use types::G3RsCodeFileTreeChecksInput;
#[cfg(feature = "api")]
pub use types::G3RsCodeSourceChecksInput;
#[cfg(feature = "api")]
pub use types::G3RsCodeStructuralCapRoot;
#[cfg(feature = "api")]
pub use types::G3RsSourceFile;
