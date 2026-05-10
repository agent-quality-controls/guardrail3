//! Shared types for the g3rs apparch family.

#[cfg(feature = "api")]
mod types;

#[cfg(feature = "api")]
pub use types::{
    G3RsApparchBoundDependency, G3RsApparchConfigChecksInput, G3RsApparchCrate,
    G3RsApparchDependencyEdge, G3RsApparchDependencyKind, G3RsApparchExternalDependency,
    G3RsApparchLayer, G3RsApparchPatchBypass, G3RsApparchPatchKind, G3RsApparchPublicItem,
    G3RsApparchPublicItemKind, G3RsApparchRustPolicyState, G3RsApparchSourceChecksInput,
};
#[cfg(feature = "api")]
pub type G3RsApparchCrateDependencyChecksInput = types::G3RsApparchCrateDependencyChecksInput;
#[cfg(feature = "api")]
pub type G3RsApparchCratePurityChecksInput = types::G3RsApparchCratePurityChecksInput;
#[cfg(feature = "api")]
pub type G3RsApparchIoTraitsSourceChecksInput = types::G3RsApparchIoTraitsSourceChecksInput;
#[cfg(feature = "api")]
pub type G3RsApparchPatchBypassChecksInput = types::G3RsApparchPatchBypassChecksInput;
#[cfg(feature = "api")]
pub type G3RsApparchSameLayerCyclesChecksInput = types::G3RsApparchSameLayerCyclesChecksInput;
#[cfg(feature = "api")]
pub type G3RsApparchSameLayerDependencyEdge = types::G3RsApparchSameLayerDependencyEdge;
#[cfg(feature = "api")]
pub type G3RsApparchTypesPublicSurfaceChecksInput = types::G3RsApparchTypesPublicSurfaceChecksInput;
