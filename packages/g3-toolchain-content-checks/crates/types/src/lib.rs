use cargo_toml_parser::CargoToml;
use rust_toolchain_toml_parser::RustToolchainToml;

/// Input contract for `RS-TOOLCHAIN-02`.
///
/// The app owns discovery, placement, and parse-failure routing. This package
/// receives the already-selected parsed `rust-toolchain.toml` file and validates
/// its content semantics only.
#[derive(Debug, Clone)]
pub struct G3ToolchainChannelAndComponentsInput {
    /// Repo-relative path to the active rust-toolchain.toml.
    pub toolchain_rel_path: String,
    /// Parsed rust-toolchain.toml content.
    pub toolchain_toml: RustToolchainToml,
}

/// Input contract for `RS-TOOLCHAIN-03`.
///
/// The app owns discovery, placement, and parse-failure routing. This package
/// receives the already-selected parsed files for one policy root and validates
/// their content semantics only.
#[derive(Debug, Clone)]
pub struct G3ToolchainMsrvConsistencyInput {
    /// Repo-relative path to the active rust-toolchain.toml.
    pub toolchain_rel_path: String,
    /// Parsed rust-toolchain.toml content.
    pub toolchain_toml: RustToolchainToml,
    /// Repo-relative path to the owning Cargo.toml.
    pub cargo_rel_path: String,
    /// Parsed Cargo.toml content.
    pub cargo_toml: CargoToml,
}
