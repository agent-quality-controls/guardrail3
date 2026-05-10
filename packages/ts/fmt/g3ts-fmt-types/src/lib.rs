/// Public input contract types for g3ts fmt config checks.
#[cfg(feature = "api")]
mod types;

#[cfg(feature = "api")]
pub use types::{
    G3TsFmtConfigChecksInput, G3TsFmtConfigSurfaceState, G3TsFmtContractInput,
    G3TsFmtPackageScriptCommandSeparator, G3TsFmtPackageScriptParseBlocker,
    G3TsFmtPackageScriptToolInvocation, G3TsFmtPackageSurfaceSnapshot, G3TsFmtPackageSurfaceState,
    G3TsFmtSyncpackSnapshot, G3TsFmtSyncpackSurfaceState, G3TsFmtSyncpackVersionGroupSnapshot,
};
