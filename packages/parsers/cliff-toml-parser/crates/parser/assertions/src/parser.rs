#![allow(
    clippy::expect_used,
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    reason = "assertion helpers are reusable panic-based proof sites for test harnesses"
)]

use cliff_toml_parser_runtime::{CliffCommitParser, CliffToml};

/// Assert that all top-level optional sections are `None`.
pub fn assert_sections_empty(cfg: &CliffToml) {
    assert!(cfg.git.is_none(), "git section should be None");
    assert!(cfg.changelog.is_none(), "changelog section should be None");
    assert!(cfg.extra.is_empty(), "extra should be empty");
}

/// Assert a boolean field matches an expected value.
pub fn assert_bool_field(actual: Option<bool>, expected: Option<bool>, field_name: &str) {
    assert_eq!(actual, expected, "{field_name} mismatch");
}

/// Assert a list has the expected length.
pub fn assert_list_len<T>(items: &[T], expected: usize, field_name: &str) {
    assert_eq!(items.len(), expected, "{field_name} count mismatch");
}

/// Assert a commit parser entry has the expected fields.
pub fn assert_commit_entry(
    parser: &CliffCommitParser,
    message: Option<&str>,
    group: Option<&str>,
    skip: Option<bool>,
) {
    assert_eq!(
        parser.message.as_deref(),
        message,
        "commit parser message mismatch",
    );
    assert_eq!(
        parser.group.as_deref(),
        group,
        "commit parser group mismatch",
    );
    assert_eq!(parser.skip, skip, "commit parser skip mismatch");
}

/// Assert that an error is a parse error (contains the expected prefix).
pub fn assert_parse_error(err: impl std::fmt::Display) {
    let msg = err.to_string();
    assert!(
        msg.contains("invalid cliff.toml"),
        "expected error message prefix, got: {msg}",
    );
}
