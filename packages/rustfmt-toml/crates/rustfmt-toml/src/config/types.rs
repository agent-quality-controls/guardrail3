use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use toml::Value;

use crate::Error;

/// Typed representation of a `rustfmt.toml` / `.rustfmt.toml` configuration file.
///
/// All known rustfmt options are represented as `Option<T>` fields so that
/// absent keys parse as `None`.  Unknown keys are captured in the `extra`
/// catch-all via `#[serde(flatten)]`.
///
/// String is used for enum-valued options (e.g. `newline_style`, `edition`)
/// so the parser accepts any value; validation belongs to downstream consumers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
#[allow(clippy::struct_excessive_bools)] // reason: config struct mirrors rustfmt.toml schema — each bool maps to a rustfmt option
pub struct RustfmtConfig {
    // ── Width ────────────────────────────────────────────────────────
    pub max_width: Option<u32>,
    pub hard_tabs: Option<bool>,
    pub tab_spaces: Option<u32>,
    pub newline_style: Option<String>,
    pub indent_style: Option<String>,

    // ── Width heuristics ─────────────────────────────────────────────
    pub use_small_heuristics: Option<String>,
    pub fn_call_width: Option<u32>,
    pub attr_fn_like_width: Option<u32>,
    pub struct_lit_width: Option<u32>,
    pub struct_variant_width: Option<u32>,
    pub array_width: Option<u32>,
    pub chain_width: Option<u32>,
    pub single_line_if_else_max_width: Option<u32>,
    pub single_line_let_else_max_width: Option<u32>,

    // ── Comments ─────────────────────────────────────────────────────
    pub wrap_comments: Option<bool>,
    pub format_code_in_doc_comments: Option<bool>,
    pub doc_comment_code_block_width: Option<u32>,
    pub comment_width: Option<u32>,
    pub normalize_comments: Option<bool>,
    pub normalize_doc_attributes: Option<bool>,

    // ── Strings / literals ───────────────────────────────────────────
    pub format_strings: Option<bool>,
    pub hex_literal_case: Option<String>,
    pub float_literal_trailing_zero: Option<String>,

    // ── Macros ───────────────────────────────────────────────────────
    pub format_macro_bodies: Option<bool>,
    pub format_macro_matchers: Option<bool>,
    #[serde(default)]
    pub skip_macro_invocations: Vec<String>,

    // ── Imports ──────────────────────────────────────────────────────
    pub reorder_imports: Option<bool>,
    pub reorder_modules: Option<bool>,
    pub group_imports: Option<String>,
    pub imports_granularity: Option<String>,
    pub imports_indent: Option<String>,
    pub imports_layout: Option<String>,
    pub merge_imports: Option<bool>,
    pub reorder_impl_items: Option<bool>,

    // ── Single-line ──────────────────────────────────────────────────
    pub empty_item_single_line: Option<bool>,
    pub struct_lit_single_line: Option<bool>,
    pub fn_single_line: Option<bool>,
    pub where_single_line: Option<bool>,

    // ── Spacing ──────────────────────────────────────────────────────
    pub space_before_colon: Option<bool>,
    pub space_after_colon: Option<bool>,
    pub spaces_around_ranges: Option<bool>,
    pub type_punctuation_density: Option<String>,
    pub binop_separator: Option<String>,

    // ── Braces / match ───────────────────────────────────────────────
    pub brace_style: Option<String>,
    pub control_brace_style: Option<String>,
    pub match_arm_blocks: Option<bool>,
    pub match_arm_leading_pipes: Option<String>,
    pub match_arm_indent: Option<bool>,
    pub match_block_trailing_comma: Option<bool>,
    pub force_multiline_blocks: Option<bool>,

    // ── Functions ────────────────────────────────────────────────────
    pub fn_args_layout: Option<String>,
    pub fn_params_layout: Option<String>,

    // ── Derives / shortcuts ──────────────────────────────────────────
    pub merge_derives: Option<bool>,
    pub use_try_shorthand: Option<bool>,
    pub use_field_init_shorthand: Option<bool>,
    pub remove_nested_parens: Option<bool>,
    pub condense_wildcard_suffixes: Option<bool>,
    pub force_explicit_abi: Option<bool>,

    // ── Trailing / alignment ─────────────────────────────────────────
    pub trailing_semicolon: Option<bool>,
    pub trailing_comma: Option<String>,
    pub combine_control_expr: Option<bool>,
    pub short_array_element_width_threshold: Option<u32>,
    pub struct_field_align_threshold: Option<u32>,
    pub enum_discrim_align_threshold: Option<u32>,
    pub inline_attribute_width: Option<u32>,

    // ── Generated files ──────────────────────────────────────────────
    pub format_generated_files: Option<bool>,
    pub generated_marker_line_search_limit: Option<u32>,

    // ── Edition / version ────────────────────────────────────────────
    pub edition: Option<String>,
    pub style_edition: Option<String>,
    pub version: Option<String>,
    pub required_version: Option<String>,

    // ── Control ──────────────────────────────────────────────────────
    #[serde(default)]
    pub ignore: Vec<String>,
    pub unstable_features: Option<bool>,
    pub disable_all_formatting: Option<bool>,
    pub skip_children: Option<bool>,

    // ── Error output ─────────────────────────────────────────────────
    pub error_on_line_overflow: Option<bool>,
    pub error_on_unformatted: Option<bool>,
    pub hide_parse_errors: Option<bool>,
    pub show_parse_errors: Option<bool>,

    // ── Catch-all for unknown / nightly keys ─────────────────────────
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

impl RustfmtConfig {
    /// Read and parse a `rustfmt.toml` file from disk.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Io`] on read failure, [`Error::Toml`] on parse failure.
    pub fn from_path(path: impl AsRef<std::path::Path>) -> Result<Self, Error> {
        let content = crate::fs::read_to_string(path)?;
        content.parse()
    }
}

impl std::str::FromStr for RustfmtConfig {
    type Err = Error;

    #[allow(clippy::disallowed_methods)] // reason: this crate IS the centralized rustfmt.toml parser — toml::from_str is its core purpose
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(toml::from_str(s)?)
    }
}
