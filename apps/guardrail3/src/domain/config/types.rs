use std::collections::BTreeMap;

use serde::Deserialize;

/// Type alias for crate configuration map.
type CrateMap = BTreeMap<String, CrateConfig>;

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
    #[garde(dive)] // reason: recursively validate nested LocalConfig
    pub local: Option<LocalConfig>,
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
    #[garde(skip)] // reason: BTreeMap values — garde cannot dive into map values; validated by layer_from_config
    pub crates: Option<CrateMap>,
}

#[derive(Debug, Deserialize, garde::Validate)]
pub struct CrateConfig {
    #[garde(inner(length(min = 1)))] // reason: layer name must be non-empty when present
    pub layer: Option<String>,
    #[garde(inner(length(min = 1)))] // reason: profile name must be non-empty when present
    pub profile: Option<String>,
    #[garde(inner(inner(length(min = 1))))] // reason: each allowed dep name must be non-empty
    pub allowed_deps: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, garde::Validate)]
pub struct TypeScriptConfig {
    #[garde(inner(inner(length(min = 1))))] // reason: each app path must be non-empty
    pub apps: Option<Vec<String>>,
    #[garde(inner(length(min = 1)))] // reason: migrations path must be non-empty when present
    pub migrations: Option<String>,
    #[garde(dive)] // reason: recursively validate nested EslintConfig
    pub eslint: Option<EslintConfig>,
    #[garde(dive)] // reason: recursively validate nested CanonicalConfig
    pub canonical: Option<CanonicalConfig>,
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
pub struct LocalConfig {
    #[garde(inner(length(min = 1)))] // reason: file path must be non-empty when present
    pub clippy_methods: Option<String>,
    #[garde(inner(length(min = 1)))] // reason: file path must be non-empty when present
    pub clippy_types: Option<String>,
    #[garde(inner(length(min = 1)))] // reason: file path must be non-empty when present
    pub deny_bans: Option<String>,
    #[garde(inner(length(min = 1)))] // reason: file path must be non-empty when present
    pub deny_skip: Option<String>,
    #[garde(inner(length(min = 1)))] // reason: file path must be non-empty when present
    pub deny_feature_bans: Option<String>,
}

#[derive(Debug, Deserialize, garde::Validate)]
pub struct HooksConfig {
    #[garde(inner(length(min = 1)))] // reason: extra_dir path must be non-empty when present
    pub extra_dir: Option<String>,
}
