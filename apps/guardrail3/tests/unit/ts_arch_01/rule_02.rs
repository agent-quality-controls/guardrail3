use super::helpers::{
    arch_errors, assert_no_landing, assert_no_packages, assert_no_rust_apps, copy_fixture,
    remove_dir, run_check, write_file,
};
use guardrail3::domain::report::CheckResult;

// ============================================================================
// Rule 02: modules/ must contain exactly {domain, ports, application, adapters}
//
// Exercised by: arch_helpers::check_exact_subdirs called from check_ts_modules_dir
// Reports: missing dirs, unexpected dirs, loose files in modules/
//
// TS expected dirs: ["adapters", "application", "domain", "ports"]
// (RS uses ["adapters", "app", "domain", "ports"] — "application" vs "app")
// ============================================================================

const EXPECTED_DIRS: &[&str] = &["adapters", "application", "domain", "ports"];

/// Filter to only rule-02-relevant errors: missing dirs, unexpected dirs, loose files
/// at the modules/ level. Excludes rule 03+ errors (inbound/outbound, container, leaf).
fn rule2_errors<'a>(errors: &[&'a CheckResult]) -> Vec<&'a CheckResult> {
    errors
        .iter()
        .filter(|e| {
            let t = &e.title;
            // Missing top-level hex dir (not inbound/outbound which is rule 03)
            (t.contains("missing src/modules/") && EXPECTED_DIRS.iter().any(|d| t.ends_with(&format!("{d}/ directory"))))
            // Unexpected dir in modules/ (not in subdirs like adapters/)
            || (t.contains("unexpected directory src/modules/"))
            // Loose files in modules/ root (not in subdirs)
            || (t.contains("loose files in src/modules/") && !t.contains("src/modules/adapters") && !t.contains("src/modules/ports") && !t.contains("src/modules/application") && !t.contains("src/modules/domain"))
        })
        .copied()
        .collect()
}

/// Assert file field points to modules dir.
fn assert_file_field_modules(errors: &[&CheckResult]) {
    for err in errors {
        let file = err.file.as_deref().unwrap_or("");
        assert!(
            file.contains("modules"),
            "expected file field containing 'modules', got: '{file}' for error: '{}'",
            err.title
        );
    }
}

/// Assert every error mentions the app name.
fn assert_all_mention_admin(errors: &[&CheckResult]) {
    for err in errors {
        assert!(
            err.title.contains("admin"),
            "expected error to mention 'admin', got: '{}'",
            err.title
        );
    }
}

/// Standard assertion battery for every mutation test on admin.
/// Call after computing r2 errors.
fn assert_standard(r2: &[&CheckResult], all_errors: &[&CheckResult]) {
    assert_all_mention_admin(r2);
    assert_file_field_modules(r2);
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
    let r2 = rule2_errors(&errors);
    assert_eq!(
        r2.len(),
        0,
        "golden should have 0 rule-02 errors, got {}: {r2:#?}",
        r2.len()
    );
}

// ============================================================================
// GROUP B: Missing required dirs — one at a time
// ============================================================================

#[test]
fn missing_domain() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/domain");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r2 = rule2_errors(&errors);
    assert_eq!(r2.len(), 1, "expected 1 missing-domain error, got {}: {r2:#?}", r2.len());
    assert!(
        r2[0].title.contains("missing") && r2[0].title.contains("domain"),
        "expected title about missing domain/, got: '{}'",
        r2[0].title
    );
    assert_standard(&r2, &errors);
}

#[test]
fn missing_ports() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/ports");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r2 = rule2_errors(&errors);
    assert_eq!(r2.len(), 1, "expected 1 missing-ports error, got {}: {r2:#?}", r2.len());
    assert!(
        r2[0].title.contains("missing") && r2[0].title.contains("ports"),
        "expected title about missing ports/, got: '{}'",
        r2[0].title
    );
    assert_standard(&r2, &errors);
}

