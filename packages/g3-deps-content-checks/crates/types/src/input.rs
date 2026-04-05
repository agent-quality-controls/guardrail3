use cargo_toml_parser::CargoToml;
use guardrail3_rs_toml_parser::Guardrail3RsToml;

/// Input contract for extracted dependency-policy content checks.
///
/// The app owns discovery, route selection, and malformed-input reporting.
/// This package receives the already-selected workspace manifest, crate
/// manifest, and workspace Rust policy file as parsed types.
#[derive(Debug, Clone)]
pub struct G3DepsContentChecksInput {
    /// Repo-relative path to the authoritative workspace `Cargo.toml`.
    pub workspace_cargo_rel_path: String,
    /// Parsed workspace manifest.
    pub workspace_cargo: CargoToml,
    /// Repo-relative path to the crate `Cargo.toml` being checked.
    pub crate_cargo_rel_path: String,
    /// Parsed crate manifest.
    pub crate_cargo: CargoToml,
    /// Repo-relative path to the workspace Rust policy file.
    pub guardrail_rs_rel_path: String,
    /// Parsed workspace Rust policy.
    pub guardrail_rs: Guardrail3RsToml,
}
