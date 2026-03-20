use super::helpers::{
    arch_errors, assert_no_packages, assert_no_rust_apps, copy_fixture, remove_dir, run_check,
    write_file,
};
use guardrail3::domain::report::{CheckResult, Severity};

// ============================================================================
// Rule 01: src/modules/ must exist for TS apps
//
// `check_hex_arch_structure` (standalone) checks ALL TS apps found via
// `discover_ts_apps` — it does NOT filter by app type. Content site filtering
// only happens in `check_hex_arch_structure_for_apps` (TsAppContext variant).
//
// discover_ts_apps: scans apps/ for subdirs with package.json OR .ts/.tsx files
// check_single_app_structure: if src/modules/ missing → Severity::Warn
// ============================================================================

const TS_APPS: &[&str] = &["admin", "portal", "landing"];

fn t_arch_01_warns(results: &[CheckResult]) -> Vec<&CheckResult> {
    results
        .iter()
        .filter(|r| r.id == "T-ARCH-01" && r.severity == Severity::Warn)
        .collect()
}

fn assert_warn_for_app(warns: &[&CheckResult], app: &str) {
    let app_warn = warns
        .iter()
        .find(|w| w.title.contains(app))
        .unwrap_or_else(|| panic!("expected warning for '{app}', got: {warns:#?}"));
    assert!(
        app_warn.title.contains("missing src/modules/"),
        "expected 'missing src/modules/' in title for '{app}', got: '{}'",
        app_warn.title
    );
    let file = app_warn.file.as_deref().unwrap_or("");
    assert!(
        file.contains(&format!("apps/{app}")),
        "expected file field containing 'apps/{app}', got: '{file}'"
    );
}

// ============================================================================
// GROUP A: Golden baseline
// ============================================================================

#[test]
fn golden_baseline_warns() {
    let tmp = copy_fixture();
    let results = run_check(tmp.path());
    let warns = t_arch_01_warns(&results);
    // Landing has package.json but no src/modules/ → 1 warning
    assert_eq!(
        warns.len(),
        1,
        "expected 1 baseline warning (landing), got {}: {warns:#?}",
        warns.len()
    );
    assert_warn_for_app(&warns, "landing");
    // Admin has src/modules/ — must NOT be warned
    assert!(
        !warns.iter().any(|w| w.title.contains("admin")),
        "admin has src/modules/ — should not be warned, got: {warns:#?}"
    );
    // No Error-severity results for T-ARCH-01 in golden
    let errors = arch_errors(&results);
    assert!(
        errors.is_empty(),
        "golden should have 0 T-ARCH-01 errors, got: {errors:#?}"
    );
}

#[test]
fn golden_no_rust_apps_flagged() {
    let tmp = copy_fixture();
    let results = run_check(tmp.path());
    let all: Vec<_> = results.iter().filter(|r| r.id == "T-ARCH-01").collect();
    assert_no_rust_apps(&all);
}

#[test]
fn golden_no_packages_flagged() {
    let tmp = copy_fixture();
    let results = run_check(tmp.path());
    let all: Vec<_> = results.iter().filter(|r| r.id == "T-ARCH-01").collect();
    assert_no_packages(&all);
}

// ============================================================================
// GROUP B: Missing src/modules/ — single app
// ============================================================================

#[test]
fn missing_modules_dir_admin_only() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules");
    let results = run_check(tmp.path());
    let warns = t_arch_01_warns(&results);
    // admin + landing both warn
    assert_eq!(
        warns.len(),
        2,
        "expected 2 warnings (admin + landing), got {}: {warns:#?}",
        warns.len()
    );
    assert_warn_for_app(&warns, "admin");
    assert_warn_for_app(&warns, "landing");
    // Rust apps and packages still clean
    assert_no_rust_apps(&warns);
    assert_no_packages(&warns);
}