#[test]
fn missing_application() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/application");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r2 = rule2_errors(&errors);
    assert_eq!(r2.len(), 1, "expected 1 missing-application error, got {}: {r2:#?}", r2.len());
    assert!(
        r2[0].title.contains("missing") && r2[0].title.contains("application"),
        "expected title about missing application/, got: '{}'",
        r2[0].title
    );
    assert_standard(&r2, &errors);
}

#[test]
fn missing_adapters() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/adapters");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r2 = rule2_errors(&errors);
    assert_eq!(r2.len(), 1, "expected 1 missing-adapters error, got {}: {r2:#?}", r2.len());
    assert!(
        r2[0].title.contains("missing") && r2[0].title.contains("adapters"),
        "expected title about missing adapters/, got: '{}'",
        r2[0].title
    );
    assert_standard(&r2, &errors);
}

// ============================================================================
// GROUP C: Missing multiple dirs
// ============================================================================

#[test]
fn missing_two_dirs() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/domain");
    remove_dir(tmp.path(), "apps/admin/src/modules/ports");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r2 = rule2_errors(&errors);
    assert_eq!(r2.len(), 2, "expected 2 missing-dir errors, got {}: {r2:#?}", r2.len());
    assert!(r2.iter().any(|e| e.title.contains("domain")), "expected domain error: {r2:#?}");
    assert!(r2.iter().any(|e| e.title.contains("ports")), "expected ports error: {r2:#?}");
    assert_standard(&r2, &errors);
}

#[test]
fn missing_all_four() {
    let tmp = copy_fixture();
    for dir in EXPECTED_DIRS {
        remove_dir(tmp.path(), &format!("apps/admin/src/modules/{dir}"));
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r2 = rule2_errors(&errors);
    assert_eq!(r2.len(), 4, "expected 4 missing-dir errors, got {}: {r2:#?}", r2.len());
    for dir in EXPECTED_DIRS {
        assert!(
            r2.iter().any(|e| e.title.contains(dir)),
            "expected error mentioning '{dir}', got: {r2:#?}"
        );
    }
    assert_standard(&r2, &errors);
}

// ============================================================================
// GROUP D: Unexpected dirs
// ============================================================================

#[test]
fn unexpected_directory() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/src/modules/services/handler.ts",
        "export function handle() {}",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r2 = rule2_errors(&errors);
    assert_eq!(r2.len(), 1, "expected 1 unexpected-dir error, got {}: {r2:#?}", r2.len());
    assert!(
        r2[0].title.contains("unexpected") && r2[0].title.contains("services"),
        "expected title about unexpected 'services', got: '{}'",
        r2[0].title
    );
    // File field should point to the unexpected dir itself
    let file = r2[0].file.as_deref().unwrap_or("");
    assert!(
        file.contains("modules/services"),
        "expected file field pointing to modules/services, got: '{file}'"
    );
    assert_standard(&r2, &errors);
}

#[test]
fn multiple_unexpected_dirs() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/admin/src/modules/services/a.ts", "");
    write_file(tmp.path(), "apps/admin/src/modules/utils/b.ts", "");
    write_file(tmp.path(), "apps/admin/src/modules/shared/c.ts", "");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r2 = rule2_errors(&errors);
    assert_eq!(r2.len(), 3, "expected 3 unexpected-dir errors, got {}: {r2:#?}", r2.len());
    for name in &["services", "utils", "shared"] {
        assert!(
            r2.iter().any(|e| e.title.contains(name)),
            "expected error about '{name}', got: {r2:#?}"
        );
    }
    assert_standard(&r2, &errors);
}

#[test]
fn hidden_dir_unexpected() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/admin/src/modules/.hidden/secret.ts", "");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r2 = rule2_errors(&errors);
    assert_eq!(r2.len(), 1, "expected 1 unexpected-dir error for .hidden, got {}: {r2:#?}", r2.len());
    assert!(
        r2[0].title.contains(".hidden"),
        "expected title mentioning '.hidden', got: '{}'",
        r2[0].title
    );
    assert_standard(&r2, &errors);
}

