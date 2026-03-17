//! Tests extracted from `app::rs::validate::test_checks`
#![allow(
    clippy::expect_used,
    clippy::disallowed_methods,
    clippy::indexing_slicing,
    clippy::panic,
    clippy::manual_assert
)] // reason: test crate

use std::fs as stdfs;

use guardrail3::app::rs::validate::test_checks::{
    check_cargo_mutants_installed, check_mutants_toml, content_has_test, file_has_cfg_test_module,
    has_mutants_profile,
};
use guardrail3::domain::report::Severity;

fn make_temp_dir() -> tempfile::TempDir {
    tempfile::tempdir().expect("failed to create temp dir")
}

// ---- R-TEST-01: cargo-mutants installed ----

#[test]
fn r_test_01_detects_installed_tool() {
    let mut results = Vec::new();
    let tc = guardrail3::adapters::outbound::tool_runner::RealToolChecker;
    check_cargo_mutants_installed(&tc, &mut results);
    assert_eq!(results.len(), 1);
    assert_eq!(results.first().map(|r| r.id.as_str()), Some("R-TEST-01"));
}

#[test]
fn r_test_01_severity_matches_installation() {
    let mut results = Vec::new();
    let tc = guardrail3::adapters::outbound::tool_runner::RealToolChecker;
    check_cargo_mutants_installed(&tc, &mut results);
    let result = results.first().expect("should have one result");
    assert!(
        result.severity == Severity::Info || result.severity == Severity::Warn,
        "Severity must be Info (installed) or Warn (missing)"
    );
}

// ---- R-TEST-02: .cargo/mutants.toml exists ----

#[test]
fn r_test_02_neg_no_mutants_toml() {
    let tmp = make_temp_dir();
    let mut results = Vec::new();
    check_mutants_toml(tmp.path(), &mut results);
    assert_eq!(results.len(), 1);
    let r = results.first().expect("should have result");
    assert_eq!(r.id, "R-TEST-02");
    assert_eq!(r.severity, Severity::Warn);
}

#[test]
fn r_test_02_pos_mutants_toml_exists() {
    let tmp = make_temp_dir();
    let cargo_dir = tmp.path().join(".cargo");
    stdfs::create_dir_all(&cargo_dir).expect("mkdir");
    stdfs::write(cargo_dir.join("mutants.toml"), "profile = \"mutants\"").expect("write");
    let mut results = Vec::new();
    check_mutants_toml(tmp.path(), &mut results);
    assert_eq!(results.len(), 1);
    let r = results.first().expect("should have result");
    assert_eq!(r.id, "R-TEST-02");
    assert_eq!(r.severity, Severity::Info);
}

// ---- R-TEST-03: [profile.mutants] in Cargo.toml ----

#[test]
fn r_test_03_neg_no_profile() {
    let content = "[package]\nname = \"foo\"\nversion = \"0.1.0\"";
    assert!(!has_mutants_profile(content));
}

#[test]
fn r_test_03_pos_has_profile() {
    let content = "[package]\nname = \"foo\"\n\n[profile.mutants]\ninherits = \"test\"";
    assert!(has_mutants_profile(content));
}

// ---- R-TEST-04: At least one #[test] exists ----

#[test]
fn r_test_04_neg_no_test_attr() {
    let content = "fn main() {}\npub fn helper() {}";
    assert!(!content_has_test(content));
}

#[test]
fn r_test_04_pos_has_test_attr() {
    let content = "#[test]\nfn it_works() { assert!(true); }";
    assert!(content_has_test(content));
}

#[test]
fn r_test_04_pos_has_tokio_test() {
    let content = "#[tokio::test]\nasync fn it_works() {}";
    assert!(content_has_test(content));
}

// ---- R-TEST-09: No test code in production source ----

#[test]
fn r_test_09_detects_cfg_test_module() {
    let content =
        "fn production() {}\n\n#[cfg(test)]\nmod tests {\n    #[test]\n    fn it_works() {}\n}";
    let parsed = guardrail3::app::rs::validate::ast_helpers::parse_file(content);
    assert!(parsed.is_some());
    assert!(file_has_cfg_test_module(&parsed.expect("should parse")));
}

#[test]
fn r_test_09_no_cfg_test_is_clean() {
    let content = "fn production() {}";
    let parsed = guardrail3::app::rs::validate::ast_helpers::parse_file(content);
    assert!(parsed.is_some());
    assert!(!file_has_cfg_test_module(&parsed.expect("should parse")));
}