#[test]
fn missing_modules_is_warn_not_error() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules");
    let results = run_check(tmp.path());
    // Must be Warn, not Error
    let admin_warns: Vec<_> = results
        .iter()
        .filter(|r| r.id == "T-ARCH-01" && r.severity == Severity::Warn && r.title.contains("admin"))
        .collect();
    assert_eq!(admin_warns.len(), 1, "expected 1 Warn for admin: {admin_warns:#?}");
    // Must NOT have Error for same condition
    let admin_errors: Vec<_> = results
        .iter()
        .filter(|r| {
            r.id == "T-ARCH-01"
                && r.severity == Severity::Error
                && r.title.contains("admin")
                && r.title.contains("missing src/modules/")
        })
        .collect();
    assert!(
        admin_errors.is_empty(),
        "missing src/modules/ should be Warn, not Error, got: {admin_errors:#?}"
    );
}

// ============================================================================
// GROUP C: All TS apps broken simultaneously
// ============================================================================

#[test]
fn all_ts_apps_missing_modules() {
    let tmp = copy_fixture();
    // Remove modules/ from ALL TS service apps — landing already has no modules
    remove_dir(tmp.path(), "apps/admin/src/modules");
    remove_dir(tmp.path(), "apps/portal/src/modules");
    let results = run_check(tmp.path());
    let warns = t_arch_01_warns(&results);
    assert_eq!(
        warns.len(),
        3,
        "expected 3 warnings (admin + portal + landing), got {}: {warns:#?}",
        warns.len()
    );
    // Per-app attribution: each TS app must appear
    for app in TS_APPS {
        assert_warn_for_app(&warns, app);
    }
    assert_no_rust_apps(&warns);
    assert_no_packages(&warns);
}

#[test]
fn different_failure_modes_across_apps() {
    let tmp = copy_fixture();
    // admin: modules/ is a file (not a dir)
    remove_dir(tmp.path(), "apps/admin/src/modules");
    write_file(tmp.path(), "apps/admin/src/modules", "not a directory");
    // landing: already missing modules/ (baseline)
    // New app: modules/ empty
    write_file(
        tmp.path(),
        "apps/api/package.json",
        r#"{"name": "api"}"#,
    );
    std::fs::create_dir_all(tmp.path().join("apps/api/src/modules")).expect("mkdir");
    let results = run_check(tmp.path());
    let warns = t_arch_01_warns(&results);
    // Only landing warns (missing modules/). admin has modules-as-file (metadata succeeds → no warn).
    // api has empty modules/ (metadata succeeds → no warn).
    assert_eq!(
        warns.len(),
        1,
        "expected 1 warning (landing only), got {}: {warns:#?}",
        warns.len()
    );
    assert_warn_for_app(&warns, "landing");
    // admin should NOT have a rule_01 warn (modules-as-file passes metadata)
    assert!(
        !warns.iter().any(|w| w.title.contains("admin")),
        "admin modules-as-file should not produce rule_01 warn, got: {warns:#?}"
    );
    // api should NOT have a rule_01 warn (empty modules/ passes metadata)
    assert!(
        !warns.iter().any(|w| w.title.contains("api")),
        "api empty modules/ should not produce rule_01 warn, got: {warns:#?}"
    );
    // But admin and api should have rule_02+ errors (cascade)
    let errors = arch_errors(&results);
    assert!(
        errors.iter().any(|e| e.title.contains("admin")),
        "admin should have rule_02+ cascade errors, got: {errors:#?}"
    );
    assert!(
        errors.iter().any(|e| e.title.contains("api")),
        "api should have rule_02+ cascade errors, got: {errors:#?}"
    );
}

// ============================================================================
// GROUP D: modules/ is not a directory
// ============================================================================

