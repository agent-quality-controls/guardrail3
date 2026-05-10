//! Shared types for g3ts style hooks.

/// Public input/state/snapshot types for the style check family.
#[cfg(feature = "api")]
mod types;

#[cfg(feature = "api")]
pub use types::{
    G3TsStyleConfigChecksInput, G3TsStyleContractInput, G3TsStyleEslintDirectiveInput,
    G3TsStyleEslintProbeDisablePolicySnapshot, G3TsStyleEslintSurfaceSnapshot,
    G3TsStyleEslintSurfaceState, G3TsStylePackageScriptCommandSeparator,
    G3TsStylePackageScriptParseBlocker, G3TsStylePackageScriptToolInvocation,
    G3TsStylePackageSurfaceSnapshot, G3TsStylePackageSurfaceState, G3TsStylePolicySnapshot,
    G3TsStylePolicySurfaceState, G3TsStyleSyncpackRequiredPin, G3TsStyleSyncpackSnapshot,
    G3TsStyleSyncpackSurfaceState, G3TsStyleSyncpackVersionGroupSnapshot,
    G3TsStylelintConfigSnapshot, G3TsStylelintConfigSurfaceState,
};
