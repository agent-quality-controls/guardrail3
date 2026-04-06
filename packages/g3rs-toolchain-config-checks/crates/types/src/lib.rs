use cargo_toml_parser::CargoToml;
use rust_toolchain_toml_parser::RustToolchainToml;

/// Input contract for `RS-TOOLCHAIN-CONFIG-01`.
///
/// The app owns discovery, placement, and parse-failure routing. This package
/// receives the already-selected parsed `rust-toolchain.toml` file and validates
/// its config semantics only.
#[derive(Debug, Clone)]
pub struct G3RsToolchainConfigChannelComponentsInput {
    /// Repo-relative path to the active rust-toolchain.toml.
    pub toolchain_rel_path: String,
    /// Parsed rust-toolchain.toml content.
    pub toolchain_toml: RustToolchainToml,
}

/// Input contract for `RS-TOOLCHAIN-CONFIG-02`.
///
/// The app owns discovery, placement, and parse-failure routing. This package
/// receives the already-selected parsed files for one policy root and validates
/// their config semantics only.
#[derive(Debug, Clone)]
pub struct G3RsToolchainConfigMsrvConsistencyInput {
    /// Repo-relative path to the active rust-toolchain.toml.
    pub toolchain_rel_path: String,
    /// Parsed rust-toolchain.toml content.
    pub toolchain_toml: RustToolchainToml,
    /// Repo-relative path to the owning Cargo.toml.
    pub cargo_rel_path: String,
    /// Parsed Cargo.toml content.
    pub cargo_toml: CargoToml,
}