#[test]
fn modules_is_file_not_dir() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules");
    write_file(tmp.path(), "apps/admin/src/modules", "not a directory");
    let results = run_check(tmp.path());
    // metadata() returns Some for files, so check proceeds to check_ts_modules_dir
    // which calls list_ts_dir_names (std::fs::read_dir on a file → empty → missing dirs)
    // This means we get rule_02 errors (missing domain, ports, etc.), NOT a rule_01 warn
    let warns = t_arch_01_warns(&results);
    let admin_warns: Vec<_> = warns.iter().filter(|w| w.title.contains("admin")).collect();
    assert_eq!(
        admin_warns.len(),
        0,
        "modules-as-file should not produce 'missing' warning (metadata succeeds), got: {admin_warns:#?}"
    );
    // Cascades to rule_02: 4 missing required dirs (domain, ports, application, adapters)
    let errors = arch_errors(&results);
    let admin_errors: Vec<_> = errors.iter().filter(|e| e.title.contains("admin")).collect();
    assert_eq!(
        admin_errors.len(),
        4,
        "modules-as-file should produce 4 rule_02 errors (missing dirs), got {}: {admin_errors:#?}",
        admin_errors.len()
    );
}

#[test]
fn modules_dir_empty() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules");
    std::fs::create_dir_all(tmp.path().join("apps/admin/src/modules")).expect("create empty modules");
    let results = run_check(tmp.path());
    // Empty dir: metadata() succeeds → no rule_01 warn
    let warns = t_arch_01_warns(&results);
    let admin_warns: Vec<_> = warns.iter().filter(|w| w.title.contains("admin")).collect();
    assert_eq!(
        admin_warns.len(),
        0,
        "empty modules/ should not produce 'missing' warning, got: {admin_warns:#?}"
    );
    // Cascades to rule_02: missing domain, ports, application, adapters
    let errors = arch_errors(&results);
    let admin_errors: Vec<_> = errors.iter().filter(|e| e.title.contains("admin")).collect();
    assert_eq!(
        admin_errors.len(),
        4,
        "expected 4 missing-dir errors (domain, ports, application, adapters), got {}: {admin_errors:#?}",
        admin_errors.len()
    );
}

#[test]
fn modules_with_only_gitkeep() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules");
    write_file(tmp.path(), "apps/admin/src/modules/.gitkeep", "");
    let results = run_check(tmp.path());
    // .gitkeep: metadata succeeds → no rule_01 warn
    let warns = t_arch_01_warns(&results);
    let admin_warns: Vec<_> = warns.iter().filter(|w| w.title.contains("admin")).collect();
    assert_eq!(
        admin_warns.len(),
        0,
        "modules/ with .gitkeep should not produce 'missing' warning, got: {admin_warns:#?}"
    );
    // Cascades to rule_02: missing 4 required dirs (domain, ports, application, adapters)
    // .gitkeep is explicitly allowed in structural dirs, so no loose-file error
    let errors = arch_errors(&results);
    let admin_errors: Vec<_> = errors.iter().filter(|e| e.title.contains("admin")).collect();
    assert_eq!(
        admin_errors.len(),
        4,
        "expected 4 errors (missing domain, ports, application, adapters), got {}: {admin_errors:#?}",
        admin_errors.len()
    );
}

#[test]
fn no_src_dir_at_all() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src");
    let results = run_check(tmp.path());
    // No src/ means no src/modules/ → warns
    let warns = t_arch_01_warns(&results);
    let admin_warns: Vec<_> = warns.iter().filter(|w| w.title.contains("admin")).collect();
    assert_eq!(
        admin_warns.len(),
        1,
        "no src/ at all should produce 'missing' warning, got {}: {admin_warns:#?}",
        admin_warns.len()
    );
    assert_warn_for_app(&warns, "admin");
}

// ============================================================================
// GROUP E: Symlinks
// ============================================================================

#[cfg(unix)]
#[test]
fn modules_is_broken_symlink() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules");
    std::os::unix::fs::symlink("/nonexistent/path", tmp.path().join("apps/admin/src/modules"))
        .expect("create broken symlink");
    let results = run_check(tmp.path());
    let warns = t_arch_01_warns(&results);
    // Broken symlink: metadata() follows link → target doesn't exist → returns None → warns
    let admin_warns: Vec<_> = warns.iter().filter(|w| w.title.contains("admin")).collect();
    assert_eq!(
        admin_warns.len(),
        1,
        "broken symlink should produce 'missing' warning, got {}: {admin_warns:#?}",
        admin_warns.len()
    );
    assert_warn_for_app(&warns, "admin");
    // Total: admin + landing = 2
    assert_eq!(warns.len(), 2, "expected 2 total warnings: {warns:#?}");
}

