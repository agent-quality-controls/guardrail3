//! Shared types for the g3rs clippy family.

/// Concrete data structures shared across clippy family crates.
#[cfg(feature = "api")]
mod types;

#[cfg(feature = "api")]
pub use types::G3RsClippyCargoConfigState;
#[cfg(feature = "api")]
pub use types::G3RsClippyCargoMemberState;
#[cfg(feature = "api")]
pub use types::G3RsClippyCargoRootState;
#[cfg(feature = "api")]
pub use types::G3RsClippyConfigChecksInput;
#[cfg(feature = "api")]
pub use types::G3RsClippyConfigState;
#[cfg(feature = "api")]
pub use types::G3RsClippyFileTreeChecksInput;
#[cfg(feature = "api")]
pub use types::G3RsClippyRustPolicyState;
#[cfg(feature = "api")]
pub use types::G3RsClippyShadowedConfig;
