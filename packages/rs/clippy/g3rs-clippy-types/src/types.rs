use cargo_config_toml_parser::types::CargoConfigToml;
use cargo_toml_parser::types::CargoTomlDocument;
use clippy_toml_parser::types::ClippyTomlDocument;
use g3rs_toml_parser::types::RustProfile;

#[derive(Debug, Clone)]
pub enum G3RsClippyConfigState {
    Unreadable { reason: String },
    ParseError { reason: String },
    Parsed(Box<ClippyTomlDocument>),
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

#[derive(Debug, Clone, PartialEq)]
pub enum G3RsClippyCargoConfigState {
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
        cargo_config: Box<CargoConfigToml>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum G3RsClippyCargoRootState {
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
        cargo: Box<CargoTomlDocument>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum G3RsClippyCargoMemberState {
    Unreadable {
        member_rel: String,
        rel_path: String,
        reason: String,
    },
    ParseError {
        member_rel: String,
        rel_path: String,
        reason: String,
    },
    Parsed {
        member_rel: String,
        rel_path: String,
        cargo: Box<CargoTomlDocument>,
    },
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
    pub cargo_root: G3RsClippyCargoRootState,
    pub cargo_workspace_members: Vec<G3RsClippyCargoMemberState>,
    pub cargo_configs: Vec<G3RsClippyCargoConfigState>,
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
