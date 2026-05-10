#![allow(
    clippy::missing_docs_in_private_items,
    reason = "this file mirrors guardrail3-ts.toml schema directly; field names intentionally track TOML keys"
)]

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use toml::Value;

/// Typed representation of a `guardrail3-ts.toml` file.
///
/// Known per-package TS policy fields are mapped to typed fields. Unknown
/// top-level keys are captured in [`extra`](Self::extra) so the model can stay
/// forward compatible as the schema evolves.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Guardrail3TsToml {
    pub version: Option<String>,
    pub checks: Option<TsChecksConfig>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct TsChecksConfig {
    pub eslint: Option<bool>,
    pub astro_setup: Option<bool>,
    pub astro_content: Option<bool>,
    pub astro_mdx: Option<bool>,
    pub astro_i18n: Option<bool>,
    pub astro_media: Option<bool>,
    pub astro_seo: Option<bool>,
    pub astro_state: Option<bool>,
    pub arch: Option<bool>,
    pub apparch: Option<bool>,
    pub tsconfig: Option<bool>,
    pub package: Option<bool>,
    pub npmrc: Option<bool>,
    pub jscpd: Option<bool>,
    pub style: Option<bool>,
    pub fmt: Option<bool>,
    pub spelling: Option<bool>,
    pub typecov: Option<bool>,
    pub hooks: Option<bool>,
    pub topology: Option<bool>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}
