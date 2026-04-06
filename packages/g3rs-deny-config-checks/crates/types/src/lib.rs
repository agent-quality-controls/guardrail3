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
}
