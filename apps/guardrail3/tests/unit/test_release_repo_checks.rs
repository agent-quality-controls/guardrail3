use std::collections::BTreeSet;

use guardrail3::app::rs::validate::release_repo_checks::{
    check_cliff_toml, check_license_file, check_release_plz_toml, check_semver_checks_installed,
    check_workflow_contains,
};
use guardrail3::domain::report::Severity;

// --- R-REL-01 ---

#[test]
#[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
fn rel01_neg_no_license() {
    let tmp = std::env::temp_dir().join("guardrail3_rel01_neg");
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(&tmp);

    let mut r = Vec::new();
    check_license_file(&tmp, &mut r);
    assert!(
        r.iter()
            .any(|c| c.id == "R-REL-01" && c.severity == Severity::Error)
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
#[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
fn rel01_pos_license_exists() {
    let tmp = std::env::temp_dir().join("guardrail3_rel01_pos");
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(&tmp);
    let _ = std::fs::write(tmp.join("LICENSE"), "MIT License");

    let mut r = Vec::new();
    check_license_file(&tmp, &mut r);
    assert!(
        r.iter()
            .any(|c| c.id == "R-REL-01" && c.severity == Severity::Info)
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

// --- R-REL-02 ---

#[test]
#[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
fn rel02_neg_no_release_plz() {
    let tmp = std::env::temp_dir().join("guardrail3_rel02_neg");
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(&tmp);

    let mut r = Vec::new();
    let names = BTreeSet::new();
    let fs = guardrail3::adapters::outbound::fs::RealFileSystem;
    check_release_plz_toml(&fs, &tmp, &names, &mut r);
    assert!(
        r.iter()
            .any(|c| c.id == "R-REL-02" && c.severity == Severity::Warn)
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
#[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
fn rel02_pos_release_plz_exists() {
    let tmp = std::env::temp_dir().join("guardrail3_rel02_pos");
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(&tmp);
    let _ = std::fs::write(
        tmp.join("release-plz.toml"),
        "[workspace]\n[[package]]\nname = \"x\"\n",
    );

    let mut r = Vec::new();
    let names = BTreeSet::from(["x".to_owned()]);
    let fs = guardrail3::adapters::outbound::fs::RealFileSystem;
    check_release_plz_toml(&fs, &tmp, &names, &mut r);
    assert!(
        r.iter()
            .any(|c| c.id == "R-REL-02" && c.severity == Severity::Info)
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

// --- R-REL-03 ---

#[test]
#[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
fn rel03_neg_invalid_toml() {
    let tmp = std::env::temp_dir().join("guardrail3_rel03_neg");
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(&tmp);
    let _ = std::fs::write(tmp.join("release-plz.toml"), "not valid toml [[[");

    let mut r = Vec::new();
    let names = BTreeSet::new();
    let fs = guardrail3::adapters::outbound::fs::RealFileSystem;
    check_release_plz_toml(&fs, &tmp, &names, &mut r);
    assert!(
        r.iter()
            .any(|c| c.id == "R-REL-03" && c.severity == Severity::Warn)
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
#[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
fn rel03_pos_valid_covers_crates() {
    let tmp = std::env::temp_dir().join("guardrail3_rel03_pos");
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(&tmp);
    let _ = std::fs::write(
        tmp.join("release-plz.toml"),
        "[workspace]\n\n[[package]]\nname = \"a\"\n\n[[package]]\nname = \"b\"\n",
    );

    let mut r = Vec::new();
    let names = BTreeSet::from(["a".to_owned(), "b".to_owned()]);
    let fs = guardrail3::adapters::outbound::fs::RealFileSystem;
    check_release_plz_toml(&fs, &tmp, &names, &mut r);
    assert!(
        r.iter()
            .any(|c| c.id == "R-REL-03" && c.severity == Severity::Info)
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

// --- R-REL-04 ---

#[test]
#[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
fn rel04_neg_no_cliff() {
    let tmp = std::env::temp_dir().join("guardrail3_rel04_neg");
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(&tmp);

    let mut r = Vec::new();
    check_cliff_toml(&tmp, &mut r);
    assert!(
        r.iter()
            .any(|c| c.id == "R-REL-04" && c.severity == Severity::Warn)
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
#[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
fn rel04_pos_cliff_exists() {
    let tmp = std::env::temp_dir().join("guardrail3_rel04_pos");
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(&tmp);
    let _ = std::fs::write(tmp.join("cliff.toml"), "[changelog]\nheader = \"\"");

    let mut r = Vec::new();
    check_cliff_toml(&tmp, &mut r);
    assert!(
        r.iter()
            .any(|c| c.id == "R-REL-04" && c.severity == Severity::Info)
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

// --- R-REL-05 ---

#[test]
#[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
fn rel05_neg_no_release_workflow() {
    let tmp = std::env::temp_dir().join("guardrail3_rel05_neg");
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(tmp.join(".github/workflows"));
    let _ = std::fs::write(tmp.join(".github/workflows/ci.yml"), "name: CI\n");

    let mut r = Vec::new();
    let fs = guardrail3::adapters::outbound::fs::RealFileSystem;
    check_workflow_contains(&fs, &tmp, "release-plz", "R-REL-05", "", "", "", "", &mut r);
    assert!(
        r.iter()
            .any(|c| c.id == "R-REL-05" && c.severity == Severity::Warn)
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
#[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
fn rel05_pos_has_release_workflow() {
    let tmp = std::env::temp_dir().join("guardrail3_rel05_pos");
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(tmp.join(".github/workflows"));
    let _ = std::fs::write(
        tmp.join(".github/workflows/release.yml"),
        "name: Release\nuses: release-plz/action@v0.5\n",
    );

    let mut r = Vec::new();
    let fs = guardrail3::adapters::outbound::fs::RealFileSystem;
    check_workflow_contains(&fs, &tmp, "release-plz", "R-REL-05", "", "", "", "", &mut r);
    assert!(
        r.iter()
            .any(|c| c.id == "R-REL-05" && c.severity == Severity::Info)
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

// --- R-REL-06 ---

#[test]
#[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
fn rel06_neg_no_dry_run() {
    let tmp = std::env::temp_dir().join("guardrail3_rel06_neg");
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(tmp.join(".github/workflows"));
    let _ = std::fs::write(
        tmp.join(".github/workflows/ci.yml"),
        "name: CI\ncargo test\n",
    );

    let mut r = Vec::new();
    let fs = guardrail3::adapters::outbound::fs::RealFileSystem;
    check_workflow_contains(
        &fs,
        &tmp,
        "cargo publish --dry-run",
        "R-REL-06",
        "",
        "",
        "",
        "",
        &mut r,
    );
    assert!(
        r.iter()
            .any(|c| c.id == "R-REL-06" && c.severity == Severity::Warn)
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
#[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
fn rel06_pos_has_dry_run() {
    let tmp = std::env::temp_dir().join("guardrail3_rel06_pos");
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(tmp.join(".github/workflows"));
    let _ = std::fs::write(
        tmp.join(".github/workflows/ci.yml"),
        "name: CI\nrun: cargo publish --dry-run\n",
    );

    let mut r = Vec::new();
    let fs = guardrail3::adapters::outbound::fs::RealFileSystem;
    check_workflow_contains(
        &fs,
        &tmp,
        "cargo publish --dry-run",
        "R-REL-06",
        "",
        "",
        "",
        "",
        &mut r,
    );
    assert!(
        r.iter()
            .any(|c| c.id == "R-REL-06" && c.severity == Severity::Info)
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

// --- R-REL-07 ---

#[test]
#[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
fn rel07_neg_no_token() {
    let tmp = std::env::temp_dir().join("guardrail3_rel07_neg");
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(tmp.join(".github/workflows"));
    let _ = std::fs::write(tmp.join(".github/workflows/ci.yml"), "name: CI\n");

    let mut r = Vec::new();
    let fs = guardrail3::adapters::outbound::fs::RealFileSystem;
    check_workflow_contains(
        &fs,
        &tmp,
        "CARGO_REGISTRY_TOKEN",
        "R-REL-07",
        "",
        "",
        "",
        "",
        &mut r,
    );
    assert!(
        r.iter()
            .any(|c| c.id == "R-REL-07" && c.severity == Severity::Warn)
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
#[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
fn rel07_pos_has_token() {
    let tmp = std::env::temp_dir().join("guardrail3_rel07_pos");
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(tmp.join(".github/workflows"));
    let _ = std::fs::write(
        tmp.join(".github/workflows/release.yml"),
        "env:\n  CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}\n",
    );

    let mut r = Vec::new();
    let fs = guardrail3::adapters::outbound::fs::RealFileSystem;
    check_workflow_contains(
        &fs,
        &tmp,
        "CARGO_REGISTRY_TOKEN",
        "R-REL-07",
        "",
        "",
        "",
        "",
        &mut r,
    );
    assert!(
        r.iter()
            .any(|c| c.id == "R-REL-07" && c.severity == Severity::Info)
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

// --- R-REL-08: cargo-semver-checks installed (runtime check, test structure only) ---

#[test]
fn rel08_emits_result() {
    let mut r = Vec::new();
    let tc = guardrail3::adapters::outbound::tool_runner::RealToolChecker;
    check_semver_checks_installed(&tc, &mut r);
    // Should emit exactly one result with id R-REL-08
    assert_eq!(r.len(), 1);
    assert_eq!(r.first().map(|c| c.id.as_str()), Some("R-REL-08"));
}