#[cfg(unix)]
#[test]
fn modules_symlink_to_valid_dir() {
    let tmp = copy_fixture();
    let target = tmp.path().join("valid_modules");
    std::fs::create_dir_all(target.join("domain/types")).expect("mkdir domain");
    std::fs::create_dir_all(target.join("application/commands")).expect("mkdir application");
    std::fs::create_dir_all(target.join("adapters/inbound")).expect("mkdir adapters/inbound");
    std::fs::create_dir_all(target.join("adapters/outbound")).expect("mkdir adapters/outbound");
    std::fs::create_dir_all(target.join("ports/inbound")).expect("mkdir ports/inbound");
    std::fs::create_dir_all(target.join("ports/outbound")).expect("mkdir ports/outbound");
    write_file(tmp.path(), "valid_modules/domain/types/index.ts", "export type T = {};");
    write_file(tmp.path(), "valid_modules/application/commands/run.ts", "export function run() {}");
    write_file(tmp.path(), "valid_modules/adapters/inbound/.gitkeep", "");
    write_file(tmp.path(), "valid_modules/adapters/outbound/.gitkeep", "");
    write_file(tmp.path(), "valid_modules/ports/inbound/.gitkeep", "");
    write_file(tmp.path(), "valid_modules/ports/outbound/.gitkeep", "");

    remove_dir(tmp.path(), "apps/admin/src/modules");
    std::os::unix::fs::symlink(&target, tmp.path().join("apps/admin/src/modules"))
        .expect("create symlink");
    let results = run_check(tmp.path());
    let warns = t_arch_01_warns(&results);
    let admin_warns: Vec<_> = warns.iter().filter(|w| w.title.contains("admin")).collect();
    assert_eq!(
        admin_warns.len(),
        0,
        "symlink to valid modules/ should not produce warning, got: {admin_warns:#?}"
    );
    // Only landing baseline warning
    assert_eq!(warns.len(), 1, "expected only landing baseline, got {}: {warns:#?}", warns.len());
}

#[cfg(unix)]
#[test]
fn modules_symlink_to_dev_null() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules");
    std::os::unix::fs::symlink("/dev/null", tmp.path().join("apps/admin/src/modules"))
        .expect("create /dev/null symlink");
    let results = run_check(tmp.path());
    // /dev/null is a file, not a dir — metadata() returns Some (it exists)
    // but it's not a directory. check_ts_modules_dir → list_ts_dir_names on /dev/null → empty
    // → cascades to rule_02 errors (missing dirs), no rule_01 warn
    let warns = t_arch_01_warns(&results);
    let admin_warns: Vec<_> = warns.iter().filter(|w| w.title.contains("admin")).collect();
    assert_eq!(
        admin_warns.len(),
        0,
        "/dev/null symlink: metadata succeeds, so no 'missing' warning, got: {admin_warns:#?}"
    );
    // Should cascade to rule_02 errors
    let errors = arch_errors(&results);
    let admin_errors: Vec<_> = errors.iter().filter(|e| e.title.contains("admin")).collect();
    assert_eq!(
        admin_errors.len(),
        4,
        "expected 4 rule_02 cascade errors for /dev/null symlink, got {}: {admin_errors:#?}",
        admin_errors.len()
    );
}

// ============================================================================
// GROUP F: App discovery edge cases
// ============================================================================

#[test]
fn app_discovered_via_ts_files_only() {
    let tmp = copy_fixture();
    // Create app with .ts files but NO package.json
    write_file(
        tmp.path(),
        "apps/scripts/src/main.ts",
        "console.log('hello');",
    );
    let results = run_check(tmp.path());
    let warns = t_arch_01_warns(&results);
    // landing (baseline) + scripts (new) = 2
    assert_eq!(
        warns.len(),
        2,
        "expected 2 warnings (landing + scripts), got {}: {warns:#?}",
        warns.len()
    );
    assert_warn_for_app(&warns, "scripts");
    assert_warn_for_app(&warns, "landing");
}

