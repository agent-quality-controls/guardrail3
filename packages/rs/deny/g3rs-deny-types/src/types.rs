use deny_toml_parser::types::DenyToml;
use g3rs_toml_parser::types::RustProfile;

#[derive(Debug, Clone)]
pub struct G3RsDenyConfigChecksInput {
    pub deny_rel_path: String,
    pub deny: DenyToml,
    pub rust_policy: G3RsDenyRustPolicyState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3RsDenyRustPolicyState {
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
    },
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct G3RsDenySourceChecksInput;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsDenyInputFailure {
    pub title: String,
    pub rel_path: String,
    pub message: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct G3RsDenyFileTreeChecksInput {
    pub selected_deny_rel_path: Option<String>,
    pub candidate_deny_rel_paths: Vec<String>,
    pub input_failures: Vec<G3RsDenyInputFailure>,
}
