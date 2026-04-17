use clippy_toml_parser::types::ClippyToml;
use guardrail3_rs_toml_parser::RustProfile;

#[derive(Debug, Clone)]
pub enum G3RsClippyConfigState {
    Unreadable {
        reason: String,
    },
    ParseError {
        reason: String,
    },
    Parsed {
        raw: toml::Value,
        typed: Result<ClippyToml, String>,
    },
}

#[derive(Debug, Clone)]
pub enum G3RsClippyRustPolicyState {
    Missing,
    Unreadable {
        rel_path: String,
        reason: String,
    },
    ParseError {
        rel_path: String,
        reason: String,
    },
    Parsed {
        rel_path: String,
        profile: Option<RustProfile>,
        garde_enabled: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsClippyCargoConfigOverride {
    pub rel_path: String,
    pub parse_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsClippyWaiver {
    pub rule: String,
    pub file: String,
    pub selector: String,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct G3RsClippyConfigChecksInput {
    pub clippy_rel_path: String,
    pub clippy: G3RsClippyConfigState,
    pub rust_policy: G3RsClippyRustPolicyState,
    pub published_library_policy: bool,
    pub cargo_config_overrides: Vec<G3RsClippyCargoConfigOverride>,
    pub waivers: Vec<G3RsClippyWaiver>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsClippyShadowedConfig {
    pub rel_path: String,
    pub preferred_rel_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsClippyFileTreeChecksInput {
    pub preferred_root_config_rel_path: Option<String>,
    pub shadowed_same_root_configs: Vec<G3RsClippyShadowedConfig>,
}
