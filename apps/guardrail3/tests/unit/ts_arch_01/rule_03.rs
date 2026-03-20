use super::helpers::{
    arch_errors, assert_no_landing, assert_no_packages, assert_no_rust_apps, copy_fixture,
    remove_dir, run_check, write_file,
};
use guardrail3::domain::report::CheckResult;

// ============================================================================
// Rule 03: adapters/ and ports/ must contain exactly {inbound, outbound}
//
// Exercised by: arch_helpers::check_exact_subdirs called from check_ts_inbound_outbound
// with expected=["inbound", "outbound"] on adapters/ and ports/ dirs.
//
// Same shared function as rule 02, different expected dirs and label prefix.
// ============================================================================

const ADAPTERS: &str = "apps/admin/src/modules/adapters";
const PORTS: &str = "apps/admin/src/modules/ports";
const TS_SERVICE_APPS: &[&str] = &["admin", "portal"];

/// All 4 inbound/outbound locations in admin.
const ALL_IO_DIRS: &[&str] = &[
    "apps/admin/src/modules/adapters/inbound",
    "apps/admin/src/modules/adapters/outbound",
    "apps/admin/src/modules/ports/inbound",
    "apps/admin/src/modules/ports/outbound",
];

/// Filter to only rule-03-relevant errors: missing/unexpected/loose in adapters/ or ports/.
fn rule3_errors<'a>(errors: &[&'a CheckResult]) -> Vec<&'a CheckResult> {
    errors
        .iter()
        .filter(|e| {
            let t = &e.title;
            let in_adapters = t.contains("src/modules/adapters/");
            let in_ports = t.contains("src/modules/ports/");
            let is_structural = in_adapters || in_ports;
            // Missing inbound/outbound
            (is_structural && t.contains("missing") && (t.contains("inbound") || t.contains("outbound")))
            // Unexpected dir in adapters/ or ports/
            || (is_structural && t.contains("unexpected directory"))
            // Loose files in adapters/ or ports/ (not in their subdirs)
            || (t.contains("loose files in") && (t.contains("src/modules/adapters/") || t.contains("src/modules/ports/"))
                && !t.contains("/inbound") && !t.contains("/outbound"))
        })
        .copied()
        .collect()
}

/// Assert file field points to adapters/ or ports/ dir.
fn assert_file_field_structural(errors: &[&CheckResult]) {
    for err in errors {
        let file = err.file.as_deref().unwrap_or("");
        assert!(
            file.contains("adapters") || file.contains("ports"),
            "expected file field containing 'adapters' or 'ports', got: '{file}' for: '{}'",
            err.title
        );
    }
}

/// Assert every error mentions admin.
fn assert_all_mention_admin(errors: &[&CheckResult]) {
    for err in errors {
        assert!(
            err.title.contains("admin"),
            "expected error to mention 'admin', got: '{}'",
            err.title
        );
    }
}

/// Standard assertion battery for every mutation test.
fn assert_standard(r3: &[&CheckResult], all_errors: &[&CheckResult]) {
    assert_all_mention_admin(r3);
    assert_file_field_structural(r3);
    assert_no_rust_apps(all_errors);
    assert_no_packages(all_errors);
    assert_no_landing(all_errors);
}

// ============================================================================
// GROUP A: Golden baseline
// ============================================================================

#[test]
fn golden_passes() {
    let tmp = copy_fixture();
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(r3.len(), 0, "golden should have 0 rule-03 errors, got {}: {r3:#?}", r3.len());
}

// ============================================================================
// GROUP B: Missing inbound/outbound — one at a time
// ============================================================================

#[test]
fn missing_adapters_inbound() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/adapters/inbound");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(r3.len(), 1, "expected 1 error, got {}: {r3:#?}", r3.len());
    assert!(
        r3[0].title.contains("missing") && r3[0].title.contains("adapters") && r3[0].title.contains("inbound"),
        "expected title about missing adapters/inbound/, got: '{}'",
        r3[0].title
    );
    assert_standard(&r3, &errors);
}

#[test]
fn missing_adapters_outbound() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/adapters/outbound");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(r3.len(), 1, "expected 1 error, got {}: {r3:#?}", r3.len());
    assert!(
        r3[0].title.contains("missing") && r3[0].title.contains("adapters") && r3[0].title.contains("outbound"),
        "expected title about missing adapters/outbound/, got: '{}'",
        r3[0].title
    );
    assert_standard(&r3, &errors);
}