#[test]
fn unexpected_dir_named_inbound() {
    let tmp = copy_fixture();
    // "inbound" is valid inside adapters/ or ports/ (rule 03) but NOT at modules/ root
    write_file(tmp.path(), "apps/admin/src/modules/inbound/handler.ts", "");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r2 = rule2_errors(&errors);
    assert_eq!(r2.len(), 1, "expected 1 unexpected-dir error for 'inbound' at root, got {}: {r2:#?}", r2.len());
    assert!(
        r2[0].title.contains("unexpected") && r2[0].title.contains("inbound"),
        "expected unexpected 'inbound' error, got: '{}'",
        r2[0].title
    );
    assert_standard(&r2, &errors);
}

// ============================================================================
// GROUP E: Loose files in modules/
// ============================================================================

#[test]
fn loose_ts_file_in_modules() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/admin/src/modules/index.ts", "// stray");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r2 = rule2_errors(&errors);
    assert_eq!(r2.len(), 1, "expected 1 loose-file error, got {}: {r2:#?}", r2.len());
    assert!(
        r2[0].title.contains("loose files"),
        "expected 'loose files' in title, got: '{}'",
        r2[0].title
    );
    assert!(
        r2[0].message.contains("index.ts"),
        "expected message to list 'index.ts', got: '{}'",
        r2[0].message
    );
    assert_standard(&r2, &errors);
}

#[test]
fn loose_json_file_in_modules() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/admin/src/modules/config.json", "{}");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r2 = rule2_errors(&errors);
    assert_eq!(r2.len(), 1, "expected 1 loose-file error, got {}: {r2:#?}", r2.len());
    assert!(r2[0].message.contains("config.json"), "expected message to list config.json");
    assert_standard(&r2, &errors);
}

#[test]
fn loose_gitignore_not_gitkeep() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/admin/src/modules/.gitignore", "node_modules/");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r2 = rule2_errors(&errors);
    assert_eq!(r2.len(), 1, "expected 1 loose-file error for .gitignore, got {}: {r2:#?}", r2.len());
    assert!(r2[0].message.contains(".gitignore"), "expected message to list .gitignore");
    assert_standard(&r2, &errors);
}

#[test]
fn multiple_loose_files() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/admin/src/modules/index.ts", "// stray");
    write_file(tmp.path(), "apps/admin/src/modules/types.ts", "// stray");
    write_file(tmp.path(), "apps/admin/src/modules/config.json", "{}");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r2 = rule2_errors(&errors);
    assert_eq!(r2.len(), 1, "expected 1 loose-file error (listing all files), got {}: {r2:#?}", r2.len());
    assert!(r2[0].message.contains("index.ts"), "expected index.ts in message");
    assert!(r2[0].message.contains("types.ts"), "expected types.ts in message");
    assert!(r2[0].message.contains("config.json"), "expected config.json in message");
    assert_standard(&r2, &errors);
}

// ============================================================================
// GROUP F: .gitkeep behavior
// ============================================================================

#[test]
fn gitkeep_allowed_in_modules() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/admin/src/modules/.gitkeep", "");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r2 = rule2_errors(&errors);
    assert_eq!(r2.len(), 0, "expected 0 rule-02 errors when .gitkeep present, got {}: {r2:#?}", r2.len());
}

#[test]
fn gitkeep_alongside_loose_file() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/admin/src/modules/.gitkeep", "");
    write_file(tmp.path(), "apps/admin/src/modules/stray.ts", "// stray");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r2 = rule2_errors(&errors);
    assert_eq!(r2.len(), 1, "expected 1 loose-file error (stray.ts only), got {}: {r2:#?}", r2.len());
    assert!(r2[0].message.contains("stray.ts"), "expected stray.ts in message");
    // .gitkeep should NOT appear in the bad files list
    let bad_files_section = r2[0]
        .message
        .split("that don't belong: ")
        .nth(1)
        .and_then(|s| s.split(". Only").next())
        .unwrap_or("");
    assert!(
        !bad_files_section.contains(".gitkeep"),
        ".gitkeep should not be in bad files list, got: '{bad_files_section}'"
    );
    assert_standard(&r2, &errors);
}

