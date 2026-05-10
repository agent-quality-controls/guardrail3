//! Shared types for the g3rs deps family.

/// Concrete data structures shared across deps family crates.
#[cfg(feature = "api")]
mod types;

#[cfg(feature = "api")]
pub use types::{
    G3RsDepsConfigChecksInput, G3RsDepsConfigInputScope, G3RsDepsDependencySection,
    G3RsDepsFileTreeChecksInput, G3RsDepsResolvedDependency, G3RsDepsSourceChecksInput,
};