#[test]
fn missing_ports_inbound() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/ports/inbound");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(r3.len(), 1, "expected 1 error, got {}: {r3:#?}", r3.len());
    assert!(
        r3[0].title.contains("missing") && r3[0].title.contains("ports") && r3[0].title.contains("inbound"),
        "expected title about missing ports/inbound/, got: '{}'",
        r3[0].title
    );
    assert_standard(&r3, &errors);
}

#[test]
fn missing_ports_outbound() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/ports/outbound");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(r3.len(), 1, "expected 1 error, got {}: {r3:#?}", r3.len());
    assert!(
        r3[0].title.contains("missing") && r3[0].title.contains("ports") && r3[0].title.contains("outbound"),
        "expected title about missing ports/outbound/, got: '{}'",
        r3[0].title
    );
    assert_standard(&r3, &errors);
}

// ============================================================================
// GROUP C: Missing multiple
// ============================================================================

#[test]
fn missing_both_in_adapters() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/adapters/inbound");
    remove_dir(tmp.path(), "apps/admin/src/modules/adapters/outbound");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(r3.len(), 2, "expected 2 missing errors in adapters/, got {}: {r3:#?}", r3.len());
    assert!(r3.iter().any(|e| e.title.contains("inbound")), "expected inbound error");
    assert!(r3.iter().any(|e| e.title.contains("outbound")), "expected outbound error");
    assert_standard(&r3, &errors);
}

#[test]
fn missing_both_in_ports() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/ports/inbound");
    remove_dir(tmp.path(), "apps/admin/src/modules/ports/outbound");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(r3.len(), 2, "expected 2 missing errors in ports/, got {}: {r3:#?}", r3.len());
    assert!(r3.iter().any(|e| e.title.contains("inbound")), "expected inbound error");
    assert!(r3.iter().any(|e| e.title.contains("outbound")), "expected outbound error");
    assert_standard(&r3, &errors);
}

#[test]
fn missing_all_four() {
    let tmp = copy_fixture();
    // Break ALL TS service apps (admin + portal)
    for dir in ALL_IO_DIRS {
        remove_dir(tmp.path(), dir);
    }
    // Portal: remove all 4 io dirs
    // (removing adapters/inbound destroys payments inner hex, adapters/outbound destroys ai-chat — both unreachable)
    remove_dir(tmp.path(), "apps/portal/src/modules/adapters/inbound");
    remove_dir(tmp.path(), "apps/portal/src/modules/adapters/outbound");
    remove_dir(tmp.path(), "apps/portal/src/modules/ports/inbound");
    remove_dir(tmp.path(), "apps/portal/src/modules/ports/outbound");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    // 4 dirs * 2 apps = 8 errors
    assert_eq!(r3.len(), 8, "expected 8 missing errors (4 per app * 2 apps), got {}: {r3:#?}", r3.len());
    // Per-parent attribution
    let adapters_errors: Vec<_> = r3.iter().filter(|e| e.title.contains("adapters")).collect();
    let ports_errors: Vec<_> = r3.iter().filter(|e| e.title.contains("ports")).collect();
    assert_eq!(adapters_errors.len(), 4, "expected 4 adapters errors (2 per app): {r3:#?}");
    assert_eq!(ports_errors.len(), 4, "expected 4 ports errors (2 per app): {r3:#?}");
    // Per-app attribution
    for app in TS_SERVICE_APPS {
        assert!(
            r3.iter().any(|e| e.title.contains(app)),
            "expected error for app '{app}', got: {r3:#?}"
        );
    }
    assert_file_field_structural(&r3);
    assert_no_rust_apps(&errors);
    assert_no_packages(&errors);
    assert_no_landing(&errors);
}

// ============================================================================
// GROUP D: Unexpected dirs
// ============================================================================

#[test]
fn unexpected_dir_in_adapters() {
    let tmp = copy_fixture();
    write_file(tmp.path(), &format!("{ADAPTERS}/middleware/auth.ts"), "export function auth() {}");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(r3.len(), 1, "expected 1 unexpected-dir error, got {}: {r3:#?}", r3.len());
    assert!(
        r3[0].title.contains("unexpected") && r3[0].title.contains("middleware"),
        "expected unexpected 'middleware' error, got: '{}'",
        r3[0].title
    );
    let file = r3[0].file.as_deref().unwrap_or("");
    assert!(file.contains("adapters/middleware"), "expected file field pointing to adapters/middleware, got: '{file}'");
    assert_standard(&r3, &errors);
}

