use cargo_toml_parser::CargoToml;
use clippy_toml_parser::ClippyToml;

/// Input contract for extracted garde config checks.
///
/// The app owns discovery, placement, and parse-failure routing. This package
/// receives already-selected parsed files and validates their config semantics.
///
/// `clippy_rel_path` and `clippy` are optional because a workspace may not
/// have a clippy config. When absent, the clippy ban checks are skipped.
#[derive(Debug, Clone)]
pub struct G3RsGardeConfigChecksInput {
    /// Repo-relative path to the routed root Cargo manifest.
    pub cargo_rel_path: String,
    /// Parsed Cargo manifest content.
    pub cargo: CargoToml,
    /// Repo-relative path to the selected clippy config, if present.
    pub clippy_rel_path: Option<String>,
    /// Parsed clippy config content, if present.
    pub clippy: Option<ClippyToml>,
}

/// Placeholder input contract for future garde file-tree checks.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct G3RsGardeFileTreeChecksInput;