#[test]
fn app_discovered_via_tsx_files_only() {
    let tmp = copy_fixture();
    // Create app with only .tsx files, no .ts, no package.json
    write_file(
        tmp.path(),
        "apps/components/src/Button.tsx",
        "export default function Button() { return <div/>; }",
    );
    let results = run_check(tmp.path());
    let warns = t_arch_01_warns(&results);
    // landing (baseline) + components (new) = 2
    assert_eq!(
        warns.len(),
        2,
        "expected 2 warnings (landing + components), got {}: {warns:#?}",
        warns.len()
    );
    assert_warn_for_app(&warns, "components");
    assert_warn_for_app(&warns, "landing");
}

#[test]
fn deeply_nested_ts_triggers_discovery() {
    let tmp = copy_fixture();
    // .ts file buried deep — should still trigger discovery via WalkDir
    write_file(
        tmp.path(),
        "apps/hidden-gem/a/b/c/d/e/deep.ts",
        "export const x = 1;",
    );
    let results = run_check(tmp.path());
    let warns = t_arch_01_warns(&results);
    // landing (baseline) + hidden-gem (new) = 2
    assert_eq!(
        warns.len(),
        2,
        "expected 2 warnings (landing + hidden-gem), got {}: {warns:#?}",
        warns.len()
    );
    assert_warn_for_app(&warns, "hidden-gem");
}

#[test]
fn ts_file_directly_in_app_root() {
    let tmp = copy_fixture();
    // .ts file at direct child level (not nested in src/)
    write_file(tmp.path(), "apps/flat/index.ts", "export const x = 1;");
    let results = run_check(tmp.path());
    let warns = t_arch_01_warns(&results);
    // landing (baseline) + flat (new) = 2
    assert_eq!(
        warns.len(),
        2,
        "expected 2 warnings (landing + flat), got {}: {warns:#?}",
        warns.len()
    );
    assert_warn_for_app(&warns, "flat");
}

#[test]
fn node_modules_ts_files_not_discovered() {
    let tmp = copy_fixture();
    // App with ONLY .ts files in node_modules/ — should NOT be discovered
    // (is_excluded_ts_dir filters node_modules)
    std::fs::create_dir_all(tmp.path().join("apps/phantom")).expect("mkdir");
    write_file(
        tmp.path(),
        "apps/phantom/node_modules/some-pkg/index.ts",
        "export const x = 1;",
    );
    let results = run_check(tmp.path());
    let warns = t_arch_01_warns(&results);
    assert!(
        !warns.iter().any(|w| w.title.contains("phantom")),
        "app with only node_modules .ts files should not be discovered, got: {warns:#?}"
    );
    // Baseline only
    assert_eq!(warns.len(), 1, "expected only landing baseline, got {}: {warns:#?}", warns.len());
}

#[test]
fn build_artifacts_ts_files_not_discovered() {
    let tmp = copy_fixture();
    // App with .ts files only in .next/ build dir — should NOT be discovered
    std::fs::create_dir_all(tmp.path().join("apps/stale")).expect("mkdir");
    write_file(
        tmp.path(),
        "apps/stale/.next/static/types.ts",
        "export type T = {};",
    );
    let results = run_check(tmp.path());
    let warns = t_arch_01_warns(&results);
    assert!(
        !warns.iter().any(|w| w.title.contains("stale")),
        "app with only .next/ .ts files should not be discovered, got: {warns:#?}"
    );
    assert_eq!(warns.len(), 1, "expected only landing baseline, got {}: {warns:#?}", warns.len());
}