#[test]
fn unexpected_dir_in_ports() {
    let tmp = copy_fixture();
    write_file(tmp.path(), &format!("{PORTS}/shared/types.ts"), "export type T = {};");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(r3.len(), 1, "expected 1 unexpected-dir error, got {}: {r3:#?}", r3.len());
    assert!(
        r3[0].title.contains("unexpected") && r3[0].title.contains("shared"),
        "expected unexpected 'shared' error, got: '{}'",
        r3[0].title
    );
    let file = r3[0].file.as_deref().unwrap_or("");
    assert!(file.contains("ports/shared"), "expected file field pointing to ports/shared, got: '{file}'");
    assert_standard(&r3, &errors);
}

#[test]
fn multiple_unexpected_in_both() {
    let tmp = copy_fixture();
    write_file(tmp.path(), &format!("{ADAPTERS}/middleware/a.ts"), "");
    write_file(tmp.path(), &format!("{ADAPTERS}/utils/b.ts"), "");
    write_file(tmp.path(), &format!("{PORTS}/shared/c.ts"), "");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(r3.len(), 3, "expected 3 unexpected-dir errors (2 adapters + 1 ports), got {}: {r3:#?}", r3.len());
    assert!(r3.iter().any(|e| e.title.contains("middleware")), "expected middleware");
    assert!(r3.iter().any(|e| e.title.contains("utils")), "expected utils");
    assert!(r3.iter().any(|e| e.title.contains("shared")), "expected shared");
    assert_standard(&r3, &errors);
}

#[test]
fn unexpected_dir_named_domain() {
    let tmp = copy_fixture();
    // "domain" is valid at modules/ root (rule 02) but NOT inside adapters/
    write_file(tmp.path(), &format!("{ADAPTERS}/domain/types.ts"), "");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(r3.len(), 1, "expected 1 unexpected 'domain' in adapters/, got {}: {r3:#?}", r3.len());
    assert!(r3[0].title.contains("unexpected") && r3[0].title.contains("domain"), "expected domain error");
    assert_standard(&r3, &errors);
}

// ============================================================================
// GROUP E: Loose files in adapters/ and ports/
// ============================================================================

#[test]
fn loose_file_in_adapters() {
    let tmp = copy_fixture();
    write_file(tmp.path(), &format!("{ADAPTERS}/index.ts"), "// stray");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(r3.len(), 1, "expected 1 loose-file error in adapters/, got {}: {r3:#?}", r3.len());
    assert!(r3[0].title.contains("loose files") && r3[0].title.contains("adapters"), "expected loose in adapters");
    assert!(r3[0].message.contains("index.ts"), "expected index.ts in message");
    assert_standard(&r3, &errors);
}

#[test]
fn loose_file_in_ports() {
    let tmp = copy_fixture();
    write_file(tmp.path(), &format!("{PORTS}/types.ts"), "// stray");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(r3.len(), 1, "expected 1 loose-file error in ports/, got {}: {r3:#?}", r3.len());
    assert!(r3[0].title.contains("loose files") && r3[0].title.contains("ports"), "expected loose in ports");
    assert!(r3[0].message.contains("types.ts"), "expected types.ts in message");
    assert_standard(&r3, &errors);
}

#[test]
fn loose_files_in_both() {
    let tmp = copy_fixture();
    write_file(tmp.path(), &format!("{ADAPTERS}/stray.ts"), "// stray");
    write_file(tmp.path(), &format!("{PORTS}/stray.ts"), "// stray");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(r3.len(), 2, "expected 2 loose-file errors (adapters + ports), got {}: {r3:#?}", r3.len());
    assert!(r3.iter().any(|e| e.title.contains("adapters")), "expected adapters loose");
    assert!(r3.iter().any(|e| e.title.contains("ports")), "expected ports loose");
    assert_standard(&r3, &errors);
}

