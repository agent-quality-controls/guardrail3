#![allow(
    clippy::expect_used,
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    clippy::panic,
    reason = "assertion helpers are reusable panic-based proof sites for test harnesses"
)]

use cargo_toml_parser_runtime::{CargoToml, Dependency, LintValue, Value};

pub fn assert_manifest_empty(manifest: &CargoToml) {
    assert!(
        manifest.cargo_features.is_empty(),
        "cargo_features should be empty"
    );
    assert_eq!(manifest.package, None, "package should be None");
    assert_eq!(manifest.project, None, "project should be None");
    assert_eq!(manifest.workspace, None, "workspace should be None");
    assert!(manifest.badges.is_empty(), "badges should be empty");
    assert!(
        manifest.dependencies.is_empty(),
        "dependencies should be empty"
    );
    assert!(manifest.target.is_empty(), "target should be empty");
    assert!(manifest.profile.is_empty(), "profile should be empty");
    assert!(manifest.extra.is_empty(), "extra should be empty");
}

pub fn assert_simple_dep(actual: Option<&Dependency>, expected: &str, field_name: &str) {
    match actual {
        Some(Dependency::Simple(value)) => assert_eq!(value, expected, "{field_name} mismatch"),
        Some(Dependency::Detailed(_)) => panic!("{field_name} should be a simple dependency"),
        None => panic!("{field_name} should exist"),
    }
}

pub fn assert_detailed_dep_version(actual: Option<&Dependency>, expected: &str, field_name: &str) {
    match actual {
        Some(Dependency::Detailed(detail)) => {
            assert_eq!(
                detail.version.as_deref(),
                Some(expected),
                "{field_name}.version mismatch"
            );
        }
        Some(Dependency::Simple(_)) => panic!("{field_name} should be a detailed dependency"),
        None => panic!("{field_name} should exist"),
    }
}

pub fn assert_lint_level(actual: Option<&LintValue>, expected: &str, field_name: &str) {
    match actual {
        Some(LintValue::Level(level)) => assert_eq!(level, expected, "{field_name} mismatch"),
        Some(LintValue::Detailed(detail)) => {
            assert_eq!(detail.level, expected, "{field_name}.level mismatch");
        }
        None => panic!("{field_name} should exist"),
    }
}

pub fn assert_top_level_extra_string(manifest: &CargoToml, key: &str, expected: &str) {
    assert_eq!(
        manifest.extra.get(key).and_then(Value::as_str),
        Some(expected),
        "top-level extra string mismatch",
    );
}

pub fn assert_parse_error(err: impl std::fmt::Display) {
    let msg = err.to_string();
    assert!(
        msg.contains("invalid Cargo.toml"),
        "expected Cargo.toml parse error prefix, got: {msg}",
    );
}