#[test]
fn dual_stack_app_discovered_by_ts() {
    let tmp = copy_fixture();
    // App with BOTH Cargo.toml and package.json — should be discovered by TS check
    write_file(
        tmp.path(),
        "apps/fullstack/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"",
    );
    write_file(
        tmp.path(),
        "apps/fullstack/package.json",
        r#"{"name": "fullstack"}"#,
    );
    let results = run_check(tmp.path());
    let warns = t_arch_01_warns(&results);
    // landing (baseline) + fullstack (new) = 2
    assert_eq!(
        warns.len(),
        2,
        "expected 2 warnings (landing + fullstack), got {}: {warns:#?}",
        warns.len()
    );
    assert_warn_for_app(&warns, "fullstack");
}

#[test]
fn package_json_empty() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/empty-pkg/package.json", "");
    let results = run_check(tmp.path());
    let warns = t_arch_01_warns(&results);
    // landing (baseline) + empty-pkg (new) = 2
    assert_eq!(
        warns.len(),
        2,
        "expected 2 warnings (landing + empty-pkg), got {}: {warns:#?}",
        warns.len()
    );
    assert_warn_for_app(&warns, "empty-pkg");
}

#[test]
fn package_json_malformed() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/bad-json/package.json",
        "this is {{{ not json",
    );
    let results = run_check(tmp.path());
    let warns = t_arch_01_warns(&results);
    // landing (baseline) + bad-json (new) = 2
    assert_eq!(
        warns.len(),
        2,
        "expected 2 warnings (landing + bad-json), got {}: {warns:#?}",
        warns.len()
    );
    assert_warn_for_app(&warns, "bad-json");
}

#[test]
fn package_json_is_directory() {
    let tmp = copy_fixture();
    std::fs::create_dir_all(tmp.path().join("apps/dir-pkg/package.json")).expect("mkdir");
    let results = run_check(tmp.path());
    let warns = t_arch_01_warns(&results);
    // read_file on a directory returns None → falls through to has_ts_files
    // No .ts files → app not discovered → no warning
    let dir_warns: Vec<_> = warns.iter().filter(|w| w.title.contains("dir-pkg")).collect();
    assert_eq!(
        dir_warns.len(),
        0,
        "app with package.json-as-directory and no .ts files should not be discovered, got: {dir_warns:#?}"
    );
    // Baseline only
    assert_eq!(warns.len(), 1, "expected only landing baseline, got {}: {warns:#?}", warns.len());
}

#[cfg(unix)]
#[test]
fn package_json_is_broken_symlink() {
    let tmp = copy_fixture();
    std::fs::create_dir_all(tmp.path().join("apps/link-pkg")).expect("mkdir");
    std::os::unix::fs::symlink(
        "/nonexistent",
        tmp.path().join("apps/link-pkg/package.json"),
    )
    .expect("create broken symlink");
    let results = run_check(tmp.path());
    let warns = t_arch_01_warns(&results);
    // read_file on broken symlink → None → falls through to has_ts_files → no .ts → not discovered
    let link_warns: Vec<_> = warns.iter().filter(|w| w.title.contains("link-pkg")).collect();
    assert_eq!(
        link_warns.len(),
        0,
        "app with broken symlink package.json and no .ts files should not be discovered, got: {link_warns:#?}"
    );
    assert_eq!(warns.len(), 1, "expected only landing baseline, got {}: {warns:#?}", warns.len());
}

#[test]
fn file_in_apps_dir_not_discovered() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/README.md", "# Apps");
    let results = run_check(tmp.path());
    let warns = t_arch_01_warns(&results);
    // README.md is a file, not a dir — discover_ts_apps skips via is_dir()
    assert!(
        !warns.iter().any(|w| w.title.contains("README")),
        "file in apps/ should not be discovered as an app, got: {warns:#?}"
    );
    assert_eq!(warns.len(), 1, "expected only landing baseline, got {}: {warns:#?}", warns.len());
}

#[test]
fn empty_apps_dir() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps");
    std::fs::create_dir_all(tmp.path().join("apps")).expect("mkdir empty apps");
    let results = run_check(tmp.path());
    let warns = t_arch_01_warns(&results);
    assert_eq!(
        warns.len(),
        0,
        "empty apps/ should produce 0 warnings, got {}: {warns:#?}",
        warns.len()
    );
}

