#![allow(
    clippy::expect_used,
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    reason = "assertion helpers are reusable panic-based proof sites for test harnesses"
)]

use cargo_toml_parser_runtime::types::{Dependency, LintValue};

pub fn assert_simple_dep(actual: Option<&Dependency>, expected: &str, field_name: &str) {
    assert!(actual.is_some(), "{field_name} should exist");
    assert!(
        matches!(actual, Some(Dependency::Simple(_))),
        "{field_name} should be a simple dependency",
    );
    let Some(Dependency::Simple(value)) = actual else {
        return;
    };
    assert_eq!(value, expected, "{field_name} mismatch");
}

pub fn assert_detailed_dep_version(actual: Option<&Dependency>, expected: &str, field_name: &str) {
    assert!(actual.is_some(), "{field_name} should exist");
    assert!(
        matches!(actual, Some(Dependency::Detailed(_))),
        "{field_name} should be a detailed dependency",
    );
    let Some(Dependency::Detailed(detail)) = actual else {
        return;
    };
    assert_eq!(
        detail.version.as_deref(),
        Some(expected),
        "{field_name}.version mismatch"
    );
}

pub fn assert_lint_level(actual: Option<&LintValue>, expected: &str, field_name: &str) {
    assert!(actual.is_some(), "{field_name} should exist");
    match actual {
        Some(LintValue::Level(level)) => assert_eq!(level, expected, "{field_name} mismatch"),
        Some(LintValue::Detailed(detail)) => {
            assert_eq!(detail.level, expected, "{field_name}.level mismatch");
        }
        None => {}
    }
}
