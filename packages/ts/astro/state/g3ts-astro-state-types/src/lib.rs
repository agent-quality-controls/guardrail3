/// Public type definitions for the Astro state surface contract.
#[cfg(feature = "api")]
mod types;

#[cfg(feature = "api")]
pub use types::{
    G3TsAstroStateAppRootInput, G3TsAstroStateFileTreeChecksInput,
    G3TsAstroStateForbiddenPathInput, G3TsAstroStateLegacyGeneratedPathInput,
    G3TsAstroStatePolicySnapshot, G3TsAstroStatePolicySurfaceState,
    G3TsAstroStateStrictAppRootInput,
};