#[test]
fn no_apps_dir_at_all() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps");
    let results = run_check(tmp.path());
    let warns = t_arch_01_warns(&results);
    assert_eq!(
        warns.len(),
        0,
        "no apps/ dir should produce 0 warnings, got {}: {warns:#?}",
        warns.len()
    );
}

#[test]
fn hidden_dir_in_apps_discovered() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/.hidden-app/package.json",
        r#"{"name": "hidden"}"#,
    );
    let results = run_check(tmp.path());
    let warns = t_arch_01_warns(&results);
    // landing (baseline) + .hidden-app (new) = 2
    assert_eq!(
        warns.len(),
        2,
        "expected 2 warnings (landing + .hidden-app), got {}: {warns:#?}",
        warns.len()
    );
    assert_warn_for_app(&warns, ".hidden-app");
}

// ============================================================================
// GROUP G: App name edge cases
// ============================================================================

#[test]
fn unicode_app_name() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/\u{00fc}ber-app/package.json",
        r#"{"name": "uber"}"#,
    );
    let results = run_check(tmp.path());
    let warns = t_arch_01_warns(&results);
    // landing (baseline) + über-app (new) = 2
    assert_eq!(
        warns.len(),
        2,
        "expected 2 warnings (landing + über-app), got {}: {warns:#?}",
        warns.len()
    );
    assert_warn_for_app(&warns, "\u{00fc}ber-app");
}

#[test]
fn app_name_with_spaces() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/my app/package.json",
        r#"{"name": "my-app"}"#,
    );
    let results = run_check(tmp.path());
    let warns = t_arch_01_warns(&results);
    // landing (baseline) + "my app" (new) = 2
    assert_eq!(
        warns.len(),
        2,
        "expected 2 warnings (landing + my app), got {}: {warns:#?}",
        warns.len()
    );
    assert_warn_for_app(&warns, "my app");
}

// ============================================================================
// GROUP H: Wrong casing and typos
// ============================================================================

#[test]
fn wrong_casing_modules() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules");
    std::fs::create_dir_all(tmp.path().join("apps/admin/src/Modules/domain")).expect("mkdir");
    let results = run_check(tmp.path());
    let warns = t_arch_01_warns(&results);
    let admin_warns: Vec<_> = warns.iter().filter(|w| w.title.contains("admin")).collect();
    // On case-insensitive filesystems (macOS default), Modules/ matches modules/ → no warn
    // On case-sensitive filesystems (Linux), Modules/ ≠ modules/ → warns
    // Either 0 or 1 admin warnings — platform-dependent, but assert bounded and no crash
    assert!(
        admin_warns.len() <= 1,
        "expected 0 or 1 admin warnings (platform-dependent), got {}: {admin_warns:#?}",
        admin_warns.len()
    );
    // Total should be 1 (landing only) or 2 (landing + admin)
    assert!(
        warns.len() == 1 || warns.len() == 2,
        "expected 1 or 2 total warnings (platform-dependent), got {}: {warns:#?}",
        warns.len()
    );
}

#[test]
fn typo_module_singular() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules");
    std::fs::create_dir_all(tmp.path().join("apps/admin/src/module/domain")).expect("mkdir");
    let results = run_check(tmp.path());
    let warns = t_arch_01_warns(&results);
    // module/ (singular) is not modules/ — should warn
    let admin_warns: Vec<_> = warns.iter().filter(|w| w.title.contains("admin")).collect();
    assert_eq!(
        admin_warns.len(),
        1,
        "module/ (singular) should not be accepted as modules/, got {}: {admin_warns:#?}",
        admin_warns.len()
    );
    assert_warn_for_app(&warns, "admin");
    // Total: admin + landing = 2
    assert_eq!(warns.len(), 2, "expected 2 total warnings: {warns:#?}");
}

// ============================================================================
// GROUP I: Filesystem permissions
// ============================================================================

