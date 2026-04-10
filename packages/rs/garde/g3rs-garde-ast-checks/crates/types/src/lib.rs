use std::path::PathBuf;

/// One governed file that the garde AST package is allowed to read directly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsAstFile {
    /// Repo-relative file path used in diagnostics.
    pub rel_path: String,
    /// Absolute file path used to read the file.
    pub abs_path: PathBuf,
}

/// Input contract for garde AST checks.
#[derive(Debug, Clone)]
pub struct G3RsGardeAstChecksInput {
    /// Whether the root Cargo.toml declares a garde dependency.
    pub garde_dependency_present: bool,
    /// Governed Rust source files for one garde root.
    pub source_files: Vec<G3RsAstFile>,
    /// Governing legacy `guardrail3.toml` for the same root.
    pub guardrail_toml: G3RsAstFile,
}
