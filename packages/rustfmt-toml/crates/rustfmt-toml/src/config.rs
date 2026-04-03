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

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &str) -> RustfmtConfig {
        input.parse::<RustfmtConfig>().expect("should parse valid rustfmt.toml")
    }

    #[test]
    fn empty_string_yields_empty_config() {
        let cfg = parse("");

        assert_eq!(cfg.max_width, None, "max_width should be None for empty input");
        assert_eq!(cfg.hard_tabs, None, "hard_tabs should be None for empty input");
        assert_eq!(cfg.edition, None, "edition should be None for empty input");
        assert_eq!(cfg.newline_style, None, "newline_style should be None for empty input");
        assert!(cfg.ignore.is_empty(), "ignore should be empty for empty input");
        assert!(cfg.skip_macro_invocations.is_empty(), "skip_macro_invocations should be empty");
        assert!(cfg.extra.is_empty(), "extra should be empty for empty input");
    }

    #[test]
    fn realistic_config_parses_typed_fields() {
        let cfg = parse(r#"
max_width = 100
hard_tabs = false
tab_spaces = 4
edition = "2021"
newline_style = "Unix"
use_small_heuristics = "Default"
reorder_imports = true
merge_derives = true
"#);

        assert_eq!(cfg.max_width, Some(100), "max_width mismatch");
        assert_eq!(cfg.hard_tabs, Some(false), "hard_tabs mismatch");
        assert_eq!(cfg.tab_spaces, Some(4), "tab_spaces mismatch");
        assert_eq!(cfg.edition.as_deref(), Some("2021"), "edition mismatch");
        assert_eq!(cfg.newline_style.as_deref(), Some("Unix"), "newline_style mismatch");
        assert_eq!(cfg.use_small_heuristics.as_deref(), Some("Default"), "use_small_heuristics mismatch");
        assert_eq!(cfg.reorder_imports, Some(true), "reorder_imports mismatch");
        assert_eq!(cfg.merge_derives, Some(true), "merge_derives mismatch");
        assert!(cfg.extra.is_empty(), "known keys should not land in extra");
    }

    #[test]
    fn unknown_keys_land_in_extra() {
        let cfg = parse(r#"
max_width = 100
some_future_nightly_option = "yes"
another_unknown = 42
"#);

        assert_eq!(cfg.max_width, Some(100), "known key should still parse");
        assert_eq!(cfg.extra.len(), 2, "should capture 2 unknown keys");
        assert_eq!(
            cfg.extra
                .get("some_future_nightly_option")
                .and_then(toml::Value::as_str),
            Some("yes"),
            "unknown string key should be captured",
        );
        assert_eq!(
            cfg.extra
                .get("another_unknown")
                .and_then(toml::Value::as_integer),
            Some(42),
            "unknown integer key should be captured",
        );
    }

    #[test]
    fn ban_style_entries_roundtrip() {
        let cfg = parse(r#"
max_width = 120
ignore = ["generated.rs", "vendor/"]
skip_macro_invocations = ["bitflags"]
disable_all_formatting = false
"#);

        assert_eq!(cfg.max_width, Some(120), "max_width mismatch");
        assert_eq!(cfg.ignore, vec!["generated.rs", "vendor/"], "ignore list mismatch");
        assert_eq!(cfg.skip_macro_invocations, vec!["bitflags"], "skip_macro_invocations mismatch");
        assert_eq!(cfg.disable_all_formatting, Some(false), "disable_all_formatting mismatch");

        let serialized = toml::to_string(&cfg).expect("serialization should succeed");
        let cfg2 = parse(&serialized);
        assert_eq!(cfg, cfg2, "roundtrip should produce identical config");
    }

    #[test]
    fn from_str_error_on_invalid_toml() {
        let bad = "this is not [[[valid toml";
        let err = bad.parse::<RustfmtConfig>();
        assert!(err.is_err(), "invalid TOML should produce an error");

        let msg = err.expect_err("should be an error").to_string();
        assert!(
            msg.contains("invalid rustfmt.toml"),
            "expected error message prefix, got: {msg}",
        );
    }
}