#[test]
fn gitkeep_as_directory() {
    let tmp = copy_fixture();
    std::fs::create_dir_all(tmp.path().join("apps/admin/src/modules/.gitkeep")).expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r2 = rule2_errors(&errors);
    assert_eq!(r2.len(), 1, "expected 1 unexpected-dir error for .gitkeep/, got {}: {r2:#?}", r2.len());
    assert!(
        r2[0].title.contains("unexpected") && r2[0].title.contains(".gitkeep"),
        "expected unexpected .gitkeep/ dir error, got: '{}'",
        r2[0].title
    );
    assert_standard(&r2, &errors);
}

// ============================================================================
// GROUP G: Combinations
// ============================================================================

#[test]
fn missing_plus_unexpected() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/domain");
    write_file(tmp.path(), "apps/admin/src/modules/services/handler.ts", "");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r2 = rule2_errors(&errors);
    assert_eq!(r2.len(), 2, "expected 2 errors (1 missing + 1 unexpected), got {}: {r2:#?}", r2.len());
    assert!(r2.iter().any(|e| e.title.contains("missing") && e.title.contains("domain")), "expected missing domain");
    assert!(r2.iter().any(|e| e.title.contains("unexpected") && e.title.contains("services")), "expected unexpected services");
    assert_standard(&r2, &errors);
}

#[test]
fn missing_plus_loose() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/ports");
    write_file(tmp.path(), "apps/admin/src/modules/stray.ts", "// stray");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r2 = rule2_errors(&errors);
    assert_eq!(r2.len(), 2, "expected 2 errors (1 missing + 1 loose), got {}: {r2:#?}", r2.len());
    assert!(r2.iter().any(|e| e.title.contains("missing") && e.title.contains("ports")), "expected missing ports");
    assert!(r2.iter().any(|e| e.title.contains("loose files")), "expected loose files");
    assert_standard(&r2, &errors);
}

#[test]
fn all_three_violations() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/domain");
    write_file(tmp.path(), "apps/admin/src/modules/services/handler.ts", "");
    write_file(tmp.path(), "apps/admin/src/modules/stray.ts", "// stray");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r2 = rule2_errors(&errors);
    assert_eq!(r2.len(), 3, "expected 3 errors (missing + unexpected + loose), got {}: {r2:#?}", r2.len());
    assert!(r2.iter().any(|e| e.title.contains("missing")), "expected missing error");
    assert!(r2.iter().any(|e| e.title.contains("unexpected")), "expected unexpected error");
    assert!(r2.iter().any(|e| e.title.contains("loose files")), "expected loose files error");
    assert_standard(&r2, &errors);
}

// ============================================================================
// GROUP H: Edge cases — dir replaced by file
// ============================================================================

#[test]
fn required_dir_replaced_with_file() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/domain");
    write_file(tmp.path(), "apps/admin/src/modules/domain", "not a directory");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r2 = rule2_errors(&errors);
    // "domain" as a file: list_dir_names won't see it (not is_dir), so "missing" fires.
    // check_loose_files sees it (not .gitkeep), so "loose files" fires.
    assert_eq!(r2.len(), 2, "expected 2 errors (missing + loose), got {}: {r2:#?}", r2.len());
    assert!(r2.iter().any(|e| e.title.contains("missing") && e.title.contains("domain")), "expected missing domain");
    assert!(r2.iter().any(|e| e.title.contains("loose files")), "expected loose files");
    assert!(
        r2.iter().filter(|e| e.title.contains("loose files")).any(|e| e.message.contains("domain")),
        "loose-file message should list 'domain'"
    );
    assert_standard(&r2, &errors);
}

