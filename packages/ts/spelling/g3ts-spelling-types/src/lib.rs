/// Public type definitions exported by the spelling-types crate.
#[cfg(feature = "api")]
mod types;

#[cfg(feature = "api")]
pub use types::{
    G3TsSpellingConfigChecksInput, G3TsSpellingConfigSurfaceState, G3TsSpellingContractInput,
    G3TsSpellingPackageScriptCommandSeparator, G3TsSpellingPackageScriptParseBlocker,
    G3TsSpellingPackageScriptToolInvocation, G3TsSpellingPackageSurfaceSnapshot,
    G3TsSpellingPackageSurfaceState, G3TsSpellingSyncpackSnapshot,
    G3TsSpellingSyncpackSurfaceState, G3TsSpellingSyncpackVersionGroupSnapshot,
};
