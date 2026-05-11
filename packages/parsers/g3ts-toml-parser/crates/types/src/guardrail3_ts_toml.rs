#![allow(
    clippy::missing_docs_in_private_items,
    reason = "this file mirrors guardrail3-ts.toml schema directly; field names intentionally track TOML keys"
)]

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use toml::Value;

/// Maps each collection name to the list of field names declared for it in `guardrail3-ts.toml`.
pub type CollectionFieldsMap = BTreeMap<String, Vec<String>>;

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
    pub astro: Option<TsAstroPolicyConfig>,
    pub style: Option<TsStylePolicyConfig>,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct TsStylePolicyConfig {
    #[serde(default)]
    pub source_globs: Vec<String>,
    #[serde(default)]
    pub stylelint_css_globs: Vec<String>,
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
    pub i18n: Option<TsAstroI18nPolicyConfig>,
    pub media: Option<TsAstroMediaPolicyConfig>,
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
    pub collection_fields: CollectionFieldsMap,
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
    pub checked_internal_link_helpers: Vec<String>,
    #[serde(default)]
    pub approved_internal_link_helpers: Vec<String>,
    #[serde(default)]
    pub approved_localized_link_components: Vec<String>,
    #[serde(default)]
    pub approved_date_format_helpers: Vec<String>,
    #[serde(default)]
    pub approved_number_format_helpers: Vec<String>,
    #[serde(default)]
    pub public_source_globs: Vec<String>,
    #[serde(default)]
    pub helper_source_globs: Vec<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct TsAstroMediaPolicyConfig {
    #[serde(default)]
    pub favicon: String,
    #[serde(default)]
    pub app_icons: Vec<String>,
    #[serde(default)]
    pub default_social_image: String,
    #[serde(default)]
    pub allow_svg_icons: Option<bool>,
    #[serde(default)]
    pub public_source_globs: Vec<String>,
    #[serde(default)]
    pub media_helper_modules: Vec<String>,
    #[serde(default)]
    pub approved_media_helpers: Vec<String>,
    #[serde(default)]
    pub content_image_components: Vec<String>,
    #[serde(default)]
    pub content_image_key_props: Vec<String>,
    #[serde(default)]
    pub banned_image_source_props: Vec<String>,
    #[serde(default)]
    pub banned_image_alt_props: Vec<String>,
    #[serde(default)]
    pub allowed_public_image_paths: Vec<String>,
    #[serde(default)]
    pub checked_image_extensions: Vec<String>,
    #[serde(default)]
    pub metadata_image_property_names: Vec<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}