#[cfg(unix)]
#[test]
fn modules_no_read_permission() {
    use std::os::unix::fs::PermissionsExt;
    let tmp = copy_fixture();
    let modules = tmp.path().join("apps/admin/src/modules");
    std::fs::set_permissions(&modules, std::fs::Permissions::from_mode(0o000)).expect("chmod");
    let results = run_check(tmp.path());
    // Restore permissions so tempdir cleanup works
    std::fs::set_permissions(&modules, std::fs::Permissions::from_mode(0o755)).expect("chmod");
    // metadata() succeeds (dir exists) → no rule_01 warn
    // But list_ts_dir_names → read_dir fails → empty → rule_02 errors
    let warns = t_arch_01_warns(&results);
    let admin_warns: Vec<_> = warns.iter().filter(|w| w.title.contains("admin")).collect();
    assert_eq!(
        admin_warns.len(),
        0,
        "unreadable modules/ should not produce 'missing' warning (metadata succeeds), got: {admin_warns:#?}"
    );
    // Should cascade to rule_02 errors (4 missing dirs)
    let errors = arch_errors(&results);
    let admin_errors: Vec<_> = errors.iter().filter(|e| e.title.contains("admin")).collect();
    assert_eq!(
        admin_errors.len(),
        4,
        "unreadable modules/ should cascade to 4 rule_02 errors, got {}: {admin_errors:#?}",
        admin_errors.len()
    );
}

// ============================================================================
// GROUP J: Isolation — break one app, verify others clean
// ============================================================================

#[test]
fn admin_broken_landing_still_warns() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules");
    let results = run_check(tmp.path());
    let warns = t_arch_01_warns(&results);
    assert_eq!(warns.len(), 2, "expected 2 warnings: {warns:#?}");
    // Landing baseline warn still present
    assert_warn_for_app(&warns, "landing");
    // Admin now warns too
    assert_warn_for_app(&warns, "admin");
    // No other apps
    assert_no_rust_apps(&warns);
    assert_no_packages(&warns);
}

#[test]
fn new_app_preserves_baseline() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/dashboard/package.json",
        r#"{"name": "dashboard"}"#,
    );
    let results = run_check(tmp.path());
    let warns = t_arch_01_warns(&results);
    // landing (baseline) + dashboard (new) = 2
    assert_eq!(
        warns.len(),
        2,
        "expected 2 warnings (landing + dashboard), got {}: {warns:#?}",
        warns.len()
    );
    assert_warn_for_app(&warns, "landing");
    assert_warn_for_app(&warns, "dashboard");
    // Admin still clean
    assert!(
        !warns.iter().any(|w| w.title.contains("admin")),
        "admin should not be warned, got: {warns:#?}"
    );
}

#[test]
fn multiple_new_apps_all_warned() {
    let tmp = copy_fixture();
    for name in &["dashboard", "api", "worker-ts"] {
        write_file(
            tmp.path(),
            &format!("apps/{name}/package.json"),
            &format!(r#"{{"name": "{name}"}}"#),
        );
    }
    let results = run_check(tmp.path());
    let warns = t_arch_01_warns(&results);
    // landing (baseline) + 3 new apps = 4
    assert_eq!(
        warns.len(),
        4,
        "expected 4 warnings (landing + 3 new), got {}: {warns:#?}",
        warns.len()
    );
    for name in &["landing", "dashboard", "api", "worker-ts"] {
        assert_warn_for_app(&warns, name);
    }
}

// ============================================================================
// GROUP K: Idempotency
// ============================================================================

#[test]
fn idempotent_results() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules");
    let results_1 = run_check(tmp.path());
    let warns_1 = t_arch_01_warns(&results_1);
    let results_2 = run_check(tmp.path());
    let warns_2 = t_arch_01_warns(&results_2);
    assert_eq!(
        warns_1.len(),
        warns_2.len(),
        "idempotent check failed: first run {} warnings, second run {} warnings",
        warns_1.len(),
        warns_2.len()
    );
    // Verify same app names warned in both runs
    for w1 in &warns_1 {
        assert!(
            warns_2.iter().any(|w2| w2.title == w1.title),
            "warning '{}' from run 1 missing in run 2, run 2: {warns_2:#?}",
            w1.title
        );
    }
}
