/// Cargo rust-version state after app-side extraction from Cargo.toml.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3CargoRustVersion {
    /// Cargo.toml is missing for this policy root.
    MissingManifest,
    /// Cargo.toml could not be parsed, so rust-version extraction is blocked.
    ParseError(String),
    /// Cargo.toml does not declare rust-version.
    Missing,
    /// Cargo.toml declares rust-version with a non-string TOML value.
    InvalidType,
    /// Cargo.toml declares a string rust-version.
    Version(String),
}

/// Input contract for extracted toolchain content checks.
///
/// The app owns discovery, placement, and parse-failure routing. This package
/// receives already-selected file content for one policy root and validates the
/// content semantics only.
#[derive(Debug, Clone)]
pub struct G3ToolchainContentChecksInput {
    /// Repo-relative path to the active rust-toolchain.toml.
    pub toolchain_rel_path: String,
    /// Parsed rust-toolchain.toml content.
    pub toolchain_toml: toml::Value,
    /// Repo-relative path to the owning Cargo.toml.
    pub cargo_rel_path: String,
    /// Extracted rust-version state from Cargo.toml.
    pub cargo_rust_version: G3CargoRustVersion,
}
