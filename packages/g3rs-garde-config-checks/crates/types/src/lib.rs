use cargo_toml_parser::CargoToml;
use clippy_toml_parser::ClippyToml;

/// Parsed root `Cargo.toml` input for `RS-GARDE-CONFIG-01`.
#[derive(Debug, Clone)]
pub struct G3RsGardeConfigDependencyCheckInput {
    /// Repo-relative path to the routed root Cargo manifest.
    pub cargo_rel_path: String,
    /// Parsed Cargo manifest content.
    pub cargo: CargoToml,
}

/// Parsed covering clippy config input for `RS-GARDE-CONFIG-02/03/04/06`.
#[derive(Debug, Clone)]
pub struct G3RsGardeConfigClippyBanChecksInput {
    /// Repo-relative path to the selected covering clippy config.
    pub clippy_rel_path: String,
    /// Parsed clippy config content.
    pub clippy: ClippyToml,
}
