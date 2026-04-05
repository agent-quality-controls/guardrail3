use cargo_toml_parser::CargoToml;

/// Input contract for extracted Cargo content checks.
///
/// The app owns discovery, routing, and parse-failure signaling. This package
/// receives one already-selected parsed `Cargo.toml` file and validates only
/// content semantics that can be determined from that file itself.
#[derive(Debug, Clone)]
pub struct G3CargoContentChecksInput {
    /// Repo-relative path to the active `Cargo.toml`.
    pub cargo_rel_path: String,
    /// Parsed Cargo manifest content.
    pub cargo: CargoToml,
}
