use cargo_toml_parser::CargoToml;

/// Input contract for extracted Cargo content checks.
///
/// The app owns discovery, authoritative-file selection, and parse-failure
/// routing. This package receives already-selected typed parsed files and
/// validates only their content semantics.
#[derive(Debug)]
pub struct G3CargoContentChecksInput {
    /// Repo-relative path to the authoritative policy root `Cargo.toml`.
    pub policy_root_rel_path: String,
    /// Parsed policy-root manifest.
    pub policy_root_manifest: CargoToml,
    /// Repo-relative path and parsed manifests for workspace members.
    pub member_manifests: Vec<CargoMemberManifest>,
    /// Repo-relative path to the root-local cargo policy config, when present.
    pub policy_rel_path: Option<String>,
    /// Workspace policy profile when cargo rules need profile-sensitive behavior.
    pub policy_profile: Option<CargoPolicyProfile>,
    /// Documented cargo-manifest lint allow waivers relevant to this workspace root.
    pub lint_allow_waivers: Vec<CargoLintAllowWaiver>,
}

/// Parsed Cargo manifest for a workspace member.
#[derive(Debug, Clone)]
pub struct CargoMemberManifest {
    /// Repo-relative path to the member `Cargo.toml`.
    pub rel_path: String,
    /// Parsed member manifest.
    pub manifest: CargoToml,
}

/// Profile-sensitive cargo policy shape needed by extracted cargo rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CargoPolicyProfile {
    Service,
    Library,
}

/// Documented waiver for a manifest-level cargo lint allow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CargoLintAllowWaiver {
    /// Repo-relative path to the manifest the waiver applies to.
    pub manifest_rel_path: String,
    /// Exact cargo-rule selector for the waived allow entry, such as `clippy:redundant_pub_crate`.
    pub selector: String,
    /// Human justification carried with the waiver.
    pub reason: String,
}
