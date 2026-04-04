#![allow(
    clippy::expect_used,
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    reason = "assertion helpers are reusable panic-based proof sites for test harnesses"
)]

use clippy_toml_parser::{BanEntry, ClippyToml, Value};

pub fn assert_thresholds_empty(cfg: &ClippyToml) {
    assert_eq!(cfg.max_struct_bools, None, "max_struct_bools should be None");
    assert_eq!(
        cfg.cognitive_complexity_threshold, None,
        "threshold should be None",
    );
}

pub fn assert_ban_lists_empty(cfg: &ClippyToml) {
    assert!(cfg.disallowed_methods.is_empty(), "disallowed_methods should be empty");
    assert!(cfg.disallowed_types.is_empty(), "disallowed_types should be empty");
    assert!(cfg.disallowed_macros.is_empty(), "disallowed_macros should be empty");
}

pub fn assert_extra_empty(cfg: &ClippyToml) {
    assert!(cfg.extra.is_empty(), "extra should be empty");
}

#[allow(clippy::too_many_arguments, reason = "threshold tuple would be less readable at callsites")]
pub fn assert_thresholds(
    cfg: &ClippyToml,
    max_struct_bools: Option<u32>,
    max_fn_params_bools: Option<u32>,
    too_many_lines_threshold: Option<u32>,
    too_many_arguments_threshold: Option<u32>,
    excessive_nesting_threshold: Option<u32>,
    cognitive_complexity_threshold: Option<u32>,
    type_complexity_threshold: Option<u32>,
) {
    assert_eq!(cfg.max_struct_bools, max_struct_bools, "max_struct_bools mismatch");
    assert_eq!(
        cfg.max_fn_params_bools, max_fn_params_bools,
        "max_fn_params_bools mismatch",
    );
    assert_eq!(
        cfg.too_many_lines_threshold, too_many_lines_threshold,
        "too_many_lines_threshold mismatch",
    );
    assert_eq!(
        cfg.too_many_arguments_threshold, too_many_arguments_threshold,
        "too_many_arguments_threshold mismatch",
    );
    assert_eq!(
        cfg.excessive_nesting_threshold, excessive_nesting_threshold,
        "excessive_nesting_threshold mismatch",
    );
    assert_eq!(
        cfg.cognitive_complexity_threshold, cognitive_complexity_threshold,
        "cognitive_complexity_threshold mismatch",
    );
    assert_eq!(
        cfg.type_complexity_threshold, type_complexity_threshold,
        "type_complexity_threshold mismatch",
    );
}

pub fn assert_ban_entry(entry: Option<&BanEntry>, path: &str, reason: Option<&str>) {
    let entry = entry.expect("ban entry should exist");
    assert_eq!(entry.path(), path, "ban path mismatch");
    assert_eq!(entry.reason(), reason, "ban reason mismatch");
}

pub fn assert_list_len<T>(items: &[T], expected: usize, field_name: &str) {
    assert_eq!(items.len(), expected, "{field_name} count mismatch");
}

pub fn assert_bool_field(actual: Option<bool>, expected: Option<bool>, field_name: &str) {
    assert_eq!(actual, expected, "{field_name} mismatch");
}

pub fn assert_top_level_string_extra(cfg: &ClippyToml, key: &str, expected: &str) {
    assert_eq!(
        cfg.extra.get(key).and_then(Value::as_str),
        Some(expected),
        "unknown key should be captured",
    );
}

pub fn assert_parse_error(err: impl std::fmt::Display) {
    let msg = err.to_string();
    assert!(
        msg.contains("invalid clippy.toml"),
        "expected error message prefix, got: {msg}",
    );
}