#[test]
fn multiple_loose_files_single_error() {
    let tmp = copy_fixture();
    write_file(tmp.path(), &format!("{ADAPTERS}/index.ts"), "");
    write_file(tmp.path(), &format!("{ADAPTERS}/types.ts"), "");
    write_file(tmp.path(), &format!("{ADAPTERS}/config.json"), "{}");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(r3.len(), 1, "expected 1 loose-file error listing all files, got {}: {r3:#?}", r3.len());
    assert!(r3[0].message.contains("index.ts"), "expected index.ts in message");
    assert!(r3[0].message.contains("types.ts"), "expected types.ts in message");
    assert!(r3[0].message.contains("config.json"), "expected config.json in message");
    assert_standard(&r3, &errors);
}

// ============================================================================
// GROUP F: .gitkeep behavior
// ============================================================================

#[test]
fn gitkeep_allowed() {
    let tmp = copy_fixture();
    write_file(tmp.path(), &format!("{ADAPTERS}/.gitkeep"), "");
    write_file(tmp.path(), &format!("{PORTS}/.gitkeep"), "");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(r3.len(), 0, "expected 0 errors when .gitkeep in structural dirs, got {}: {r3:#?}", r3.len());
}

#[test]
fn gitkeep_alongside_loose_file() {
    let tmp = copy_fixture();
    write_file(tmp.path(), &format!("{ADAPTERS}/.gitkeep"), "");
    write_file(tmp.path(), &format!("{ADAPTERS}/stray.ts"), "// stray");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(r3.len(), 1, "expected 1 loose-file error, got {}: {r3:#?}", r3.len());
    assert!(r3[0].message.contains("stray.ts"), "expected stray.ts in message");
    let bad_files_section = r3[0]
        .message
        .split("that don't belong: ")
        .nth(1)
        .and_then(|s| s.split(". Only").next())
        .unwrap_or("");
    assert!(
        !bad_files_section.contains(".gitkeep"),
        ".gitkeep should not be in bad files list, got: '{bad_files_section}'"
    );
    assert_standard(&r3, &errors);
}

// ============================================================================
// GROUP G: Combinations
// ============================================================================

#[test]
fn missing_plus_unexpected() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/adapters/inbound");
    write_file(tmp.path(), &format!("{ADAPTERS}/middleware/auth.ts"), "");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(r3.len(), 2, "expected 2 errors (1 missing + 1 unexpected), got {}: {r3:#?}", r3.len());
    assert!(r3.iter().any(|e| e.title.contains("missing") && e.title.contains("inbound")), "expected missing inbound");
    assert!(r3.iter().any(|e| e.title.contains("unexpected") && e.title.contains("middleware")), "expected unexpected middleware");
    assert_standard(&r3, &errors);
}

#[test]
fn missing_plus_loose() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/ports/outbound");
    write_file(tmp.path(), &format!("{PORTS}/stray.ts"), "// stray");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(r3.len(), 2, "expected 2 errors (1 missing + 1 loose), got {}: {r3:#?}", r3.len());
    assert!(r3.iter().any(|e| e.title.contains("missing") && e.title.contains("outbound")), "expected missing outbound");
    assert!(r3.iter().any(|e| e.title.contains("loose files")), "expected loose files");
    assert_standard(&r3, &errors);
}

#[test]
fn all_three_violations() {
    let tmp = copy_fixture();
    // admin: all 3 violation types in adapters/
    remove_dir(tmp.path(), "apps/admin/src/modules/adapters/inbound");
    write_file(tmp.path(), &format!("{ADAPTERS}/middleware/a.ts"), "");
    write_file(tmp.path(), &format!("{ADAPTERS}/stray.ts"), "// stray");
    // portal: all 3 violation types in ports/ (avoid adapters/ to not destroy inner hexes)
    remove_dir(tmp.path(), "apps/portal/src/modules/ports/inbound");
    write_file(tmp.path(), "apps/portal/src/modules/ports/shared/types.ts", "");
    write_file(tmp.path(), "apps/portal/src/modules/ports/stray.ts", "// stray");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    // 3 violations * 2 apps = 6 errors
    assert_eq!(r3.len(), 6, "expected 6 errors (3 per app * 2 apps), got {}: {r3:#?}", r3.len());
    assert!(r3.iter().any(|e| e.title.contains("missing")), "expected missing");
    assert!(r3.iter().any(|e| e.title.contains("unexpected")), "expected unexpected");
    assert!(r3.iter().any(|e| e.title.contains("loose files")), "expected loose");
    for app in TS_SERVICE_APPS {
        assert!(
            r3.iter().any(|e| e.title.contains(app)),
            "expected error for app '{app}', got: {r3:#?}"
        );
    }
    assert_file_field_structural(&r3);
    assert_no_rust_apps(&errors);
    assert_no_packages(&errors);
    assert_no_landing(&errors);
}

