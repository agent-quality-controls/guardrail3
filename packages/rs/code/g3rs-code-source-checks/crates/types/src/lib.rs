/// One Rust source file selected for `code` source checks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsSourceFile {
    /// Repo-relative path used in diagnostics.
    pub rel_path: String,
    /// Full source text.
    pub content: String,
    /// Whether this file belongs to test-owned code.
    pub is_test: bool,
    /// Optional profile name already resolved by ingestion.
    pub profile_name: Option<String>,
    /// Whether this file is the actual library root source for its owning crate.
    pub is_library_root: bool,
}

/// Input contract for `g3rs-code-source-checks`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsCodeSourceChecksInput {
    /// The one source file this checks call may inspect.
    pub source_file: G3RsSourceFile,
}
