use deny_toml_parser::DenyToml;

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
    /// Active deny profile for this pointed workspace, when one is known.
    pub profile_name: Option<String>,
    /// Whether profile-sensitive deny rules can trust the active policy context.
    pub policy_context_valid: bool,
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