#[test]
fn file_coexists_with_same_named_dir() {
    let tmp = copy_fixture();
    // File named "adapters.bak" alongside real adapters/ dir
    write_file(tmp.path(), "apps/admin/src/modules/adapters.bak", "backup");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r2 = rule2_errors(&errors);
    // Only loose-file error (adapters.bak is a file, not a dir)
    assert_eq!(r2.len(), 1, "expected 1 loose-file error, got {}: {r2:#?}", r2.len());
    assert!(r2[0].title.contains("loose files"), "expected loose files error");
    assert!(r2[0].message.contains("adapters.bak"), "expected adapters.bak in message");
    // No missing-dir error (adapters/ still exists as a dir)
    assert_standard(&r2, &errors);
}

// ============================================================================
// GROUP I: Edge cases — symlinks
// ============================================================================

#[cfg(unix)]
#[test]
fn required_dir_replaced_with_symlink() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/domain");
    std::os::unix::fs::symlink(
        tmp.path().join("apps/admin/src/modules/application"),
        tmp.path().join("apps/admin/src/modules/domain"),
    )
    .expect("create symlink");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r2 = rule2_errors(&errors);
    // DirEntry::file_type() does NOT follow symlinks — returns symlink type.
    // Symlink is !is_dir() → list_dir_names skips it → "missing" fires.
    // check_loose_files sees it (not .gitkeep, not dir) → "loose files" fires.
    assert_eq!(
        r2.len(),
        2,
        "expected 2 errors (missing + loose) for symlink replacing dir, got {}: {r2:#?}",
        r2.len()
    );
    assert!(r2.iter().any(|e| e.title.contains("missing") && e.title.contains("domain")), "expected missing domain");
    assert!(r2.iter().any(|e| e.title.contains("loose files")), "expected loose files");
    assert_standard(&r2, &errors);
}

#[cfg(unix)]
#[test]
fn required_dir_is_broken_symlink() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/domain");
    std::os::unix::fs::symlink(
        "/nonexistent/path",
        tmp.path().join("apps/admin/src/modules/domain"),
    )
    .expect("create broken symlink");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r2 = rule2_errors(&errors);
    assert_eq!(r2.len(), 2, "expected 2 errors (missing + loose) for broken symlink, got {}: {r2:#?}", r2.len());
    assert!(r2.iter().any(|e| e.title.contains("missing") && e.title.contains("domain")), "expected missing domain");
    assert!(r2.iter().any(|e| e.title.contains("loose files")), "expected loose files");
    assert_standard(&r2, &errors);
}

// ============================================================================
// GROUP J: Edge cases — empty required dir
// ============================================================================

#[test]
fn empty_required_dir_passes_rule2() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/domain");
    std::fs::create_dir_all(tmp.path().join("apps/admin/src/modules/domain")).expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r2 = rule2_errors(&errors);
    assert_eq!(r2.len(), 0, "empty dir should pass rule 02, got {}: {r2:#?}", r2.len());
}

// ============================================================================
// GROUP K: Near-miss dir names
// ============================================================================

#[test]
fn near_miss_dir_names() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/domain");
    remove_dir(tmp.path(), "apps/admin/src/modules/ports");
    remove_dir(tmp.path(), "apps/admin/src/modules/adapters");
    remove_dir(tmp.path(), "apps/admin/src/modules/application");
    // Near-misses: plural/singular confusion, wrong names
    std::fs::create_dir_all(tmp.path().join("apps/admin/src/modules/domains")).expect("mkdir");
    std::fs::create_dir_all(tmp.path().join("apps/admin/src/modules/port")).expect("mkdir");
    std::fs::create_dir_all(tmp.path().join("apps/admin/src/modules/adapter")).expect("mkdir");
    std::fs::create_dir_all(tmp.path().join("apps/admin/src/modules/app")).expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r2 = rule2_errors(&errors);
    // 4 missing (real names) + 4 unexpected (near-misses) = 8 errors
    assert_eq!(r2.len(), 8, "expected 8 errors (4 missing + 4 unexpected), got {}: {r2:#?}", r2.len());
    for dir in EXPECTED_DIRS {
        assert!(
            r2.iter().any(|e| e.title.contains("missing") && e.title.contains(dir)),
            "expected missing '{dir}' error, got: {r2:#?}"
        );
    }
    for name in &["domains", "port", "adapter", "app"] {
        assert!(
            r2.iter().any(|e| e.title.contains("unexpected") && e.title.contains(name)),
            "expected unexpected '{name}' error, got: {r2:#?}"
        );
    }
    assert_standard(&r2, &errors);
}

