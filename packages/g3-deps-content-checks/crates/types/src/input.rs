use cargo_toml_parser::CargoToml;
use guardrail3_domain_config::types::GuardrailConfig;

/// Parsed local Cargo package discovered through a path dependency reference.
#[derive(Debug)]
pub struct G3DepsLocalPathCargoManifest {
    /// Repo-relative path to the local dependency `Cargo.toml`.
    pub cargo_rel_path: String,
    /// Parsed local dependency manifest.
    pub cargo: CargoToml,
}

/// Input contract for extracted dependency-policy content checks.
///
/// The app owns discovery, route selection, and malformed-input reporting.
/// This package receives the already-selected workspace manifest, crate
/// manifest, and workspace `guardrail3.toml` policy file as parsed types.
#[derive(Debug)]
pub struct G3DepsPolicyContentChecksInput {
    /// Repo-relative path to the authoritative workspace `Cargo.toml`.
    pub workspace_cargo_rel_path: String,
    /// Parsed workspace manifest.
    pub workspace_cargo: CargoToml,
    /// Repo-relative path to the crate `Cargo.toml` being checked.
    pub crate_cargo_rel_path: String,
    /// Parsed crate manifest.
    pub crate_cargo: CargoToml,
    /// Repo-relative path to the workspace `guardrail3.toml`.
    pub guardrail_rel_path: String,
    /// Parsed workspace policy config.
    pub guardrail: GuardrailConfig,
    /// Repo-relative `Cargo.toml` paths reached through local path dependency
    /// references for this crate policy site.
    pub local_path_cargo_rel_paths: Vec<String>,
    /// Parsed local Cargo manifests available for path dependency resolution.
    pub local_path_cargo_manifests: Vec<G3DepsLocalPathCargoManifest>,
}

/// Input contract for the direct-dependency-cap content check.
///
/// This check only needs the workspace and crate manifests.
#[derive(Debug)]
pub struct G3DepsDirectDependencyCapInput {
    /// Repo-relative path to the authoritative workspace `Cargo.toml`.
    pub workspace_cargo_rel_path: String,
    /// Parsed workspace manifest.
    pub workspace_cargo: CargoToml,
    /// Repo-relative path to the crate `Cargo.toml` being checked.
    pub crate_cargo_rel_path: String,
    /// Parsed crate manifest.
    pub crate_cargo: CargoToml,
    /// Repo-relative `Cargo.toml` paths reached through local path dependency
    /// references for this crate policy site.
    pub local_path_cargo_rel_paths: Vec<String>,
    /// Parsed local Cargo manifests available for path dependency resolution.
    pub local_path_cargo_manifests: Vec<G3DepsLocalPathCargoManifest>,
}
