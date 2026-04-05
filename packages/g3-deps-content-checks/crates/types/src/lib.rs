mod input;

#[cfg(feature = "api")]
pub use input::{
    G3DepsDirectDependencyCapInput, G3DepsLocalPathCargoManifest, G3DepsPolicyContentChecksInput,
};
