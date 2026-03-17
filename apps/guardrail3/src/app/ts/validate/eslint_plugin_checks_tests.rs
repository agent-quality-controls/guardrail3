use super::*;
use std::path::PathBuf;

fn path() -> PathBuf {
    PathBuf::from("eslint.config.mjs")
}

#[test]
fn test_all_unicorn_disabled_present() {
    let content = UNICORN_DISABLED.join("\n");
    let missing = find_missing_rules(&content, UNICORN_DISABLED);
    assert!(missing.is_empty());
}

#[test]
fn test_unicorn_disabled_missing_one() {
    let rules: Vec<&str> = UNICORN_DISABLED.iter().skip(1).copied().collect();
    let content = rules.join("\n");
    let missing = find_missing_rules(&content, UNICORN_DISABLED);
    assert_eq!(missing, vec!["unicorn/no-null"]);
}

#[test]
fn test_core_plugins_all_pass() {
    let mut parts: Vec<&str> = vec![
        // T-ESLP-01
        "unicorn",
        "flat/recommended",
        // T-ESLP-04
        "regexp",
        // T-ESLP-11 test override marker
        "files: **/*.test.* overrides",
    ];
    // All rule lists
    parts.extend_from_slice(UNICORN_DISABLED);
    parts.extend_from_slice(UNICORN_EXTRA);
    parts.extend_from_slice(REGEXP_EXTRA);
    parts.extend_from_slice(SONARJS_RULES);
    parts.extend_from_slice(REACT_EXTRA);
    parts.extend_from_slice(BUILTIN_RULES);
    parts.extend_from_slice(TEST_RELAXATION_RULES);

    let content = parts.join("\n");
    let mut results = Vec::new();
    check_core_plugins(&content, &path(), &mut results);

    let errors: Vec<_> = results
        .iter()
        .filter(|r| r.severity == Severity::Error)
        .collect();
    assert!(errors.is_empty(), "unexpected errors: {errors:?}");
}

#[test]
fn test_content_plugins_all_pass() {
    let content = "jsxA11y strict\njsx-a11y/control-has-associated-label\ntailwind-ban";
    let mut results = Vec::new();
    check_content_plugins(content, &path(), &mut results);

    let errors: Vec<_> = results
        .iter()
        .filter(|r| r.severity == Severity::Error)
        .collect();
    assert!(errors.is_empty(), "unexpected errors: {errors:?}");
}

#[test]
fn test_content_plugins_all_missing() {
    let content = "// empty config";
    let mut results = Vec::new();
    check_content_plugins(content, &path(), &mut results);

    let error_count = results
        .iter()
        .filter(|r| r.severity == Severity::Error)
        .count();
    assert_eq!(error_count, 3, "expected 3 errors for T-ESLP-07/08/12");
}

#[test]
fn test_test_relaxation_missing_section() {
    let content = "// no test override";
    let mut results = Vec::new();
    check_test_relaxations(content, &path(), &mut results);
    assert_eq!(results.len(), 1);
    if let Some(first) = results.first() {
        assert_eq!(first.severity, Severity::Error);
        assert!(first.title.contains("missing"));
    }
}
