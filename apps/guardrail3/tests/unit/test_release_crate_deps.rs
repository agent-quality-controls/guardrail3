use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use guardrail3::app::rs::validate::release_checks::CrateInfo;
use guardrail3::app::rs::validate::release_crate_deps::{
    check_categories, check_keywords, check_path_deps, check_version_consistency, is_valid_semver,
    version_satisfies,
};
use guardrail3::domain::report::Severity;

fn parse_toml(content: &str) -> Option<toml::Value> {
    content.parse().ok()
}

// --- R-PUB-09 ---

#[test]
#[allow(clippy::expect_used)] // reason: test assertion
fn pub09_not_run_without_thorough() {
    let fs = guardrail3::adapters::outbound::fs::RealFileSystem;
    let t = parse_toml(
        "[package]\nname = \"x\"\nversion = \"0.1.0\"\ndescription = \"d\"\nlicense = \"MIT\"\nrepository = \"https://x\"",
    ).expect("parse"); // reason: test
    let tmp = std::env::temp_dir().join("guardrail3_pub09");
    let krate = CrateInfo {
        name: "x".to_owned(),
        cargo_toml_path: tmp.join("Cargo.toml"),
        dir: tmp,
        publishable: true,
        is_binary: false,
        table: t,
    };
    let names = BTreeSet::new();
    let versions = BTreeMap::new();
    let mut r = Vec::new();
    // Call check_per_crate to verify thorough=false skips R-PUB-09
    let tc = guardrail3::adapters::outbound::tool_runner::RealToolChecker;
    guardrail3::app::rs::validate::release_crate_checks::check_per_crate(
        &fs, &tc, &krate, &names, &versions, false, &mut r,
    );
    assert!(
        !r.iter().any(|c| c.id == "R-PUB-09"),
        "R-PUB-09 should not run without thorough"
    );
}

// --- R-PUB-10 ---

#[test]
#[allow(clippy::expect_used)] // reason: test assertion
fn pub10_neg_path_to_unpublishable() {
    let t = parse_toml(
        "[package]\nname = \"a\"\n[dependencies]\nunpub-crate = { path = \"../unpub\" }",
    )
    .expect("parse"); // reason: test
    let publishable = BTreeSet::from(["a".to_owned()]);
    let krate = CrateInfo {
        name: "a".to_owned(),
        cargo_toml_path: PathBuf::from("Cargo.toml"),
        dir: PathBuf::from("."),
        publishable: true,
        is_binary: false,
        table: t.clone(),
    };
    let mut r = Vec::new();
    check_path_deps(&t, &krate, &publishable, None, &mut r);
    assert!(
        r.iter()
            .any(|c| c.id == "R-PUB-10" && c.severity == Severity::Error),
        "expected Error"
    );
}

#[test]
#[allow(clippy::expect_used)] // reason: test assertion
fn pub10_pos_dev_deps_exempt() {
    let t = parse_toml(
        "[package]\nname = \"a\"\n[dev-dependencies]\ntest-util = { path = \"../util\" }",
    )
    .expect("parse"); // reason: test
    let publishable = BTreeSet::from(["a".to_owned()]);
    let krate = CrateInfo {
        name: "a".to_owned(),
        cargo_toml_path: PathBuf::from("Cargo.toml"),
        dir: PathBuf::from("."),
        publishable: true,
        is_binary: false,
        table: t.clone(),
    };
    let mut r = Vec::new();
    check_path_deps(&t, &krate, &publishable, None, &mut r);
    assert!(
        !r.iter()
            .any(|c| c.id == "R-PUB-10" && c.severity == Severity::Error),
        "dev-deps exempt"
    );
}

#[test]
#[allow(clippy::expect_used)] // reason: test assertion
fn pub10_pos_copublished_member() {
    let t = parse_toml(
        "[package]\nname = \"a\"\n[dependencies]\nb = { path = \"../b\", version = \"0.1\" }",
    )
    .expect("parse"); // reason: test
    let publishable = BTreeSet::from(["a".to_owned(), "b".to_owned()]);
    let krate = CrateInfo {
        name: "a".to_owned(),
        cargo_toml_path: PathBuf::from("Cargo.toml"),
        dir: PathBuf::from("."),
        publishable: true,
        is_binary: false,
        table: t.clone(),
    };
    let mut r = Vec::new();
    check_path_deps(&t, &krate, &publishable, None, &mut r);
    assert!(
        !r.iter()
            .any(|c| c.id == "R-PUB-10" && c.severity == Severity::Error),
        "copublished OK"
    );
}

// --- R-PUB-11 ---

#[test]
#[allow(clippy::expect_used)] // reason: test assertion
fn pub11_neg_version_mismatch() {
    let t = parse_toml(
        "[package]\nname = \"a\"\nversion = \"0.1.0\"\n[dependencies]\nb = { path = \"../b\", version = \"0.2\" }",
    ).expect("parse"); // reason: test
    let versions = BTreeMap::from([
        ("a".to_owned(), "0.1.0".to_owned()),
        ("b".to_owned(), "0.1.0".to_owned()),
    ]);
    let krate = CrateInfo {
        name: "a".to_owned(),
        cargo_toml_path: PathBuf::from("Cargo.toml"),
        dir: PathBuf::from("."),
        publishable: true,
        is_binary: false,
        table: t.clone(),
    };
    let mut r = Vec::new();
    check_version_consistency(&t, &krate, &versions, None, &mut r);
    assert!(
        r.iter()
            .any(|c| c.id == "R-PUB-11" && c.severity == Severity::Error),
        "expected Error"
    );
}

