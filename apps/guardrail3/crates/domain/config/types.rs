use std::collections::BTreeMap;

use serde::Deserialize;

/// Type alias for crate configuration map.
type CrateMap = BTreeMap<String, CrateConfig>;

/// Type alias for TypeScript app configuration map.
type TsAppMap = BTreeMap<String, TsAppConfig>;

#[derive(Debug, Deserialize, garde::Validate)]
pub struct GuardrailConfig {
    #[garde(inner(length(min = 1)))] // reason: version string must be non-empty when present
    pub version: Option<String>,
    #[garde(dive)] // reason: recursively validate nested ProfileConfig
    pub profile: Option<ProfileConfig>,
    #[garde(dive)] // reason: recursively validate nested RustConfig
    pub rust: Option<RustConfig>,
    #[garde(dive)] // reason: recursively validate nested TypeScriptConfig
    pub typescript: Option<TypeScriptConfig>,
    #[garde(dive)] // reason: recursively validate nested HooksConfig
    pub hooks: Option<HooksConfig>,
}

#[derive(Debug, Deserialize, garde::Validate)]
pub struct ProfileConfig {
    #[garde(length(min = 1))] // reason: profile name must be non-empty
    pub name: String,
}

#[derive(Debug, Deserialize, garde::Validate)]
pub struct RustConfig {
    #[garde(inner(length(min = 1)))] // reason: workspace root path must be non-empty when present
    pub workspace_root: Option<String>,
    #[garde(inner(inner(length(min = 1))))] // reason: each workspace path must be non-empty
    pub workspaces: Option<Vec<String>>,
    #[garde(skip)]
    // reason: BTreeMap values — garde cannot dive into map values; validated by layer_from_config
    pub apps: Option<CrateMap>,
    #[garde(dive)] // reason: recursively validate nested CrateConfig for packages
    pub packages: Option<CrateConfig>,
    #[garde(dive)] // reason: recursively validate nested RustChecksConfig
    pub checks: Option<RustChecksConfig>,
}

#[derive(Debug, Clone, Deserialize, garde::Validate)]
pub struct CrateConfig {
    #[garde(inner(length(min = 1)))] // reason: layer name must be non-empty when present
    pub layer: Option<String>,
    #[garde(inner(length(min = 1)))] // reason: profile name must be non-empty when present
    pub profile: Option<String>,
    /// App type — unified alias for `profile` (matches TS convention)
    #[serde(rename = "type")]
    #[garde(inner(length(min = 1)))] // reason: type name must be non-empty when present
    pub type_: Option<String>,
    #[garde(inner(inner(length(min = 1))))] // reason: each allowed dep name must be non-empty
    pub allowed_deps: Option<Vec<String>>,
    #[garde(dive)] // reason: recursively validate nested RustChecksConfig
    pub checks: Option<RustChecksConfig>,
}

#[derive(Debug, Clone, Deserialize, garde::Validate)]
pub struct RustChecksConfig {
    #[garde(skip)] // reason: Option<bool> — inherently valid
    pub architecture: Option<bool>,
    #[garde(skip)] // reason: Option<bool> — inherently valid
    pub garde: Option<bool>,
    #[garde(skip)] // reason: Option<bool> — inherently valid
    pub tests: Option<bool>,
    #[garde(skip)] // reason: Option<bool> — inherently valid
    pub release: Option<bool>,
}

#[derive(Debug, Deserialize, garde::Validate)]
pub struct TsChecksConfig {
    #[garde(skip)] // reason: Option<bool> — inherently valid
    pub architecture: Option<bool>,
    #[garde(skip)] // reason: Option<bool> — inherently valid
    pub content: Option<bool>,
    #[garde(skip)] // reason: Option<bool> — inherently valid
    pub tests: Option<bool>,
}

#[derive(Debug, Deserialize, garde::Validate)]
pub struct TsAppConfig {
    /// App type: "service", "content", or "library"
    #[serde(rename = "type")]
    #[garde(inner(length(min = 1)))] // reason: type name must be non-empty when present
    pub type_: Option<String>,
    #[garde(dive)] // reason: recursively validate nested TsChecksConfig
    pub checks: Option<TsChecksConfig>,
}

#[derive(Debug, Deserialize, garde::Validate)]
pub struct TypeScriptConfig {
    #[garde(skip)]
    // reason: BTreeMap values — garde cannot dive into map values; validated by type resolution
    pub apps: Option<TsAppMap>,
    #[garde(inner(length(min = 1)))] // reason: migrations path must be non-empty when present
    pub migrations: Option<String>,
    #[garde(dive)] // reason: recursively validate nested EslintConfig
    pub eslint: Option<EslintConfig>,
    #[garde(dive)] // reason: recursively validate nested CanonicalConfig
    pub canonical: Option<CanonicalConfig>,
    #[garde(dive)] // reason: recursively validate nested TsChecksConfig
    pub checks: Option<TsChecksConfig>,
}

#[derive(Debug, Deserialize, garde::Validate)]
pub struct EslintConfig {
    #[garde(inner(length(min = 1)))] // reason: eslint mode must be non-empty when present
    pub mode: Option<String>,
}

#[derive(Debug, Deserialize, garde::Validate)]
pub struct CanonicalConfig {
    #[garde(skip)] // reason: Option<bool> — inherently valid, no string validation needed
    pub npmrc: Option<bool>,
    #[garde(skip)] // reason: Option<bool> — inherently valid, no string validation needed
    pub tsconfig_base: Option<bool>,
    #[garde(skip)] // reason: Option<bool> — inherently valid, no string validation needed
    pub jscpd: Option<bool>,
}

#[derive(Debug, Deserialize, garde::Validate)]
pub struct HooksConfig {
    #[garde(inner(length(min = 1)))] // reason: extra_dir path must be non-empty when present
    pub extra_dir: Option<String>,
}
