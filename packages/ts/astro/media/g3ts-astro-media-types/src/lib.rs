/// Public type definitions shared across the astro-media family.
#[cfg(feature = "api")]
mod types;

#[cfg(feature = "api")]
pub use types::{
    G3TsAstroCallSnapshot, G3TsAstroConfigSurfaceSnapshot, G3TsAstroConfigSurfaceState,
    G3TsAstroIntegrationSnapshot, G3TsAstroMediaConfigChecksInput,
    G3TsAstroMediaEslintPluginContractInput, G3TsAstroMediaEslintSurfaceSnapshot,
    G3TsAstroMediaEslintSurfaceState, G3TsAstroMediaIntegrationContractInput,
    G3TsAstroMediaPolicySnapshot, G3TsAstroMediaPolicySurfaceState, G3TsAstroPackageScriptBody,
    G3TsAstroPackageScriptCommand, G3TsAstroPackageScriptCommandSeparator,
    G3TsAstroPackageScriptParseBlocker, G3TsAstroPackageScriptToolInvocation,
    G3TsAstroPackageSurfaceSnapshot, G3TsAstroPackageSurfaceState,
    G3TsAstroPublicPluginPackageNames, G3TsAstroStaticObjectProperty, G3TsAstroStaticValue,
};
