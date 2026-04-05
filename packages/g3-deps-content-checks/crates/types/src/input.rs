use cargo_toml_parser::CargoToml;
use guardrail3_domain_config::types::GuardrailConfig;

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
}
