use cargo_toml_parser::types::CargoToml;
use rust_toolchain_toml_parser::types::RustToolchainToml;

/// Input contract for extracted rust-toolchain config checks.
///
/// The app owns discovery, placement, and parse-failure routing. This package
/// receives already-selected parsed files and validates their config semantics.
///
/// `cargo_rel_path` and `cargo_toml` are optional because a workspace may not
/// have a `Cargo.toml` at the point of toolchain validation. When absent, the
/// MSRV consistency check is skipped.
#[derive(Debug, Clone)]
pub struct G3RsToolchainConfigChecksInput {
    /// Repo-relative path to the active `rust-toolchain.toml`.
    pub toolchain_rel_path: String,
    /// Parsed `rust-toolchain.toml` content.
    pub toolchain_toml: RustToolchainToml,
    /// Repo-relative path to the owning `Cargo.toml`, if present.
    pub cargo_rel_path: Option<String>,
    /// Parsed `Cargo.toml` content, if present.
    pub cargo_toml: Option<CargoToml>,
}

/// Placeholder input contract for future toolchain source checks.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct G3RsToolchainSourceChecksInput;

/// Input contract for extracted rust-toolchain filetree checks.
///
/// The package model validates one pointed workspace root. This input therefore
/// only describes root-level toolchain file presence.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct G3RsToolchainFileTreeChecksInput {
    /// Repo-relative path to `rust-toolchain.toml`, when present at the root.
    pub toolchain_toml_rel_path: Option<String>,
    /// Repo-relative path to legacy `rust-toolchain`, when present at the root.
    pub legacy_toolchain_rel_path: Option<String>,
}
