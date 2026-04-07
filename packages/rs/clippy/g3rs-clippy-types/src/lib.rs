use clippy_toml_parser::ClippyToml;

/// Input contract for the extracted typed `clippy.toml` config checks.
///
/// The app owns discovery, route legality, and parse/schema failure reporting.
/// This package receives an already selected parsed `clippy.toml` and validates
/// only config rules that operate on valid typed data.
#[derive(Debug, Clone)]
pub struct G3RsClippyConfigChecksInput {
    /// Repo-relative path to the active clippy config.
    pub clippy_rel_path: String,
    /// Parsed clippy config content.
    pub clippy: ClippyToml,
}

/// Placeholder input contract for future clippy AST checks.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct G3RsClippyAstChecksInput;

/// Placeholder input contract for future clippy file-tree checks.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct G3RsClippyFileTreeChecksInput;
