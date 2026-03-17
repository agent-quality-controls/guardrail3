//! Tests extracted from `app::rs::validate::test_quality_checks`
#![allow(
    clippy::expect_used,
    clippy::disallowed_methods,
    clippy::indexing_slicing,
    clippy::panic,
    clippy::manual_assert
)] // reason: test crate

use std::fs as stdfs;

use guardrail3::app::rs::validate::test_quality_checks::{
    check_integration_tests, check_mutation_hook, check_test_coverage_inventory, count_pub_fns,
    count_test_fns, find_ignore_without_reason,
};
use guardrail3::domain::report::Severity;

fn make_temp_dir() -> tempfile::TempDir {
    tempfile::tempdir().expect("failed to create temp dir")
}

// ---- R-TEST-05: Test coverage inventory ----

#[test]
fn r_test_05_counts_pub_fns() {
    let content = "pub fn foo() {}\nfn bar() {}\npub fn baz() {}";
    assert_eq!(count_pub_fns(content), 2);
}

#[test]
fn r_test_05_counts_test_fns() {
    let content = "#[test]\nfn a() {}\n#[test]\nfn b() {}\nfn c() {}";
    assert_eq!(count_test_fns(content), 2);
}

#[test]
fn r_test_05_emits_inventory() {
    let fs = guardrail3::adapters::outbound::fs::RealFileSystem;
    let tmp = make_temp_dir();
    let src_dir = tmp.path().join("src");
    stdfs::create_dir_all(&src_dir).expect("mkdir");
    stdfs::write(
        src_dir.join("lib.rs"),
        "pub fn foo() {}\npub fn bar() {}\n#[test]\nfn test_foo() {}",
    )
    .expect("write");
    let mut results = Vec::new();
    check_test_coverage_inventory(&fs, tmp.path(), &mut results);
    assert_eq!(results.len(), 1);
    let r = results.first().expect("should have result");
    assert_eq!(r.id, "R-TEST-05");
    assert_eq!(r.severity, Severity::Info);
    assert!(r.message.contains("2 public functions"));
    assert!(r.message.contains("1 test functions"));
}

// ---- R-TEST-06: Integration tests exist ----

#[test]
fn r_test_06_neg_no_tests_dir() {
    let fs = guardrail3::adapters::outbound::fs::RealFileSystem;
    let tmp = make_temp_dir();
    let mut results = Vec::new();
    check_integration_tests(&fs, tmp.path(), &mut results);
    assert_eq!(results.len(), 1);
    let r = results.first().expect("should have result");
    assert_eq!(r.id, "R-TEST-06");
    assert!(r.title.contains("No integration"));
}

#[test]
fn r_test_06_pos_tests_dir_with_rs() {
    let fs = guardrail3::adapters::outbound::fs::RealFileSystem;
    let tmp = make_temp_dir();
    let tests_dir = tmp.path().join("tests");
    stdfs::create_dir_all(&tests_dir).expect("mkdir");
    stdfs::write(tests_dir.join("integration.rs"), "#[test]\nfn it() {}").expect("write");
    let mut results = Vec::new();
    check_integration_tests(&fs, tmp.path(), &mut results);
    assert_eq!(results.len(), 1);
    let r = results.first().expect("should have result");
    assert_eq!(r.id, "R-TEST-06");
    assert!(r.title.contains("Integration tests exist"));
}

// ---- R-TEST-07: No #[ignore] without reason ----

#[test]
fn r_test_07_neg_bare_ignore() {
    let content = "#[test]\n#[ignore]\nfn slow_test() {}";
    let violations = find_ignore_without_reason(content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0], 2); // line 2, 1-based
}

#[test]
fn r_test_07_pos_ignore_with_reason_same_line() {
    let content = "#[test]\n#[ignore] // reason: requires network\nfn slow_test() {}";
    let violations = find_ignore_without_reason(content);
    assert!(violations.is_empty(), "Should accept reason on same line");
}

#[test]
fn r_test_07_pos_ignore_with_reason_prev_line() {
    let content = "#[test]\n// reason: requires database\n#[ignore]\nfn slow_test() {}";
    let violations = find_ignore_without_reason(content);
    assert!(
        violations.is_empty(),
        "Should accept reason on previous line"
    );
}

#[test]
fn r_test_07_pos_ignore_with_name_value_reason() {
    let content = "#[test]\n#[ignore = \"requires network\"]\nfn slow_test() {}";
    let violations = find_ignore_without_reason(content);
    assert!(
        violations.is_empty(),
        "ignore with = reason should not be flagged"
    );
}

// ---- R-TEST-08: Mutation test hook configured ----

#[test]
fn r_test_08_neg_no_hook() {
    let fs = guardrail3::adapters::outbound::fs::RealFileSystem;
    let tmp = make_temp_dir();
    let mut results = Vec::new();
    check_mutation_hook(&fs, tmp.path(), &mut results);
    assert_eq!(results.len(), 1);
    let r = results.first().expect("should have result");
    assert_eq!(r.id, "R-TEST-08");
    assert!(r.title.contains("No mutation"));
}

#[test]
fn r_test_08_pos_claude_hook() {
    let fs = guardrail3::adapters::outbound::fs::RealFileSystem;
    let tmp = make_temp_dir();
    let claude_dir = tmp.path().join(".claude");
    stdfs::create_dir_all(&claude_dir).expect("mkdir");
    stdfs::write(
        claude_dir.join("hooks.json"),
        r#"{"hooks": [{"command": "cargo-mutants --in-diff"}]}"#,
    )
    .expect("write");
    let mut results = Vec::new();
    check_mutation_hook(&fs, tmp.path(), &mut results);
    assert_eq!(results.len(), 1);
    let r = results.first().expect("should have result");
    assert_eq!(r.id, "R-TEST-08");
    assert!(r.title.contains("Mutation test hook configured"));
}

#[test]
fn r_test_08_pos_pre_commit_hook() {
    let fs = guardrail3::adapters::outbound::fs::RealFileSystem;
    let tmp = make_temp_dir();
    let hooks_dir = tmp.path().join(".git").join("hooks");
    stdfs::create_dir_all(&hooks_dir).expect("mkdir");
    stdfs::write(
        hooks_dir.join("pre-commit"),
        "#!/bin/bash\ncargo mutants --in-diff -\n",
    )
    .expect("write");
    let mut results = Vec::new();
    check_mutation_hook(&fs, tmp.path(), &mut results);
    assert_eq!(results.len(), 1);
    let r = results.first().expect("should have result");
    assert_eq!(r.id, "R-TEST-08");
    assert!(r.title.contains("Mutation test hook configured"));
}