#[test]
#[allow(clippy::expect_used)] // reason: test assertion
fn pub11_pos_compatible_version() {
    let t = parse_toml(
        "[package]\nname = \"a\"\nversion = \"0.1.0\"\n[dependencies]\nb = { path = \"../b\", version = \"0.1\" }",
    ).expect("parse"); // reason: test
    let versions = BTreeMap::from([
        ("a".to_owned(), "0.1.0".to_owned()),
        ("b".to_owned(), "0.1.0".to_owned()),
    ]);
    let krate = CrateInfo {
        name: "a".to_owned(),
        cargo_toml_path: PathBuf::from("Cargo.toml"),
        dir: PathBuf::from("."),
        publishable: true,
        is_binary: false,
        table: t.clone(),
    };
    let mut r = Vec::new();
    check_version_consistency(&t, &krate, &versions, None, &mut r);
    assert!(
        !r.iter()
            .any(|c| c.id == "R-PUB-11" && c.severity == Severity::Error),
        "should not Error"
    );
}

// --- R-PUB-06 ---

#[test]
#[allow(clippy::expect_used)] // reason: test assertion
fn pub06_neg_missing_keywords() {
    let t = parse_toml("[package]\nname = \"x\"").expect("parse"); // reason: test
    let pkg = t.get("package");
    let mut r = Vec::new();
    check_keywords(pkg, "x", None, &mut r);
    assert!(
        r.iter()
            .any(|c| c.id == "R-PUB-06" && c.severity == Severity::Warn),
        "expected Warn"
    );
}

#[test]
#[allow(clippy::expect_used)] // reason: test assertion
fn pub06_neg_too_many() {
    let t =
        parse_toml("[package]\nname = \"x\"\nkeywords = [\"a\",\"b\",\"c\",\"d\",\"e\",\"f\"]")
            .expect("parse"); // reason: test
    let pkg = t.get("package");
    let mut r = Vec::new();
    check_keywords(pkg, "x", None, &mut r);
    assert!(
        r.iter()
            .any(|c| c.id == "R-PUB-06" && c.severity == Severity::Warn),
        "expected Warn"
    );
}

#[test]
#[allow(clippy::expect_used)] // reason: test assertion
fn pub06_pos_good_keywords() {
    let t =
        parse_toml("[package]\nname = \"x\"\nkeywords = [\"a\",\"b\",\"c\"]").expect("parse"); // reason: test
    let pkg = t.get("package");
    let mut r = Vec::new();
    check_keywords(pkg, "x", None, &mut r);
    assert!(
        r.iter()
            .any(|c| c.id == "R-PUB-06" && c.severity == Severity::Info),
        "expected Info"
    );
}

// --- R-PUB-07 ---

#[test]
#[allow(clippy::expect_used)] // reason: test assertion
fn pub07_neg_no_categories() {
    let t = parse_toml("[package]\nname = \"x\"").expect("parse"); // reason: test
    let pkg = t.get("package");
    let mut r = Vec::new();
    check_categories(pkg, "x", None, &mut r);
    assert!(
        r.iter()
            .any(|c| c.id == "R-PUB-07" && c.severity == Severity::Warn),
        "expected Warn"
    );
}

#[test]
#[allow(clippy::expect_used)] // reason: test assertion
fn pub07_pos_has_categories() {
    let t = parse_toml("[package]\nname = \"x\"\ncategories = [\"development-tools\"]")
        .expect("parse"); // reason: test
    let pkg = t.get("package");
    let mut r = Vec::new();
    check_categories(pkg, "x", None, &mut r);
    assert!(
        r.iter()
            .any(|c| c.id == "R-PUB-07" && c.severity == Severity::Info),
        "expected Info"
    );
}

// --- Semver helper tests ---

#[test]
fn semver_valid_basic() {
    assert!(is_valid_semver("1.2.3"), "1.2.3 should be valid");
    assert!(is_valid_semver("0.0.0"), "0.0.0 should be valid");
    assert!(
        is_valid_semver("1.2.3-beta.1"),
        "prerelease should be valid"
    );
}

#[test]
fn semver_invalid() {
    assert!(!is_valid_semver("not-a-version"), "not-a-version invalid");
    assert!(!is_valid_semver("1.2"), "1.2 invalid");
    assert!(!is_valid_semver("1"), "1 invalid");
    assert!(!is_valid_semver(""), "empty invalid");
}

#[test]
fn version_satisfies_caret() {
    assert!(version_satisfies("1.2.3", "1.0"), "1.2.3 satisfies ^1.0");
    assert!(
        version_satisfies("1.2.3", "^1.0.0"),
        "1.2.3 satisfies ^1.0.0"
    );
    assert!(!version_satisfies("0.1.0", "0.2"), "0.1.0 not satisfy ^0.2");
    assert!(
        version_satisfies("0.1.5", "0.1.0"),
        "0.1.5 satisfies ^0.1.0"
    );
}

#[test]
fn version_satisfies_tilde() {
    assert!(
        version_satisfies("1.2.5", "~1.2.3"),
        "1.2.5 satisfies ~1.2.3"
    );
    assert!(
        !version_satisfies("1.3.0", "~1.2.3"),
        "1.3.0 not satisfy ~1.2.3"
    );
}
