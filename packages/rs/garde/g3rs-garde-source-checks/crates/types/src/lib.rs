use std::path::PathBuf;

/// One governed file that the garde source package is allowed to read directly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsSourceFile {
    /// Repo-relative file path used in diagnostics.
    pub rel_path: String,
    /// Absolute file path used to read the file.
    pub abs_path: PathBuf,
}

/// Input contract for garde source checks.
#[derive(Debug, Clone)]
pub struct G3RsGardeSourceChecksInput {
    /// Whether the root Cargo.toml declares a garde dependency.
    pub garde_dependency_present: bool,
    /// Governed Rust source files for one garde root.
    pub source_files: Vec<G3RsSourceFile>,
    /// Governing legacy `guardrail3.toml` for the same root.
    pub guardrail_toml: G3RsSourceFile,
}
