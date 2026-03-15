use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)] // reason: fields deserialized from guardrail3.toml, used by generate features
pub struct GuardrailConfig {
    pub version: Option<String>,
    pub profile: Option<ProfileConfig>,
    pub rust: Option<RustConfig>,
    pub typescript: Option<TypeScriptConfig>,
    pub local: Option<LocalConfig>,
    pub hooks: Option<HooksConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ProfileConfig {
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)] // reason: fields deserialized from guardrail3.toml, used by generate features
pub struct RustConfig {
    pub workspace_root: Option<String>,
    pub workspaces: Option<Vec<String>>,
    #[allow(clippy::type_complexity)] // reason: legitimate complex type
    pub crates: Option<BTreeMap<String, CrateConfig>>,
}

#[derive(Debug, Deserialize)]
pub struct CrateConfig {
    pub layer: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)] // reason: fields deserialized from guardrail3.toml, used by generate features
pub struct TypeScriptConfig {
    pub apps: Option<Vec<String>>,
    pub migrations: Option<String>,
    pub eslint: Option<EslintConfig>,
    pub canonical: Option<CanonicalConfig>,
}

#[derive(Debug, Deserialize)]
pub struct EslintConfig {
    pub mode: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CanonicalConfig {
    pub npmrc: Option<bool>,
    pub tsconfig_base: Option<bool>,
    pub jscpd: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct LocalConfig {
    pub clippy_methods: Option<String>,
    pub clippy_types: Option<String>,
    pub deny_bans: Option<String>,
    pub deny_skip: Option<String>,
    pub deny_feature_bans: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)] // reason: fields deserialized from guardrail3.toml, used by generate features
pub struct HooksConfig {
    pub extra_dir: Option<String>,
}
