#![allow(
    clippy::expect_used,
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    reason = "assertion helpers are reusable panic-based proof sites for test harnesses"
)]

use clippy_toml_parser_runtime::{ClippyToml, DisallowedPath};

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

#[allow(clippy::too_many_arguments, reason = "threshold tuple would be less readable at callsites")]
pub fn assert_thresholds(
    cfg: &ClippyToml,
    max_struct_bools: Option<u64>,
    max_fn_params_bools: Option<u64>,
    too_many_lines_threshold: Option<u64>,
    too_many_arguments_threshold: Option<u64>,
    excessive_nesting_threshold: Option<u64>,
    cognitive_complexity_threshold: Option<u64>,
    type_complexity_threshold: Option<u64>,
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

pub fn assert_ban_entry(entry: Option<&DisallowedPath>, path: &str, reason: Option<&str>) {
    let entry = entry.expect("ban entry should exist");
    match entry {
        DisallowedPath::Simple(simple) => {
            assert_eq!(simple, path, "ban path mismatch");
            assert_eq!(reason, None, "simple entries have no reason");
        }
        DisallowedPath::Detailed(detail) => {
            assert_eq!(detail.path, path, "ban path mismatch");
            assert_eq!(detail.reason.as_deref(), reason, "ban reason mismatch");
        }
    }
}

pub fn assert_list_len<T>(items: &[T], expected: usize, field_name: &str) {
    assert_eq!(items.len(), expected, "{field_name} count mismatch");
}

pub fn assert_bool_field(actual: Option<bool>, expected: Option<bool>, field_name: &str) {
    assert_eq!(actual, expected, "{field_name} mismatch");
}

pub fn assert_string_list(actual: &[String], expected: &[&str], field_name: &str) {
    let expected_values = expected.iter().map(ToString::to_string).collect::<Vec<_>>();
    assert_eq!(actual, expected_values, "{field_name} mismatch");
}

pub fn assert_parse_error(err: impl std::fmt::Display) {
    let msg = err.to_string();
    assert!(
        msg.contains("invalid clippy.toml"),
        "expected error message prefix, got: {msg}",
    );
}
