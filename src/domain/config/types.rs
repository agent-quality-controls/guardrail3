use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Debug, Deserialize, garde::Validate)]
#[allow(dead_code)] // reason: fields deserialized from guardrail3.toml, used by generate features
pub struct GuardrailConfig {
    #[garde(skip)] // reason: guardrail3's own config — validated by TOML schema, not runtime garde
    pub version: Option<String>,
    #[garde(skip)] // reason: guardrail3's own config — validated by TOML schema
    pub profile: Option<ProfileConfig>,
    #[garde(skip)] // reason: guardrail3's own config — validated by TOML schema
    pub rust: Option<RustConfig>,
    #[garde(skip)] // reason: guardrail3's own config — validated by TOML schema
    pub typescript: Option<TypeScriptConfig>,
    #[garde(skip)] // reason: guardrail3's own config — validated by TOML schema
    pub local: Option<LocalConfig>,
    #[garde(skip)] // reason: guardrail3's own config — validated by TOML schema
    pub hooks: Option<HooksConfig>,
}

#[derive(Debug, Deserialize, garde::Validate)]
pub struct ProfileConfig {
    #[garde(skip)] // reason: validated by profile matching logic
    pub name: String,
}

#[derive(Debug, Deserialize, garde::Validate)]
#[allow(dead_code)] // reason: fields deserialized from guardrail3.toml, used by generate features
pub struct RustConfig {
    #[garde(skip)] // reason: validated at use site
    pub workspace_root: Option<String>,
    #[garde(skip)] // reason: validated at use site
    pub workspaces: Option<Vec<String>>,
    #[allow(clippy::type_complexity)] // reason: legitimate complex type
    #[garde(skip)] // reason: validated at use site
    pub crates: Option<BTreeMap<String, CrateConfig>>,
}

#[derive(Debug, Deserialize, garde::Validate)]
pub struct CrateConfig {
    #[garde(skip)] // reason: validated by layer_from_config
    pub layer: Option<String>,
    #[garde(skip)] // reason: validated by profile matching logic
    pub profile: Option<String>,
    #[garde(skip)] // reason: validated by dependency_allowlist checks
    pub allowed_deps: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, garde::Validate)]
#[allow(dead_code)] // reason: fields deserialized from guardrail3.toml, used by generate features
pub struct TypeScriptConfig {
    #[garde(skip)] // reason: validated at use site
    pub apps: Option<Vec<String>>,
    #[garde(skip)] // reason: validated at use site
    pub migrations: Option<String>,
    #[garde(skip)] // reason: validated at use site
    pub eslint: Option<EslintConfig>,
    #[garde(skip)] // reason: validated at use site
    pub canonical: Option<CanonicalConfig>,
}

#[derive(Debug, Deserialize, garde::Validate)]
pub struct EslintConfig {
    #[garde(skip)] // reason: validated at use site
    pub mode: Option<String>,
}

#[derive(Debug, Deserialize, garde::Validate)]
pub struct CanonicalConfig {
    #[garde(skip)] // reason: boolean flags, no validation needed
    pub npmrc: Option<bool>,
    #[garde(skip)] // reason: boolean flags, no validation needed
    pub tsconfig_base: Option<bool>,
    #[garde(skip)] // reason: boolean flags, no validation needed
    pub jscpd: Option<bool>,
}

#[derive(Debug, Deserialize, garde::Validate)]
pub struct LocalConfig {
    #[garde(skip)] // reason: file paths validated at read time
    pub clippy_methods: Option<String>,
    #[garde(skip)] // reason: file paths validated at read time
    pub clippy_types: Option<String>,
    #[garde(skip)] // reason: file paths validated at read time
    pub deny_bans: Option<String>,
    #[garde(skip)] // reason: file paths validated at read time
    pub deny_skip: Option<String>,
    #[garde(skip)] // reason: file paths validated at read time
    pub deny_feature_bans: Option<String>,
}

#[derive(Debug, Deserialize, garde::Validate)]
#[allow(dead_code)] // reason: fields deserialized from guardrail3.toml, used by generate features
pub struct HooksConfig {
    #[garde(skip)] // reason: validated at use site
    pub extra_dir: Option<String>,
}
