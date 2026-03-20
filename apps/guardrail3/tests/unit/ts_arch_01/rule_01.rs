use super::helpers::{
    assert_no_packages, assert_no_rust_apps, copy_fixture, remove_dir, run_check, write_file,
};
use guardrail3::domain::report::Severity;

// ============================================================================
// Rule 01: src/modules/ must exist for TS apps
//
// Note: `check_hex_arch_structure` (standalone) checks ALL TS apps found via
// `discover_ts_apps` — it does NOT filter by app type. Content site filtering
// only happens in the `check_hex_arch_structure_for_apps` variant which uses
// `TsAppContext`. The standalone function warns about ANY TS app missing
// src/modules/, including content sites like `landing`.
// ============================================================================

fn t_arch_01_warns(results: &[guardrail3::domain::report::CheckResult]) -> Vec<&guardrail3::domain::report::CheckResult> {
    results
        .iter()
        .filter(|r| r.id == "T-ARCH-01" && r.severity == Severity::Warn)
        .collect()
}

#[test]
fn golden_baseline_warns() {
    // Golden fixture: admin has src/modules/ (passes), landing has no src/modules/ (warns)
    let tmp = copy_fixture();
    let results = run_check(tmp.path());
    let warns = t_arch_01_warns(&results);
    // Landing is a TS app (has package.json) and has no src/modules/ → 1 warning
    assert_eq!(warns.len(), 1, "expected 1 baseline warning (landing), got {}: {warns:#?}", warns.len());
    assert!(
        warns[0].title.contains("landing"),
        "expected warning about 'landing', got: '{}'",
        warns[0].title
    );
}

#[test]
fn missing_modules_dir_admin() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules");
    let results = run_check(tmp.path());
    let warns = t_arch_01_warns(&results);
    // Now both admin and landing warn
    assert_eq!(warns.len(), 2, "expected 2 warnings (admin + landing), got {}: {warns:#?}", warns.len());
    assert!(
        warns.iter().any(|w| w.title.contains("admin")),
        "expected warning for 'admin', got: {warns:#?}"
    );
    assert!(
        warns.iter().any(|w| w.title.contains("landing")),
        "expected warning for 'landing', got: {warns:#?}"
    );
}

#[test]
fn missing_modules_triggers_warn_not_error() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules");
    let results = run_check(tmp.path());
    let admin_warns: Vec<_> = results
        .iter()
        .filter(|r| r.id == "T-ARCH-01" && r.severity == Severity::Warn && r.title.contains("admin"))
        .collect();
    assert_eq!(admin_warns.len(), 1, "expected Warn severity, got: {admin_warns:#?}");
    assert!(
        admin_warns[0].title.contains("missing src/modules/"),
        "expected 'missing src/modules/' in title, got: '{}'",
        admin_warns[0].title
    );
}

#[test]
fn rust_apps_not_checked() {
    let tmp = copy_fixture();
    let results = run_check(tmp.path());
    let all: Vec<_> = results.iter().filter(|r| r.id == "T-ARCH-01").collect();
    assert_no_rust_apps(&all);
}

#[test]
fn packages_not_checked() {
    let tmp = copy_fixture();
    let results = run_check(tmp.path());
    let all: Vec<_> = results.iter().filter(|r| r.id == "T-ARCH-01").collect();
    assert_no_packages(&all);
}

#[test]
fn new_ts_app_missing_modules() {
    let tmp = copy_fixture();
    // Create a new TS app with package.json but no src/modules/
    write_file(
        tmp.path(),
        "apps/dashboard/package.json",
        r#"{"name": "dashboard", "dependencies": {"express": "4.0"}}"#,
    );
    write_file(
        tmp.path(),
        "apps/dashboard/src/index.ts",
        "console.log('hello');",
    );
    let results = run_check(tmp.path());
    let warns = t_arch_01_warns(&results);
    assert!(
        warns.iter().any(|w| w.title.contains("dashboard")),
        "expected warning for new dashboard app, got: {warns:#?}"
    );
}

#[test]
fn app_with_modules_no_warning() {
    // Admin has src/modules/ — should NOT get a missing-modules warning
    let tmp = copy_fixture();
    let results = run_check(tmp.path());
    let warns = t_arch_01_warns(&results);
    assert!(
        !warns.iter().any(|w| w.title.contains("admin")),
        "admin has src/modules/ — should not be warned, got: {warns:#?}"
    );
}

#[test]
fn file_field_set_on_warnings() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules");
    let results = run_check(tmp.path());
    let warns = t_arch_01_warns(&results);
    for w in &warns {
        assert!(
            w.file.is_some(),
            "expected file field set on warning, got None: {w:#?}"
        );
    }
}