#[test]
fn violations_in_both_adapters_and_ports() {
    let tmp = copy_fixture();
    // admin: missing in adapters + ports
    remove_dir(tmp.path(), "apps/admin/src/modules/adapters/inbound");
    remove_dir(tmp.path(), "apps/admin/src/modules/ports/outbound");
    // portal: missing in adapters + ports
    // (removing adapters/inbound destroys payments inner hex — unreachable, no cascade)
    remove_dir(tmp.path(), "apps/portal/src/modules/adapters/inbound");
    remove_dir(tmp.path(), "apps/portal/src/modules/ports/outbound");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    // 2 violations * 2 apps = 4 errors
    assert_eq!(r3.len(), 4, "expected 4 errors (2 per app * 2 apps), got {}: {r3:#?}", r3.len());
    assert!(r3.iter().any(|e| e.title.contains("adapters") && e.title.contains("inbound")), "expected adapters/inbound");
    assert!(r3.iter().any(|e| e.title.contains("ports") && e.title.contains("outbound")), "expected ports/outbound");
    for app in TS_SERVICE_APPS {
        assert!(
            r3.iter().any(|e| e.title.contains(app)),
            "expected error for app '{app}', got: {r3:#?}"
        );
    }
    assert_file_field_structural(&r3);
    assert_no_rust_apps(&errors);
    assert_no_packages(&errors);
    assert_no_landing(&errors);
}

// ============================================================================
// GROUP H: Edge cases — dir replaced by file
// ============================================================================

#[test]
fn inbound_replaced_with_file() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/adapters/inbound");
    write_file(tmp.path(), &format!("{ADAPTERS}/inbound"), "not a directory");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    // "inbound" as a file: not is_dir → missing + loose
    assert_eq!(r3.len(), 2, "expected 2 errors (missing + loose), got {}: {r3:#?}", r3.len());
    assert!(r3.iter().any(|e| e.title.contains("missing") && e.title.contains("inbound")), "expected missing inbound");
    assert!(r3.iter().any(|e| e.title.contains("loose files")), "expected loose files");
    assert_standard(&r3, &errors);
}

// ============================================================================
// GROUP I: Edge cases — symlinks
// ============================================================================

#[cfg(unix)]
#[test]
fn inbound_replaced_with_symlink() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/adapters/inbound");
    std::os::unix::fs::symlink(
        tmp.path().join("apps/admin/src/modules/adapters/outbound"),
        tmp.path().join("apps/admin/src/modules/adapters/inbound"),
    )
    .expect("create symlink");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    // DirEntry::file_type() doesn't follow symlinks → missing + loose
    assert_eq!(r3.len(), 2, "expected 2 errors (missing + loose) for symlink, got {}: {r3:#?}", r3.len());
    assert_standard(&r3, &errors);
}

#[cfg(unix)]
#[test]
fn inbound_is_broken_symlink() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/adapters/inbound");
    std::os::unix::fs::symlink(
        "/nonexistent",
        tmp.path().join("apps/admin/src/modules/adapters/inbound"),
    )
    .expect("create broken symlink");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(r3.len(), 2, "expected 2 errors (missing + loose) for broken symlink, got {}: {r3:#?}", r3.len());
    assert_standard(&r3, &errors);
}

// ============================================================================
// GROUP J: Edge cases — empty dirs
// ============================================================================

#[test]
fn empty_inbound_passes_rule3() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/adapters/inbound");
    std::fs::create_dir_all(tmp.path().join("apps/admin/src/modules/adapters/inbound")).expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    // Empty dir is still a dir → passes rule 03
    assert_eq!(r3.len(), 0, "empty inbound/ should pass rule 03, got {}: {r3:#?}", r3.len());
}

// ============================================================================
// GROUP K: Near-miss names
// ============================================================================

