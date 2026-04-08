/// One Rust source file selected for `code` AST checks.
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
}

/// Input contract for `g3rs-code-ast-checks`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsCodeAstChecksInput {
    /// The one source file this checks call may inspect.
    pub source_file: G3RsSourceFile,
}
