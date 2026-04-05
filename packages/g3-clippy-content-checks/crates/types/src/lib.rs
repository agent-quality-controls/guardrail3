use clippy_toml_parser::ClippyToml;

/// Input contract for the extracted typed `clippy.toml` content checks.
///
/// The app owns discovery, route legality, and parse/schema failure reporting.
/// This package receives an already selected parsed `clippy.toml` and validates
/// only content rules that operate on valid typed data.
#[derive(Debug, Clone)]
pub struct G3ClippyContentChecksInput {
    /// Repo-relative path to the active clippy config.
    pub clippy_rel_path: String,
    /// Parsed clippy config content.
    pub clippy: ClippyToml,
}