#[test]
fn unicode_lookalike_dir_name() {
    let tmp = copy_fixture();
    // Zero-width space in dir name: "d\u{200B}omain" looks like "domain" but isn't
    remove_dir(tmp.path(), "apps/admin/src/modules/domain");
    std::fs::create_dir_all(tmp.path().join("apps/admin/src/modules/d\u{200B}omain")).expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r2 = rule2_errors(&errors);
    // "domain" is missing (the lookalike doesn't match), plus unexpected dir
    assert_eq!(r2.len(), 2, "expected 2 errors (missing domain + unexpected lookalike), got {}: {r2:#?}", r2.len());
    assert!(r2.iter().any(|e| e.title.contains("missing") && e.title.contains("domain")), "expected missing domain");
    assert!(r2.iter().any(|e| e.title.contains("unexpected")), "expected unexpected dir");
    assert_standard(&r2, &errors);
}

// ============================================================================
// GROUP L: Cross-language isolation and false positives
// ============================================================================

#[test]
fn rust_apps_not_flagged() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/domain");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_no_rust_apps(&errors);
}

#[test]
fn packages_not_flagged() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/domain");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_no_packages(&errors);
}

#[test]
fn landing_not_flagged() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/domain");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_no_landing(&errors);
}

// ============================================================================
// GROUP M: New app
// ============================================================================

#[test]
fn new_ts_app_with_broken_modules() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/dashboard/package.json", r#"{"name": "dashboard"}"#);
    std::fs::create_dir_all(tmp.path().join("apps/dashboard/src/modules/application")).expect("mkdir");
    std::fs::create_dir_all(tmp.path().join("apps/dashboard/src/modules/adapters/inbound")).expect("mkdir");
    std::fs::create_dir_all(tmp.path().join("apps/dashboard/src/modules/adapters/outbound")).expect("mkdir");
    std::fs::create_dir_all(tmp.path().join("apps/dashboard/src/modules/ports/inbound")).expect("mkdir");
    std::fs::create_dir_all(tmp.path().join("apps/dashboard/src/modules/ports/outbound")).expect("mkdir");
    // Missing domain/ (application, adapters, ports are created above) → errors for dashboard
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let dashboard_errors: Vec<_> = errors.iter().filter(|e| e.title.contains("dashboard")).collect();
    assert!(
        dashboard_errors.iter().any(|e| e.title.contains("missing") && e.title.contains("domain")),
        "expected missing domain/ for dashboard, got: {dashboard_errors:#?}"
    );
    // admin should still be clean (golden structure)
    let r2 = rule2_errors(&errors);
    let admin_r2: Vec<_> = r2.iter().filter(|e| e.title.contains("admin")).collect();
    assert_eq!(
        admin_r2.len(),
        0,
        "admin should have 0 rule-02 errors, got: {admin_r2:#?}"
    );
    assert_no_rust_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn new_ts_app_with_valid_modules() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/dashboard/package.json", r#"{"name": "dashboard"}"#);
    for dir in EXPECTED_DIRS {
        std::fs::create_dir_all(
            tmp.path().join(format!("apps/dashboard/src/modules/{dir}")),
        )
        .expect("mkdir");
    }
    std::fs::create_dir_all(tmp.path().join("apps/dashboard/src/modules/adapters/inbound")).expect("mkdir");
    std::fs::create_dir_all(tmp.path().join("apps/dashboard/src/modules/adapters/outbound")).expect("mkdir");
    std::fs::create_dir_all(tmp.path().join("apps/dashboard/src/modules/ports/inbound")).expect("mkdir");
    std::fs::create_dir_all(tmp.path().join("apps/dashboard/src/modules/ports/outbound")).expect("mkdir");
    write_file(tmp.path(), "apps/dashboard/src/modules/domain/.gitkeep", "");
    write_file(tmp.path(), "apps/dashboard/src/modules/application/.gitkeep", "");
    write_file(tmp.path(), "apps/dashboard/src/modules/adapters/inbound/.gitkeep", "");
    write_file(tmp.path(), "apps/dashboard/src/modules/adapters/outbound/.gitkeep", "");
    write_file(tmp.path(), "apps/dashboard/src/modules/ports/inbound/.gitkeep", "");
    write_file(tmp.path(), "apps/dashboard/src/modules/ports/outbound/.gitkeep", "");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r2 = rule2_errors(&errors);
    let dashboard_r2: Vec<_> = r2.iter().filter(|e| e.title.contains("dashboard")).collect();
    assert_eq!(
        dashboard_r2.len(),
        0,
        "new app with valid modules/ should have 0 rule-02 errors, got: {dashboard_r2:#?}"
    );
    assert_no_rust_apps(&errors);
    assert_no_packages(&errors);
}

