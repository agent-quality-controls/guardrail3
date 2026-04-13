use cargo_toml_parser::CargoToml;
use clippy_toml_parser::ClippyToml;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum G3RsGardeClippyInput {
    Missing,
    Parsed { rel_path: String, clippy: ClippyToml },
    Invalid { rel_path: String, message: String },
}

/// Input contract for extracted garde config checks.
///
/// The app owns discovery, placement, and parse-failure routing. This package
/// receives already-selected parsed files and validates their config semantics.
#[derive(Debug, Clone)]
pub struct G3RsGardeConfigChecksInput {
    /// Repo-relative path to the routed root Cargo manifest.
    pub cargo_rel_path: String,
    /// Parsed Cargo manifest content.
    pub cargo: CargoToml,
    /// Covering clippy config state for garde ban checks.
    pub clippy_input: G3RsGardeClippyInput,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsSourceFile {
    pub rel_path: String,
    pub abs_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct G3RsGardeSourceChecksInput {
    pub garde_dependency_present: bool,
    pub source_files: Vec<G3RsSourceFile>,
    pub guardrail_toml: G3RsSourceFile,
}

/// Placeholder input contract for future garde file-tree checks.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct G3RsGardeFileTreeChecksInput;
