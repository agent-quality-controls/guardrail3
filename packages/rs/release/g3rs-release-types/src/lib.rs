use cargo_toml_parser::CargoToml;
use cliff_toml_parser::CliffToml;
use release_plz_toml_parser::ReleasePlzToml;

/// Input contract for extracted release config checks.
///
/// The app owns discovery, routing, and parse-failure signaling. This package
/// receives already-selected parsed files and validates config semantics.
///
/// Per-crate checks (01-09) read only `cargo_rel_path` + `cargo`.
/// Per-repo checks (10-11) read the optional release-plz and cliff fields.
/// The caller provides release-plz/cliff once and can pass `None` for
/// subsequent crates to avoid duplicate per-repo findings.
#[derive(Debug, Clone)]
pub struct G3RsReleaseConfigChecksInput {
    /// Repo-relative path to the crate `Cargo.toml` being checked.
    pub cargo_rel_path: String,
    /// Parsed Cargo manifest.
    pub cargo: CargoToml,
    /// Repo-relative path to `release-plz.toml`, if present.
    pub release_plz_rel_path: Option<String>,
    /// Parsed `release-plz.toml` content, if present.
    pub release_plz: Option<ReleasePlzToml>,
    /// Repo-relative path to `cliff.toml`, if present.
    pub cliff_rel_path: Option<String>,
    /// Parsed `cliff.toml` content, if present.
    pub cliff: Option<CliffToml>,
}
