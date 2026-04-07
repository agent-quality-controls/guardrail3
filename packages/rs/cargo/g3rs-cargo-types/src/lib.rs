use cargo_toml_parser::CargoToml;

/// Input contract for extracted Cargo config checks.
///
/// The app owns discovery, routing, and parse-failure signaling. This package
/// receives one already-selected parsed `Cargo.toml` file and validates only
/// config semantics that can be determined from that file itself.
#[derive(Debug, Clone)]
pub struct G3RsCargoConfigChecksInput {
    /// Repo-relative path to the active `Cargo.toml`.
    pub cargo_rel_path: String,
    /// Parsed Cargo manifest content.
    pub cargo: CargoToml,
}

/// Placeholder input contract for future Cargo AST checks.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct G3RsCargoAstChecksInput;

/// Placeholder input contract for future Cargo file-tree checks.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct G3RsCargoFileTreeChecksInput;
