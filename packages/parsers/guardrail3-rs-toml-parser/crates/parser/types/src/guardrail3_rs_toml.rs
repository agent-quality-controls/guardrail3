#![allow(
    clippy::missing_docs_in_private_items,
    reason = "this file mirrors guardrail3-rs.toml schema directly; field names intentionally track TOML keys"
)]

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use toml::Value;

/// Typed representation of a `guardrail3-rs.toml` file.
///
/// Known workspace-level Rust policy fields are mapped to typed fields.
/// Unknown top-level keys are captured in [`extra`](Self::extra) so the model
/// can stay forward compatible as the schema evolves.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Guardrail3RsToml {
    pub version: Option<String>,
    pub profile: Option<RustProfile>,
    #[serde(default)]
    pub excluded_paths: Vec<String>,
    #[serde(default)]
    pub allowed_deps: Vec<String>,
    pub checks: Option<RustChecksConfig>,
    #[serde(default)]
    pub waivers: Vec<WaiverConfig>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RustProfile {
    Service,
    Library,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RustChecksConfig {
    pub topology: Option<bool>,
    pub arch: Option<bool>,
    pub fmt: Option<bool>,
    pub toolchain: Option<bool>,
    pub clippy: Option<bool>,
    pub deny: Option<bool>,
    pub cargo: Option<bool>,
    pub code: Option<bool>,
    pub deps: Option<bool>,
    pub garde: Option<bool>,
    pub test: Option<bool>,
    pub release: Option<bool>,
    pub hooks_shared: Option<bool>,
    pub hooks_rs: Option<bool>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WaiverConfig {
    pub rule: String,
    pub file: String,
    pub selector: String,
    pub reason: String,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}
