/// Public type definitions for the Astro MDX surface contract.
#[cfg(feature = "api")]
mod types;

#[cfg(feature = "api")]
pub use types::{
    G3TsAstroMdxApprovedSourcePaths, G3TsAstroMdxConfigChecksInput,
    G3TsAstroMdxEslintDirectiveInput, G3TsAstroMdxEslintPluginContractInput,
    G3TsAstroMdxEslintSurfaceSnapshot, G3TsAstroMdxEslintSurfaceState,
    G3TsAstroMdxIntegrationContractInput, G3TsAstroMdxMissingComponentMapInput,
    G3TsAstroMdxPolicySnapshot, G3TsAstroMdxPolicySurfaceState, G3TsAstroPackageScriptCommand,
    G3TsAstroPackageScriptCommandSeparator, G3TsAstroPackageScriptParseBlocker,
    G3TsAstroPackageScriptToolInvocation, G3TsAstroPackageSurfaceSnapshot,
    G3TsAstroPackageSurfaceState,
};
