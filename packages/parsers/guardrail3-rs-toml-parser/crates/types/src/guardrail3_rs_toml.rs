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
    pub ts: Option<TsPolicyConfig>,
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
pub struct TsPolicyConfig {
    pub astro: Option<TsAstroPolicyConfig>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TsAstroPolicyConfig {
    pub profile: Option<String>,
    #[serde(default)]
    pub routes: TsAstroRoutesPolicyConfig,
    #[serde(default)]
    pub content: TsAstroContentPolicyConfig,
    #[serde(default)]
    pub mdx: TsAstroMdxPolicyConfig,
    #[serde(default)]
    pub seo: TsAstroSeoPolicyConfig,
    #[serde(default)]
    pub state: TsAstroStatePolicyConfig,
    #[serde(default)]
    pub i18n: TsAstroI18nPolicyConfig,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct TsAstroRoutesPolicyConfig {
    #[serde(default)]
    pub content: Vec<String>,
    #[serde(default)]
    pub non_content: Vec<String>,
    #[serde(default)]
    pub endpoints: Vec<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct TsAstroContentPolicyConfig {
    #[serde(default)]
    pub root: Option<String>,
    #[serde(default)]
    pub adapters: Vec<String>,
    #[serde(default)]
    pub required_collections: Vec<String>,
    #[serde(default)]
    pub collection_fields: BTreeMap<String, Vec<String>>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct TsAstroMdxPolicyConfig {
    #[serde(default)]
    pub component_maps: Vec<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct TsAstroSeoPolicyConfig {
    #[serde(default)]
    pub metadata_helpers: Vec<String>,
    #[serde(default)]
    pub json_ld_helpers: Vec<String>,
    #[serde(default)]
    pub strict_ai_readable: bool,
    #[serde(default)]
    pub llms_required_sections: Vec<String>,
    #[serde(default)]
    pub llms_required_links: Vec<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct TsAstroStatePolicyConfig {
    #[serde(default)]
    pub forbidden: Vec<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct TsAstroI18nPolicyConfig {
    #[serde(default)]
    pub locales: Vec<String>,
    #[serde(default)]
    pub default_locale: Option<String>,
    #[serde(default)]
    pub require_locale_prefix_for_content_routes: bool,
    #[serde(default)]
    pub allowed_unprefixed_routes: Vec<String>,
    #[serde(default)]
    pub content_route_prefixes: Vec<String>,
    #[serde(default)]
    pub approved_internal_link_helpers: Vec<String>,
    #[serde(default)]
    pub approved_localized_link_components: Vec<String>,
    #[serde(default)]
    pub approved_date_format_helpers: Vec<String>,
    #[serde(default)]
    pub approved_number_format_helpers: Vec<String>,
    #[serde(default)]
    pub content_image_components: Vec<String>,
    #[serde(default)]
    pub content_image_key_props: Vec<String>,
    #[serde(default)]
    pub banned_image_source_props: Vec<String>,
    #[serde(default)]
    pub banned_image_alt_props: Vec<String>,
    #[serde(default)]
    pub public_source_globs: Vec<String>,
    #[serde(default)]
    pub helper_source_globs: Vec<String>,
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
