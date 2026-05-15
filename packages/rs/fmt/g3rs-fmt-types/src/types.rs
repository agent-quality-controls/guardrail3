//! Shared fmt-family types: parsed state, waivers, and check inputs.

use cargo_toml_parser::types::CargoToml;
use rust_toolchain_toml_parser::types::RustToolchainToml;
use rustfmt_toml_parser::types::RustfmtToml;
use serde::Serialize;

/// Parse state of a `rustfmt.toml` (or `.rustfmt.toml`) configuration file.
#[derive(Debug, Clone, Serialize)]
pub enum G3RsFmtRustfmtConfigState {
    /// Rustfmt config parsed successfully.
    Parsed(Box<RustfmtToml>),
    /// Rustfmt config exists but could not be read.
    Unreadable,
    /// Rustfmt config exists but failed to parse.
    ParseError,
}

/// Parse state of a `Cargo.toml` manifest.
#[derive(Debug, Clone, Serialize)]
pub enum G3RsFmtCargoState {
    /// Cargo manifest parsed successfully.
    Parsed(Box<CargoToml>),
    /// Cargo manifest is missing.
    Missing,
    /// Cargo manifest exists but could not be read.
    Unreadable,
    /// Cargo manifest exists but failed to parse.
    ParseError,
}

/// Parse state of a `rust-toolchain.toml` configuration file.
#[derive(Debug, Clone, Serialize)]
pub enum G3RsFmtToolchainState {
    /// Toolchain config parsed successfully.
    Parsed(Box<RustToolchainToml>),
    /// Toolchain config is missing.
    Missing,
    /// Toolchain config exists but could not be read.
    Unreadable,
    /// Toolchain config exists but failed to parse.
    ParseError,
}

/// A waiver entry declared in the rust policy file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct G3RsFmtWaiver {
    /// Rule identifier the waiver applies to.
    pub rule: String,
    /// File the waiver applies to.
    pub file: String,
    /// Selector targeting the waived item.
    pub selector: String,
    /// Reason justifying the waiver.
    pub reason: String,
}

/// Parse state of the per-workspace rust policy `guardrail3-rs.toml` file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum G3RsFmtRustPolicyState {
    /// No rust policy file present.
    Missing,
    /// Rust policy file exists but could not be read.
    Unreadable {
        /// Repo-relative path to the policy file.
        rel_path: String,
        /// Reason the file could not be read.
        reason: String,
    },
    /// Rust policy file exists and was read but failed to parse.
    ParseError {
        /// Repo-relative path to the policy file.
        rel_path: String,
        /// Reason the file failed to parse.
        reason: String,
    },
    /// Rust policy file was parsed successfully.
    Parsed {
        /// Repo-relative path to the policy file.
        rel_path: String,
        /// Waivers declared by the policy.
        waivers: Vec<G3RsFmtWaiver>,
    },
}

/// Aggregated config-level inputs for fmt-family checks.
#[derive(Debug, Clone, Serialize)]
pub struct G3RsFmtConfigChecksInput {
    /// Repo-relative path of the rustfmt config file.
    pub rustfmt_rel_path: String,
    /// Parse state of the rustfmt config file.
    pub rustfmt_state: G3RsFmtRustfmtConfigState,
    /// Keys explicitly set in the rustfmt config (preserves order).
    pub rustfmt_explicit_keys: Vec<String>,
    /// Repo-relative path of the workspace `Cargo.toml`.
    pub cargo_rel_path: String,
    /// Parse state of the workspace `Cargo.toml`.
    pub cargo_state: G3RsFmtCargoState,
    /// Repo-relative path of the rust-toolchain config file.
    pub toolchain_rel_path: String,
    /// Parse state of the rust-toolchain config file.
    pub toolchain_state: G3RsFmtToolchainState,
    /// Resolved rust policy state.
    pub rust_policy: G3RsFmtRustPolicyState,
}

/// Source-level input for fmt checks (currently empty; placeholder).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct G3RsFmtSourceChecksInput;

/// Kind of nested rustfmt config file detected in the file tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum G3RsFmtConfigFileKind {
    /// `rustfmt.toml` file.
    RustfmtToml,
    /// `.rustfmt.toml` file.
    DotRustfmtToml,
}

/// A nested rustfmt config file discovered under the workspace root.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct G3RsFmtNestedConfigFile {
    /// Repo-relative path to the nested config file.
    pub rel_path: String,
    /// Kind of nested config file.
    pub kind: G3RsFmtConfigFileKind,
}

/// File-tree-level input for fmt checks.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct G3RsFmtFileTreeChecksInput {
    /// Repo-relative path of the root `rustfmt.toml`, if present.
    pub root_rustfmt_toml_rel_path: Option<String>,
    /// Repo-relative path of the root `.rustfmt.toml`, if present.
    pub root_dot_rustfmt_toml_rel_path: Option<String>,
    /// Nested rustfmt config files discovered under the root.
    pub nested_config_files: Vec<G3RsFmtNestedConfigFile>,
    /// Directories that contain both `rustfmt.toml` and `.rustfmt.toml`.
    pub dual_conflict_dirs: Vec<String>,
}
