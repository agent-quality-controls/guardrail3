#![allow(
    clippy::enum_variant_names,
    reason = "enum variant names intentionally mirror rustfmt's upstream option names"
)]

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use toml::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NewlineStyle {
    Auto,
    Windows,
    Unix,
    Native,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BraceStyle {
    AlwaysNextLine,
    PreferSameLine,
    SameLineWhere,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ControlBraceStyle {
    AlwaysSameLine,
    ClosingNextLine,
    AlwaysNextLine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IndentStyle {
    Visual,
    Block,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Heuristics {
    Off,
    Max,
    Default,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GroupImportsTactic {
    Preserve,
    StdExternalCrate,
    One,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImportGranularity {
    Preserve,
    Crate,
    Module,
    Item,
    One,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HexLiteralCase {
    Preserve,
    Upper,
    Lower,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FloatLiteralTrailingZero {
    Preserve,
    Always,
    IfNoPostfix,
    Never,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmitMode {
    Files,
    Stdout,
    Coverage,
    Checkstyle,
    Json,
    ModifiedLines,
    Diff,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Color {
    Always,
    Never,
    Auto,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Version {
    One,
    Two,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MatchArmLeadingPipe {
    Always,
    Never,
    Preserve,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Edition {
    #[serde(rename = "2015")]
    Edition2015,
    #[serde(rename = "2018")]
    Edition2018,
    #[serde(rename = "2021")]
    Edition2021,
    #[serde(rename = "2024")]
    Edition2024,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StyleEdition {
    #[serde(rename = "2015")]
    Edition2015,
    #[serde(rename = "2018")]
    Edition2018,
    #[serde(rename = "2021")]
    Edition2021,
    #[serde(rename = "2024")]
    Edition2024,
    #[serde(rename = "2027")]
    Edition2027,
}

/// Typed representation of a `rustfmt.toml` / `.rustfmt.toml` configuration file.
///
/// All known rustfmt options are represented as `Option<T>` fields so that
/// absent keys parse as `None`. Unknown keys are captured in the `extra`
/// catch-all via `#[serde(flatten)]`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct RustfmtToml {
    pub max_width: Option<u32>,
    pub hard_tabs: Option<bool>,
    pub tab_spaces: Option<u32>,
    pub newline_style: Option<NewlineStyle>,
    pub indent_style: Option<IndentStyle>,
    pub use_small_heuristics: Option<Heuristics>,
    pub fn_call_width: Option<u32>,
    pub attr_fn_like_width: Option<u32>,
    pub blank_lines_lower_bound: Option<u32>,
    pub blank_lines_upper_bound: Option<u32>,
    pub struct_lit_width: Option<u32>,
    pub struct_variant_width: Option<u32>,
    pub array_width: Option<u32>,
    pub chain_width: Option<u32>,
    pub single_line_if_else_max_width: Option<u32>,
    pub single_line_let_else_max_width: Option<u32>,
    pub wrap_comments: Option<bool>,
    pub format_code_in_doc_comments: Option<bool>,
    pub doc_comment_code_block_width: Option<u32>,
    pub comment_width: Option<u32>,
    pub normalize_comments: Option<bool>,
    pub normalize_doc_attributes: Option<bool>,
    pub overflow_delimited_expr: Option<bool>,
    pub format_strings: Option<bool>,
    pub hex_literal_case: Option<HexLiteralCase>,
    pub float_literal_trailing_zero: Option<FloatLiteralTrailingZero>,
    pub format_macro_bodies: Option<bool>,
    pub format_macro_matchers: Option<bool>,
    #[serde(default)]
    pub skip_macro_invocations: Vec<String>,
    pub color: Option<Color>,
    pub reorder_imports: Option<bool>,
    pub reorder_modules: Option<bool>,
    pub group_imports: Option<GroupImportsTactic>,
    pub imports_granularity: Option<ImportGranularity>,
    pub imports_indent: Option<String>,
    pub imports_layout: Option<String>,
    pub merge_imports: Option<bool>,
    pub reorder_impl_items: Option<bool>,
    pub empty_item_single_line: Option<bool>,
    pub struct_lit_single_line: Option<bool>,
    pub fn_single_line: Option<bool>,
    pub where_single_line: Option<bool>,
    pub space_before_colon: Option<bool>,
    pub space_after_colon: Option<bool>,
    pub spaces_around_ranges: Option<bool>,
    pub type_punctuation_density: Option<String>,
    pub binop_separator: Option<String>,
    pub brace_style: Option<BraceStyle>,
    pub control_brace_style: Option<ControlBraceStyle>,
    pub match_arm_blocks: Option<bool>,
    pub match_arm_leading_pipes: Option<MatchArmLeadingPipe>,
    pub match_arm_indent: Option<bool>,
    pub match_block_trailing_comma: Option<bool>,
    pub force_multiline_blocks: Option<bool>,
    pub fn_args_layout: Option<String>,
    pub fn_params_layout: Option<String>,
    pub merge_derives: Option<bool>,
    pub use_try_shorthand: Option<bool>,
    pub use_field_init_shorthand: Option<bool>,
    pub remove_nested_parens: Option<bool>,
    pub condense_wildcard_suffixes: Option<bool>,
    pub force_explicit_abi: Option<bool>,
    pub trailing_semicolon: Option<bool>,
    pub trailing_comma: Option<String>,
    pub combine_control_expr: Option<bool>,
    pub short_array_element_width_threshold: Option<u32>,
    pub struct_field_align_threshold: Option<u32>,
    pub enum_discrim_align_threshold: Option<u32>,
    pub inline_attribute_width: Option<u32>,
    pub format_generated_files: Option<bool>,
    pub generated_marker_line_search_limit: Option<u32>,
    pub edition: Option<Edition>,
    pub style_edition: Option<StyleEdition>,
    pub version: Option<Version>,
    pub required_version: Option<String>,
    pub emit_mode: Option<EmitMode>,
    #[serde(default)]
    pub ignore: Vec<String>,
    pub make_backup: Option<bool>,
    pub print_misformatted_file_names: Option<bool>,
    pub unstable_features: Option<bool>,
    pub disable_all_formatting: Option<bool>,
    pub skip_children: Option<bool>,
    pub error_on_line_overflow: Option<bool>,
    pub error_on_unformatted: Option<bool>,
    pub hide_parse_errors: Option<bool>,
    pub show_parse_errors: Option<bool>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}
