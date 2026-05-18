/// Public type definitions for the typecov surface contract.
#[cfg(feature = "api")]
mod types;

#[cfg(feature = "api")]
pub use types::{
    G3TsTypecovConfigChecksInput, G3TsTypecovContractInput,
    G3TsTypecovDependencyDeclarationSnapshot, G3TsTypecovPackageScriptCommandSeparator,
    G3TsTypecovPackageScriptParseBlocker, G3TsTypecovPackageScriptToolInvocation,
    G3TsTypecovPackageSurfaceSnapshot, G3TsTypecovPackageSurfaceState, G3TsTypecovPolicySnapshot,
    G3TsTypecovPolicySurfaceState, G3TsTypecovSyncpackSnapshot, G3TsTypecovSyncpackSurfaceState,
};