// ============================================================================
// GROUP N: Nested unexpected dir (no recursion into garbage)
// ============================================================================

#[test]
fn nested_unexpected_dir_only_top_flagged() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/src/modules/services/internal/deep/handler.ts",
        "",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r2 = rule2_errors(&errors);
    assert_eq!(r2.len(), 1, "expected 1 error (top-level services/ only), got {}: {r2:#?}", r2.len());
    assert!(
        r2[0].title.contains("services"),
        "expected error about 'services', got: '{}'",
        r2[0].title
    );
    assert!(
        !r2.iter().any(|e| e.title.contains("internal") || e.title.contains("deep")),
        "should not flag nested dirs inside unexpected dir, got: {r2:#?}"
    );
    assert_standard(&r2, &errors);
}

// ============================================================================
// GROUP O: Idempotency
// ============================================================================

#[test]
fn idempotent_results() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/domain");
    write_file(tmp.path(), "apps/admin/src/modules/services/a.ts", "");
    write_file(tmp.path(), "apps/admin/src/modules/stray.ts", "");
    let results_1 = run_check(tmp.path());
    let errors_1 = arch_errors(&results_1);
    let r2_1 = rule2_errors(&errors_1);
    let results_2 = run_check(tmp.path());
    let errors_2 = arch_errors(&results_2);
    let r2_2 = rule2_errors(&errors_2);
    assert_eq!(
        r2_1.len(),
        r2_2.len(),
        "idempotent: run 1 = {} errors, run 2 = {} errors",
        r2_1.len(),
        r2_2.len()
    );
    for e1 in &r2_1 {
        assert!(
            r2_2.iter().any(|e2| e2.title == e1.title),
            "error '{}' from run 1 missing in run 2",
            e1.title
        );
    }
}

// ============================================================================
// GROUP P: Filesystem permissions
// ============================================================================

#[cfg(unix)]
#[test]
fn modules_no_read_permission() {
    use std::os::unix::fs::PermissionsExt;
    let tmp = copy_fixture();
    let modules = tmp.path().join("apps/admin/src/modules");
    std::fs::set_permissions(&modules, std::fs::Permissions::from_mode(0o000)).expect("chmod");
    let results = run_check(tmp.path());
    std::fs::set_permissions(&modules, std::fs::Permissions::from_mode(0o755)).expect("chmod");
    let errors = arch_errors(&results);
    let r2 = rule2_errors(&errors);
    assert_eq!(
        r2.len(),
        4,
        "unreadable modules/ should produce 4 missing-dir errors, got {}: {r2:#?}",
        r2.len()
    );
    assert_all_mention_admin(&r2);
    assert_file_field_modules(&r2);
}
