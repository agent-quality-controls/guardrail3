use deny_toml_parser::DenyToml;
use guardrail3_rs_toml_parser::RustProfile;

/// Input contract for extracted deny config checks.
///
/// The app owns discovery, authoritative-file selection, and parse-failure
/// routing. This package receives one already-selected typed parsed deny file
/// and validates only its config semantics.
#[derive(Debug, Clone)]
pub struct G3RsDenyConfigChecksInput {
    /// Repo-relative path to the active `deny.toml`.
    pub deny_rel_path: String,
    /// Parsed deny config.
    pub deny: DenyToml,
    /// Active Rust-only guardrail policy state for this pointed workspace.
    pub rust_policy: G3RsDenyRustPolicyState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3RsDenyRustPolicyState {
    Missing,
    Unreadable { rel_path: String, reason: String },
    ParseError { rel_path: String, reason: String },
    Parsed {
        rel_path: String,
        profile: Option<RustProfile>,
    },
}

/// Placeholder input contract for future deny source checks.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct G3RsDenySourceChecksInput;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsDenyInputFailure {
    pub title: String,
    pub rel_path: String,
    pub message: String,
}

/// Input contract for deny filetree checks.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct G3RsDenyFileTreeChecksInput {
    pub selected_deny_rel_path: Option<String>,
    pub candidate_deny_rel_paths: Vec<String>,
    pub input_failures: Vec<G3RsDenyInputFailure>,
}
