use cargo_toml_parser::CargoToml;
use rust_toolchain_toml_parser::RustToolchainToml;
use rustfmt_toml_parser::RustfmtToml;

/// Input contract for extracted rustfmt config checks.
///
/// The app owns discovery, authoritative-file selection, and parse-failure
/// routing. This package receives already-selected typed parsed files and
/// validates only their config semantics.
#[derive(Debug, Clone)]
pub struct G3RsFmtConfigChecksInput {
    /// Repo-relative path to the active `rustfmt.toml` / `.rustfmt.toml`.
    pub rustfmt_rel_path: String,
    /// Parsed rustfmt config.
    pub rustfmt: RustfmtToml,
    /// Repo-relative path to the authoritative Cargo manifest.
    pub cargo_rel_path: String,
    /// Parsed Cargo manifest.
    pub cargo: CargoToml,
    /// Repo-relative path to the authoritative rust-toolchain file.
    pub toolchain_rel_path: String,
    /// Parsed rust-toolchain manifest.
    pub toolchain: RustToolchainToml,
}

/// Placeholder input contract for future rustfmt source checks.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct G3RsFmtSourceChecksInput;

/// Placeholder input contract for future rustfmt file-tree checks.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct G3RsFmtFileTreeChecksInput;