#[test]
fn near_miss_names() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/adapters/inbound");
    remove_dir(tmp.path(), "apps/admin/src/modules/adapters/outbound");
    std::fs::create_dir_all(tmp.path().join(&format!("{ADAPTERS}/in"))).expect("mkdir");
    std::fs::create_dir_all(tmp.path().join(&format!("{ADAPTERS}/out"))).expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    // 2 missing (inbound, outbound) + 2 unexpected (in, out) = 4
    assert_eq!(r3.len(), 4, "expected 4 errors (2 missing + 2 unexpected), got {}: {r3:#?}", r3.len());
    assert!(r3.iter().any(|e| e.title.contains("missing") && e.title.contains("inbound")), "expected missing inbound");
    assert!(r3.iter().any(|e| e.title.contains("missing") && e.title.contains("outbound")), "expected missing outbound");
    assert_standard(&r3, &errors);
}

// ============================================================================
// GROUP L: Cross-language isolation
// ============================================================================

#[test]
fn rust_apps_not_flagged() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/adapters/inbound");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_no_rust_apps(&errors);
}

#[test]
fn packages_not_flagged() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/adapters/inbound");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_no_packages(&errors);
}

#[test]
fn landing_not_flagged() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/adapters/inbound");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_no_landing(&errors);
}

// ============================================================================
// GROUP M: Idempotency
// ============================================================================

#[test]
fn idempotent_results() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/adapters/inbound");
    write_file(tmp.path(), &format!("{PORTS}/stray.ts"), "");
    let results_1 = run_check(tmp.path());
    let errors_1 = arch_errors(&results_1);
    let r3_1 = rule3_errors(&errors_1);
    let results_2 = run_check(tmp.path());
    let errors_2 = arch_errors(&results_2);
    let r3_2 = rule3_errors(&errors_2);
    assert_eq!(r3_1.len(), r3_2.len(), "idempotent: run 1 = {}, run 2 = {}", r3_1.len(), r3_2.len());
    for e1 in &r3_1 {
        assert!(
            r3_2.iter().any(|e2| e2.title == e1.title),
            "error '{}' from run 1 missing in run 2",
            e1.title
        );
    }
    assert_standard(&r3_1, &errors_1);
}

// ============================================================================
// GROUP N: Filesystem permissions
// ============================================================================

#[cfg(unix)]
#[test]
fn adapters_no_read_permission() {
    use std::os::unix::fs::PermissionsExt;
    let tmp = copy_fixture();
    let adapters = tmp.path().join("apps/admin/src/modules/adapters");
    std::fs::set_permissions(&adapters, std::fs::Permissions::from_mode(0o000)).expect("chmod");
    let results = run_check(tmp.path());
    std::fs::set_permissions(&adapters, std::fs::Permissions::from_mode(0o755)).expect("chmod");
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    // Unreadable adapters/: list_dir_names returns empty → inbound + outbound "missing"
    assert_eq!(r3.len(), 2, "expected 2 missing errors for unreadable adapters/, got {}: {r3:#?}", r3.len());
    assert!(r3.iter().any(|e| e.title.contains("missing") && e.title.contains("inbound")), "expected missing inbound");
    assert!(r3.iter().any(|e| e.title.contains("missing") && e.title.contains("outbound")), "expected missing outbound");
    assert_standard(&r3, &errors);
}

// ============================================================================
// GROUP O: Early return — parent dir missing
// ============================================================================

#[test]
fn parent_adapters_missing_rule3_silent() {
    let tmp = copy_fixture();
    // Remove adapters/ entirely — rule 02 catches this, rule 03 should produce 0 errors
    remove_dir(tmp.path(), "apps/admin/src/modules/adapters");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(
        r3.len(),
        0,
        "rule 03 should produce 0 errors when adapters/ is missing (rule 02 handles it), got {}: {r3:#?}",
        r3.len()
    );
}

#[test]
fn parent_ports_missing_rule3_silent() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/ports");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(
        r3.len(),
        0,
        "rule 03 should produce 0 errors when ports/ is missing (rule 02 handles it), got {}: {r3:#?}",
        r3.len()
    );
}

#[test]
fn both_parents_missing_rule3_silent() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/adapters");
    remove_dir(tmp.path(), "apps/admin/src/modules/ports");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(
        r3.len(),
        0,
        "rule 03 should produce 0 errors when both parents missing, got {}: {r3:#?}",
        r3.len()
    );
}

// ============================================================================
// GROUP P: Additional edge cases
// ============================================================================

#[test]
fn wrong_case_dir_name() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/adapters/inbound");
    std::fs::create_dir_all(tmp.path().join(&format!("{ADAPTERS}/Inbound"))).expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    // "Inbound" (capital I) is not "inbound" — should produce missing + unexpected
    // On case-insensitive FS (macOS), Inbound/ may resolve as inbound/ → 0 errors
    // On case-sensitive FS (Linux), produces 2 errors (missing inbound + unexpected Inbound)
    assert!(
        r3.len() == 0 || r3.len() == 2,
        "expected 0 (case-insensitive) or 2 (case-sensitive) errors, got {}: {r3:#?}",
        r3.len()
    );
}

#[test]
fn gitignore_is_loose_file() {
    let tmp = copy_fixture();
    write_file(tmp.path(), &format!("{ADAPTERS}/.gitignore"), "node_modules/");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(r3.len(), 1, "expected 1 loose-file error for .gitignore, got {}: {r3:#?}", r3.len());
    assert!(r3[0].message.contains(".gitignore"), "expected .gitignore in message");
    assert_standard(&r3, &errors);
}

#[test]
fn ds_store_is_loose_file() {
    let tmp = copy_fixture();
    write_file(tmp.path(), &format!("{ADAPTERS}/.DS_Store"), "");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(r3.len(), 1, "expected 1 loose-file error for .DS_Store, got {}: {r3:#?}", r3.len());
    assert!(r3[0].message.contains(".DS_Store"), "expected .DS_Store in message");
    assert_standard(&r3, &errors);
}

#[test]
fn hidden_dir_unexpected() {
    let tmp = copy_fixture();
    write_file(tmp.path(), &format!("{ADAPTERS}/.hidden/secret.ts"), "");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(r3.len(), 1, "expected 1 unexpected-dir error for .hidden, got {}: {r3:#?}", r3.len());
    assert!(r3[0].title.contains(".hidden"), "expected .hidden in title");
    assert_standard(&r3, &errors);
}

#[test]
fn file_coexists_with_same_named_dir() {
    let tmp = copy_fixture();
    // "inbound.bak" file alongside real inbound/ dir
    write_file(tmp.path(), &format!("{ADAPTERS}/inbound.bak"), "backup");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    // Only loose-file error (inbound.bak is a file, inbound/ dir is still present)
    assert_eq!(r3.len(), 1, "expected 1 loose-file error, got {}: {r3:#?}", r3.len());
    assert!(r3[0].title.contains("loose files"), "expected loose files error");
    assert!(r3[0].message.contains("inbound.bak"), "expected inbound.bak in message");
    assert_standard(&r3, &errors);
}

#[test]
fn gitkeep_as_directory() {
    let tmp = copy_fixture();
    std::fs::create_dir_all(tmp.path().join(&format!("{ADAPTERS}/.gitkeep"))).expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(r3.len(), 1, "expected 1 unexpected-dir error for .gitkeep/, got {}: {r3:#?}", r3.len());
    assert!(
        r3[0].title.contains("unexpected") && r3[0].title.contains(".gitkeep"),
        "expected unexpected .gitkeep/ dir error, got: '{}'",
        r3[0].title
    );
    assert_standard(&r3, &errors);
}

#[test]
fn nested_unexpected_only_top_flagged() {
    let tmp = copy_fixture();
    write_file(tmp.path(), &format!("{ADAPTERS}/middleware/internal/deep/handler.ts"), "");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(r3.len(), 1, "expected 1 error (top-level middleware/ only), got {}: {r3:#?}", r3.len());
    assert!(r3[0].title.contains("middleware"), "expected middleware in title");
    assert!(
        !r3.iter().any(|e| e.title.contains("internal") || e.title.contains("deep")),
        "should not flag nested dirs inside unexpected dir"
    );
    assert_standard(&r3, &errors);
}

#[test]
fn unicode_lookalike_dir_name() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/adapters/inbound");
    // Zero-width space: "i\u{200B}nbound" looks like "inbound" but isn't
    std::fs::create_dir_all(tmp.path().join(&format!("{ADAPTERS}/i\u{200B}nbound"))).expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r3 = rule3_errors(&errors);
    // Missing real inbound + unexpected lookalike
    assert_eq!(r3.len(), 2, "expected 2 errors (missing + unexpected lookalike), got {}: {r3:#?}", r3.len());
    assert!(r3.iter().any(|e| e.title.contains("missing") && e.title.contains("inbound")), "expected missing inbound");
    assert!(r3.iter().any(|e| e.title.contains("unexpected")), "expected unexpected dir");
    assert_standard(&r3, &errors);
}
