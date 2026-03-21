use super::helpers::{
    arch_errors, assert_no_landing, assert_no_packages, assert_no_rust_apps, copy_fixture,
    remove_dir, run_check, write_file,
};
use guardrail3::domain::report::CheckResult;

// ============================================================================
// Rule 05: Container dirs must not be empty (need subdirs or .gitkeep)
//
// Exercised by: arch_helpers::check_container_not_empty called from validate_ts_container
//
// Behavior:
// - Container has subdirs → passes (non-empty)
// - Container has .gitkeep (no subdirs) → passes (gitkeep satisfies)
// - Container is truly empty (no subdirs, no .gitkeep, no files) → "empty container" error
// - Container has files but no subdirs, no .gitkeep → "empty container" error with file listing
//   (does NOT also fire "loose files" — double-fire fix)
// - Container missing entirely → metadata guard returns early (rule 02 handles)
// ============================================================================

const TS_SERVICE_APPS: &[&str] = &["admin", "portal"];

const CONTAINER_SUFFIXES: &[&str] = &[
    "application",
    "domain",
    "adapters/inbound",
    "adapters/outbound",
    "ports/inbound",
    "ports/outbound",
];

const PORTAL_PAYMENTS_BASE: &str = "apps/portal/src/modules/adapters/inbound/payments/modules";
const PORTAL_AICHAT_BASE: &str = "apps/portal/src/modules/adapters/outbound/ai-chat/modules";

/// Filter to only empty-container errors.
fn empty_errors<'a>(errors: &[&'a CheckResult]) -> Vec<&'a CheckResult> {
    errors
        .iter()
        .filter(|e| e.title.contains("empty container"))
        .copied()
        .collect()
}

/// Make a container empty (remove contents, recreate dir).
fn make_empty(tmp: &std::path::Path, rel: &str) {
    let path = tmp.join(rel);
    if path.exists() {
        std::fs::remove_dir_all(&path).expect("remove dir");
    }
    std::fs::create_dir_all(&path).expect("create empty dir");
}

/// Assert file field contains "modules".
fn assert_file_field_modules(errors: &[&CheckResult]) {
    for err in errors {
        let file = err.file.as_deref().unwrap_or("");
        assert!(
            file.contains("modules"),
            "expected file field containing 'modules', got: '{file}' for: '{}'",
            err.title
        );
    }
}

/// Assert every error mentions a TS service app.
fn assert_mentions_ts_app(errors: &[&CheckResult]) {
    for err in errors {
        assert!(
            TS_SERVICE_APPS.iter().any(|app| err.title.contains(app)),
            "expected error to mention a TS service app, got: '{}'",
            err.title
        );
    }
}

/// Standard assertion battery.
fn assert_standard(empty: &[&CheckResult], all_errors: &[&CheckResult]) {
    assert_mentions_ts_app(empty);
    assert_file_field_modules(empty);
    assert_no_rust_apps(all_errors);
    assert_no_packages(all_errors);
    assert_no_landing(all_errors);
}

/// Standard for admin-only tests.
fn assert_standard_admin(empty: &[&CheckResult], all_errors: &[&CheckResult]) {
    for err in empty {
        assert!(err.title.contains("admin"), "expected 'admin' in title, got: '{}'", err.title);
    }
    assert_file_field_modules(empty);
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
    let empty = empty_errors(&errors);
    assert_eq!(empty.len(), 0, "golden should have 0 empty-container errors, got {}: {empty:#?}", empty.len());
}

// ============================================================================
// GROUP B: Empty individual containers (admin)
// ============================================================================

#[test]
fn empty_domain() {
    let tmp = copy_fixture();
    make_empty(tmp.path(), "apps/admin/src/modules/domain");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let empty = empty_errors(&errors);
    assert_eq!(empty.len(), 1, "expected 1 empty-container error, got {}: {empty:#?}", empty.len());
    assert!(empty[0].title.contains("domain"), "expected domain in title");
    assert!(empty[0].message.contains("is empty"), "expected 'is empty' in message");
    assert_standard_admin(&empty, &errors);
}

#[test]
fn empty_application() {
    let tmp = copy_fixture();
    make_empty(tmp.path(), "apps/admin/src/modules/application");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let empty = empty_errors(&errors);
    assert_eq!(empty.len(), 1, "expected 1 empty-container error, got {}: {empty:#?}", empty.len());
    assert!(empty[0].title.contains("application"), "expected application in title");
    assert_standard_admin(&empty, &errors);
}

#[test]
fn empty_adapters_inbound() {
    let tmp = copy_fixture();
    make_empty(tmp.path(), "apps/admin/src/modules/adapters/inbound");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let empty = empty_errors(&errors);
    assert_eq!(empty.len(), 1, "expected 1 empty-container error, got {}: {empty:#?}", empty.len());
    assert!(empty[0].title.contains("adapters") && empty[0].title.contains("inbound"), "expected adapters/inbound");
    assert_standard_admin(&empty, &errors);
}

#[test]
fn empty_ports_outbound() {
    let tmp = copy_fixture();
    make_empty(tmp.path(), "apps/admin/src/modules/ports/outbound");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let empty = empty_errors(&errors);
    assert_eq!(empty.len(), 1, "expected 1 empty-container error, got {}: {empty:#?}", empty.len());
    assert!(empty[0].title.contains("ports") && empty[0].title.contains("outbound"), "expected ports/outbound");
    assert_standard_admin(&empty, &errors);
}

// ============================================================================
// GROUP C: All containers empty
// ============================================================================

#[test]
fn all_admin_containers_empty() {
    let tmp = copy_fixture();
    for suffix in CONTAINER_SUFFIXES {
        make_empty(tmp.path(), &format!("apps/admin/src/modules/{suffix}"));
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let empty = empty_errors(&errors);
    assert_eq!(empty.len(), 6, "expected 6 empty-container errors, got {}: {empty:#?}", empty.len());
    assert_standard_admin(&empty, &errors);
}

#[test]
fn all_containers_empty_both_apps() {
    let tmp = copy_fixture();
    for app in TS_SERVICE_APPS {
        for suffix in CONTAINER_SUFFIXES {
            make_empty(tmp.path(), &format!("apps/{app}/src/modules/{suffix}"));
        }
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let empty = empty_errors(&errors);
    // 6 containers * 2 apps = 12
    // (emptying portal's adapters/inbound destroys payments inner hex — it becomes unreachable)
    // (emptying portal's adapters/outbound destroys ai-chat inner hex — same)
    assert_eq!(empty.len(), 12, "expected 12 empty-container errors (6 per app), got {}: {empty:#?}", empty.len());
    for app in TS_SERVICE_APPS {
        assert!(
            empty.iter().any(|e| e.title.contains(app)),
            "expected error for app '{app}', got: {empty:#?}"
        );
    }
    assert_standard(&empty, &errors);
}

// ============================================================================
// GROUP D: .gitkeep satisfies non-empty
// ============================================================================

#[test]
fn gitkeep_satisfies_non_empty() {
    let tmp = copy_fixture();
    make_empty(tmp.path(), "apps/admin/src/modules/domain");
    write_file(tmp.path(), "apps/admin/src/modules/domain/.gitkeep", "");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let empty = empty_errors(&errors);
    let domain_empty: Vec<_> = empty.iter().filter(|e| e.title.contains("domain")).collect();
    assert_eq!(domain_empty.len(), 0, ".gitkeep should satisfy non-empty for domain/, got: {domain_empty:#?}");
}

#[test]
fn gitkeep_in_all_containers() {
    let tmp = copy_fixture();
    for suffix in CONTAINER_SUFFIXES {
        make_empty(tmp.path(), &format!("apps/admin/src/modules/{suffix}"));
        write_file(tmp.path(), &format!("apps/admin/src/modules/{suffix}/.gitkeep"), "");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let empty = empty_errors(&errors);
    let admin_empty: Vec<_> = empty.iter().filter(|e| e.title.contains("admin")).collect();
    assert_eq!(admin_empty.len(), 0, ".gitkeep should satisfy non-empty for all containers, got: {admin_empty:#?}");
}

#[test]
fn subdir_satisfies_non_empty() {
    // Golden fixture already has subdirs in all containers — this is the golden test
    let tmp = copy_fixture();
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let empty = empty_errors(&errors);
    assert_eq!(empty.len(), 0, "golden (subdirs present) should have 0 empty-container errors, got: {empty:#?}");
}

// ============================================================================
// GROUP E: Container with files but no subdirs
// ============================================================================

#[test]
fn files_but_no_subdirs() {
    let tmp = copy_fixture();
    make_empty(tmp.path(), "apps/admin/src/modules/domain");
    write_file(tmp.path(), "apps/admin/src/modules/domain/stray.ts", "// stray");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let empty = empty_errors(&errors);
    let domain_empty: Vec<_> = empty.iter().filter(|e| e.title.contains("domain")).copied().collect();
    assert_eq!(domain_empty.len(), 1, "expected 1 empty-container error with file listing, got: {domain_empty:#?}");
    // Message should list the file
    assert!(
        domain_empty[0].message.contains("stray.ts"),
        "expected stray.ts in message, got: '{}'",
        domain_empty[0].message
    );
    assert!(
        domain_empty[0].message.contains("but no subdirectories"),
        "expected 'but no subdirectories' in message, got: '{}'",
        domain_empty[0].message
    );
    assert_standard_admin(&domain_empty, &errors);
}

#[test]
fn multiple_files_but_no_subdirs() {
    let tmp = copy_fixture();
    make_empty(tmp.path(), "apps/admin/src/modules/domain");
    write_file(tmp.path(), "apps/admin/src/modules/domain/index.ts", "");
    write_file(tmp.path(), "apps/admin/src/modules/domain/types.ts", "");
    write_file(tmp.path(), "apps/admin/src/modules/domain/.env", "");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let empty = empty_errors(&errors);
    let domain_empty: Vec<_> = empty.iter().filter(|e| e.title.contains("domain")).collect();
    assert_eq!(domain_empty.len(), 1, "expected 1 empty-container error listing all files");
    assert!(domain_empty[0].message.contains("index.ts"), "expected index.ts");
    assert!(domain_empty[0].message.contains("types.ts"), "expected types.ts");
    assert!(domain_empty[0].message.contains(".env"), "expected .env");
    let domain_empty_owned: Vec<_> = domain_empty.into_iter().copied().collect();
    assert_standard_admin(&domain_empty_owned, &errors);
}

// ============================================================================
// GROUP F: Missing container (metadata guard)
// ============================================================================

#[test]
fn missing_container_no_empty_error() {
    let tmp = copy_fixture();
    // Remove domain/ entirely — metadata returns None → early return
    remove_dir(tmp.path(), "apps/admin/src/modules/domain");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let empty = empty_errors(&errors);
    let domain_empty: Vec<_> = empty.iter().filter(|e| e.title.contains("domain")).collect();
    assert_eq!(
        domain_empty.len(),
        0,
        "missing container should not produce empty-container error (rule 02 handles), got: {domain_empty:#?}"
    );
}

// ============================================================================
// GROUP G: Inner hex containers
// ============================================================================

#[test]
fn empty_inner_hex_container() {
    let tmp = copy_fixture();
    make_empty(tmp.path(), &format!("{PORTAL_PAYMENTS_BASE}/domain"));
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let empty = empty_errors(&errors);
    let inner_empty: Vec<_> = empty
        .iter()
        .filter(|e| e.file.as_deref().unwrap_or("").contains("payments/modules/domain"))
        .collect();
    assert_eq!(inner_empty.len(), 1, "expected 1 empty error in payments inner hex domain/, got: {inner_empty:#?}");
    assert!(inner_empty[0].title.contains("portal"), "expected portal in title");
    // Label prefix should reflect the nested path
    assert!(
        inner_empty[0].title.contains("payments/modules/domain"),
        "expected nested label in title, got: '{}'",
        inner_empty[0].title
    );
    assert_no_rust_apps(&errors);
    assert_no_packages(&errors);
    assert_no_landing(&errors);
}

#[test]
fn all_inner_hex_containers_empty() {
    let tmp = copy_fixture();
    for suffix in CONTAINER_SUFFIXES {
        make_empty(tmp.path(), &format!("{PORTAL_PAYMENTS_BASE}/{suffix}"));
        make_empty(tmp.path(), &format!("{PORTAL_AICHAT_BASE}/{suffix}"));
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let empty = empty_errors(&errors);
    let inner_empty: Vec<_> = empty
        .iter()
        .filter(|e| {
            let f = e.file.as_deref().unwrap_or("");
            f.contains("payments/modules") || f.contains("ai-chat/modules")
        })
        .collect();
    // 6 containers * 2 inner hexes = 12
    assert_eq!(inner_empty.len(), 12, "expected 12 empty inner hex container errors, got {}: {inner_empty:#?}", inner_empty.len());
    // Outer should be clean
    let admin_empty: Vec<_> = empty.iter().filter(|e| e.title.contains("admin")).collect();
    assert_eq!(admin_empty.len(), 0, "admin should have 0 empty errors: {admin_empty:#?}");
    assert_no_rust_apps(&errors);
    assert_no_packages(&errors);
    assert_no_landing(&errors);
}

// ============================================================================
// GROUP N: Double-fire prevention boundary
// ============================================================================

#[test]
fn no_double_fire_files_but_no_subdirs() {
    let tmp = copy_fixture();
    // Container with files but no subdirs, no .gitkeep
    make_empty(tmp.path(), "apps/admin/src/modules/domain");
    write_file(tmp.path(), "apps/admin/src/modules/domain/stray.ts", "// stray");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    // "empty container" fires (with file listing)
    let empty = empty_errors(&errors);
    let domain_empty: Vec<_> = empty.iter().filter(|e| e.title.contains("domain")).collect();
    assert_eq!(domain_empty.len(), 1, "expected 1 empty-container error: {domain_empty:#?}");
    assert!(domain_empty[0].message.contains("stray.ts"), "expected stray.ts in message");
    // "loose files" does NOT fire (double-fire prevention)
    let loose: Vec<_> = errors.iter().filter(|e| e.title.contains("loose files") && e.file.as_deref().unwrap_or("").contains("domain")).collect();
    assert_eq!(loose.len(), 0, "should NOT have loose-files error (double-fire prevention), got: {loose:#?}");
    assert_no_rust_apps(&errors);
    assert_no_packages(&errors);
    assert_no_landing(&errors);
}

#[test]
fn subdirs_plus_loose_files_only_loose_fires() {
    let tmp = copy_fixture();
    // Container with subdirs (from golden) + loose file → only "loose files", NOT "empty container"
    write_file(tmp.path(), "apps/admin/src/modules/domain/stray.ts", "// stray");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    // "empty container" does NOT fire (has subdirs)
    let empty = empty_errors(&errors);
    let domain_empty: Vec<_> = empty.iter().filter(|e| e.title.contains("domain")).collect();
    assert_eq!(domain_empty.len(), 0, "should NOT have empty-container error (has subdirs), got: {domain_empty:#?}");
    // "loose files" DOES fire
    let loose: Vec<_> = errors.iter().filter(|e| e.title.contains("loose files") && e.file.as_deref().unwrap_or("").contains("domain")).collect();
    assert_eq!(loose.len(), 1, "expected 1 loose-files error: {loose:#?}");
}

#[test]
fn gitkeep_plus_loose_files_no_subdirs() {
    let tmp = copy_fixture();
    // .gitkeep + loose file + no subdirs → "loose files" fires, NOT "empty container"
    make_empty(tmp.path(), "apps/admin/src/modules/domain");
    write_file(tmp.path(), "apps/admin/src/modules/domain/.gitkeep", "");
    write_file(tmp.path(), "apps/admin/src/modules/domain/stray.ts", "// stray");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    // "empty container" does NOT fire (.gitkeep makes it non-empty)
    let empty = empty_errors(&errors);
    let domain_empty: Vec<_> = empty.iter().filter(|e| e.title.contains("domain")).collect();
    assert_eq!(domain_empty.len(), 0, ".gitkeep should prevent empty-container error, got: {domain_empty:#?}");
    // "loose files" DOES fire (stray.ts is not .gitkeep)
    let loose: Vec<_> = errors.iter().filter(|e| e.title.contains("loose files") && e.file.as_deref().unwrap_or("").contains("domain")).collect();
    assert_eq!(loose.len(), 1, "expected 1 loose-files error for stray.ts, got: {loose:#?}");
}

// ============================================================================
// GROUP O: Additional edge cases
// ============================================================================

#[test]
fn gitkeep_as_directory_does_not_satisfy() {
    let tmp = copy_fixture();
    make_empty(tmp.path(), "apps/admin/src/modules/domain");
    // mkdir .gitkeep (directory, not file) — has_gitkeep checks read_file which fails for dirs
    std::fs::create_dir_all(tmp.path().join("apps/admin/src/modules/domain/.gitkeep")).expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let empty = empty_errors(&errors);
    // .gitkeep as a dir: has_gitkeep → read_file returns None → not satisfied
    // But .gitkeep/ IS a subdir → dir_names is non-empty → empty-container check passes!
    let domain_empty: Vec<_> = empty.iter().filter(|e| e.title.contains("domain")).collect();
    assert_eq!(domain_empty.len(), 0, ".gitkeep dir is a subdir → non-empty, got: {domain_empty:#?}");
}

#[test]
fn gitkeep_wrong_case_does_not_satisfy() {
    let tmp = copy_fixture();
    make_empty(tmp.path(), "apps/admin/src/modules/domain");
    write_file(tmp.path(), "apps/admin/src/modules/domain/.GITKEEP", "");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let empty = empty_errors(&errors);
    let domain_empty: Vec<_> = empty.iter().filter(|e| e.title.contains("domain")).collect();
    // On case-sensitive FS: .GITKEEP != .gitkeep → not satisfied → empty-container fires
    // On case-insensitive FS (macOS): .GITKEEP resolves as .gitkeep → satisfied → 0 errors
    assert!(
        domain_empty.len() == 0 || domain_empty.len() == 1,
        "expected 0 (case-insensitive) or 1 (case-sensitive) errors, got: {domain_empty:#?}"
    );
}

#[test]
fn unicode_lookalike_gitkeep_does_not_satisfy() {
    let tmp = copy_fixture();
    make_empty(tmp.path(), "apps/admin/src/modules/domain");
    write_file(tmp.path(), "apps/admin/src/modules/domain/.git\u{200B}keep", "");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let empty = empty_errors(&errors);
    let domain_empty: Vec<_> = empty.iter().filter(|e| e.title.contains("domain")).collect();
    assert_eq!(domain_empty.len(), 1, "unicode lookalike should NOT satisfy non-empty, got: {domain_empty:#?}");
}

#[cfg(unix)]
#[test]
fn dangling_symlink_as_only_content() {
    let tmp = copy_fixture();
    make_empty(tmp.path(), "apps/admin/src/modules/domain");
    std::os::unix::fs::symlink(
        "/nonexistent",
        tmp.path().join("apps/admin/src/modules/domain/dangling"),
    )
    .expect("create dangling symlink");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let empty = empty_errors(&errors);
    let domain_empty: Vec<_> = empty.iter().filter(|e| e.title.contains("domain")).collect();
    // Dangling symlink: !is_dir → not a subdir; not .gitkeep → empty-container fires
    assert_eq!(domain_empty.len(), 1, "expected 1 empty-container error for dangling symlink, got: {domain_empty:#?}");
}

#[test]
fn hidden_file_as_sole_content() {
    let tmp = copy_fixture();
    make_empty(tmp.path(), "apps/admin/src/modules/domain");
    write_file(tmp.path(), "apps/admin/src/modules/domain/.DS_Store", "");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let empty = empty_errors(&errors);
    let domain_empty: Vec<_> = empty.iter().filter(|e| e.title.contains("domain")).collect();
    assert_eq!(domain_empty.len(), 1, "expected 1 empty-container error for .DS_Store, got: {domain_empty:#?}");
    assert!(domain_empty[0].message.contains(".DS_Store"), "expected .DS_Store in message");
    let domain_empty_owned: Vec<_> = domain_empty.into_iter().copied().collect();
    assert_standard_admin(&domain_empty_owned, &errors);
}

#[test]
fn empty_in_one_app_valid_in_another() {
    let tmp = copy_fixture();
    // admin domain/ empty, portal domain/ untouched (golden, has subdirs)
    make_empty(tmp.path(), "apps/admin/src/modules/domain");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let empty = empty_errors(&errors);
    let admin_empty: Vec<_> = empty.iter().filter(|e| e.title.contains("admin")).collect();
    let portal_empty: Vec<_> = empty.iter().filter(|e| e.title.contains("portal")).collect();
    assert_eq!(admin_empty.len(), 1, "expected 1 admin empty error: {admin_empty:#?}");
    assert_eq!(portal_empty.len(), 0, "portal should have 0 empty errors: {portal_empty:#?}");
    assert_no_rust_apps(&errors);
    assert_no_packages(&errors);
    assert_no_landing(&errors);
}

// ============================================================================
// GROUP H: Edge cases
// ============================================================================

#[test]
fn gitkeep_with_content_satisfies() {
    let tmp = copy_fixture();
    make_empty(tmp.path(), "apps/admin/src/modules/domain");
    write_file(tmp.path(), "apps/admin/src/modules/domain/.gitkeep", "this has content");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let empty = empty_errors(&errors);
    let domain_empty: Vec<_> = empty.iter().filter(|e| e.title.contains("domain")).collect();
    assert_eq!(domain_empty.len(), 0, ".gitkeep with content should satisfy non-empty, got: {domain_empty:#?}");
}

#[test]
fn near_miss_gitkeep_does_not_satisfy() {
    let tmp = copy_fixture();
    make_empty(tmp.path(), "apps/admin/src/modules/domain");
    write_file(tmp.path(), "apps/admin/src/modules/domain/.git_keep", "");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let empty = empty_errors(&errors);
    let domain_empty: Vec<_> = empty.iter().filter(|e| e.title.contains("domain")).collect();
    // .git_keep is NOT .gitkeep — container is treated as having files but no subdirs
    assert_eq!(
        domain_empty.len(),
        1,
        ".git_keep should NOT satisfy non-empty, got: {domain_empty:#?}"
    );
    assert!(domain_empty[0].message.contains(".git_keep"), "expected .git_keep in message");
}

#[cfg(unix)]
#[test]
fn empty_container_symlink_to_empty_dir() {
    let tmp = copy_fixture();
    make_empty(tmp.path(), "apps/admin/src/modules/domain");
    let target = tmp.path().join("empty_target");
    std::fs::create_dir_all(&target).expect("mkdir target");
    // Replace domain/ with symlink to empty dir
    std::fs::remove_dir(tmp.path().join("apps/admin/src/modules/domain")).expect("rmdir");
    std::os::unix::fs::symlink(&target, tmp.path().join("apps/admin/src/modules/domain"))
        .expect("symlink");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let empty = empty_errors(&errors);
    let domain_empty: Vec<_> = empty.iter().filter(|e| e.title.contains("domain")).collect();
    // Symlink to empty dir: metadata succeeds, list_dir_names returns empty, no .gitkeep → "empty container"
    assert_eq!(domain_empty.len(), 1, "expected 1 empty-container error for symlink to empty dir, got: {domain_empty:#?}");
}

#[cfg(unix)]
#[test]
fn container_no_read_permission() {
    use std::os::unix::fs::PermissionsExt;
    let tmp = copy_fixture();
    let domain = tmp.path().join("apps/admin/src/modules/domain");
    std::fs::set_permissions(&domain, std::fs::Permissions::from_mode(0o000)).expect("chmod");
    let results = run_check(tmp.path());
    std::fs::set_permissions(&domain, std::fs::Permissions::from_mode(0o755)).expect("chmod");
    let errors = arch_errors(&results);
    let empty = empty_errors(&errors);
    let domain_empty: Vec<_> = empty.iter().filter(|e| e.title.contains("domain")).copied().collect();
    // Unreadable: list_dir_names returns empty, has_gitkeep fails → "empty container"
    assert_eq!(domain_empty.len(), 1, "unreadable domain/ should produce empty-container error, got: {domain_empty:#?}");
    assert_standard_admin(&domain_empty, &errors);
}

// ============================================================================
// GROUP I: Cross-language isolation
// ============================================================================

#[test]
fn rust_apps_not_flagged() {
    let tmp = copy_fixture();
    make_empty(tmp.path(), "apps/admin/src/modules/domain");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_no_rust_apps(&errors);
}

#[test]
fn packages_not_flagged() {
    let tmp = copy_fixture();
    make_empty(tmp.path(), "apps/admin/src/modules/domain");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_no_packages(&errors);
}

#[test]
fn landing_not_flagged() {
    let tmp = copy_fixture();
    make_empty(tmp.path(), "apps/admin/src/modules/domain");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_no_landing(&errors);
}

// ============================================================================
// GROUP J: Idempotency
// ============================================================================

#[test]
fn idempotent_results() {
    let tmp = copy_fixture();
    for suffix in CONTAINER_SUFFIXES {
        make_empty(tmp.path(), &format!("apps/admin/src/modules/{suffix}"));
    }
    let results_1 = run_check(tmp.path());
    let errors_1 = arch_errors(&results_1);
    let empty_1 = empty_errors(&errors_1);
    assert!(empty_1.len() > 0, "precondition: expected empty-container errors");
    let results_2 = run_check(tmp.path());
    let errors_2 = arch_errors(&results_2);
    let empty_2 = empty_errors(&errors_2);
    assert_eq!(
        empty_1.len(),
        empty_2.len(),
        "idempotent: run 1 = {}, run 2 = {}",
        empty_1.len(),
        empty_2.len()
    );
    for e1 in &empty_1 {
        assert!(
            empty_2.iter().any(|e2| e2.title == e1.title),
            "error '{}' from run 1 missing in run 2",
            e1.title
        );
    }
    assert_standard_admin(&empty_1, &errors_1);
}

// ============================================================================
// GROUP K: Per-app attribution
// ============================================================================

#[test]
fn per_app_attribution() {
    let tmp = copy_fixture();
    // Empty one container in each app
    make_empty(tmp.path(), "apps/admin/src/modules/domain");
    make_empty(tmp.path(), "apps/portal/src/modules/domain");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let empty = empty_errors(&errors);
    assert_eq!(empty.len(), 2, "expected 2 empty-container errors (1 per app), got {}: {empty:#?}", empty.len());
    for app in TS_SERVICE_APPS {
        assert!(
            empty.iter().any(|e| e.title.contains(app)),
            "expected error for app '{app}', got: {empty:#?}"
        );
    }
    assert_standard(&empty, &errors);
}

// ============================================================================
// GROUP L: Different breakage per app
// ============================================================================

#[test]
fn different_breakage_per_app() {
    let tmp = copy_fixture();
    // admin: truly empty domain/
    make_empty(tmp.path(), "apps/admin/src/modules/domain");
    // portal: domain/ with files but no subdirs
    make_empty(tmp.path(), "apps/portal/src/modules/domain");
    write_file(tmp.path(), "apps/portal/src/modules/domain/stray.ts", "// stray");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let empty = empty_errors(&errors);
    let admin_empty: Vec<_> = empty.iter().filter(|e| e.title.contains("admin")).collect();
    let portal_empty: Vec<_> = empty.iter().filter(|e| e.title.contains("portal")).collect();
    assert_eq!(admin_empty.len(), 1, "expected 1 admin empty error: {admin_empty:#?}");
    assert_eq!(portal_empty.len(), 1, "expected 1 portal empty error: {portal_empty:#?}");
    // admin message: "is empty"
    assert!(admin_empty[0].message.contains("is empty"), "admin should say 'is empty'");
    // portal message: "contains files (stray.ts) but no subdirectories"
    assert!(portal_empty[0].message.contains("stray.ts"), "portal should list stray.ts");
    assert!(portal_empty[0].message.contains("but no subdirectories"), "portal should say 'but no subdirectories'");
    assert_standard(&empty, &errors);
}

// ============================================================================
// GROUP M: New app
// ============================================================================

#[test]
fn new_app_gets_checked() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/dashboard/package.json", r#"{"name": "dashboard"}"#);
    // Create hex arch — CONTAINER_SUFFIXES includes adapters/inbound etc. via create_dir_all
    for suffix in CONTAINER_SUFFIXES {
        std::fs::create_dir_all(tmp.path().join(format!("apps/dashboard/src/modules/{suffix}"))).expect("mkdir");
    }
    // All containers are empty → 6 empty-container errors for dashboard
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let empty = empty_errors(&errors);
    let dashboard_empty: Vec<_> = empty.iter().filter(|e| e.title.contains("dashboard")).collect();
    assert_eq!(dashboard_empty.len(), 6, "expected 6 empty-container errors for dashboard, got: {dashboard_empty:#?}");
    // admin + portal should still be clean
    let admin_empty: Vec<_> = empty.iter().filter(|e| e.title.contains("admin")).collect();
    assert_eq!(admin_empty.len(), 0, "admin should have 0 empty errors: {admin_empty:#?}");
    assert_no_rust_apps(&errors);
    assert_no_packages(&errors);
    assert_no_landing(&errors);
}

// ============================================================================
// GROUP P: RS parity edge cases (round 3)
// ============================================================================

#[test]
fn file_replacing_subdir() {
    let tmp = copy_fixture();
    // Replace domain/types/ (a subdir) with a file named "types"
    remove_dir(tmp.path(), "apps/admin/src/modules/domain/types");
    write_file(tmp.path(), "apps/admin/src/modules/domain/types", "not a directory");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let empty = empty_errors(&errors);
    let domain_empty: Vec<_> = empty.iter().filter(|e| e.title.contains("domain")).copied().collect();
    // "types" as a file: not is_dir → not a subdir. No other subdirs remain.
    // has_gitkeep: no → empty-container fires with "contains files (types)"
    assert_eq!(domain_empty.len(), 1, "expected 1 empty-container error: {domain_empty:#?}");
    assert!(
        domain_empty[0].message.contains("types"),
        "expected 'types' in file listing, got: '{}'",
        domain_empty[0].message
    );
    assert!(
        domain_empty[0].message.contains("but no subdirectories"),
        "expected 'but no subdirectories', got: '{}'",
        domain_empty[0].message
    );
    assert_standard_admin(&domain_empty, &errors);
}

#[test]
fn maximally_complex_empty_container() {
    let tmp = copy_fixture();
    make_empty(tmp.path(), "apps/admin/src/modules/domain");
    // Add multiple junk files of different types — none should satisfy non-empty
    write_file(tmp.path(), "apps/admin/src/modules/domain/.DS_Store", "");
    write_file(tmp.path(), "apps/admin/src/modules/domain/.git_keep", ""); // near-miss
    write_file(tmp.path(), "apps/admin/src/modules/domain/README.md", "# docs");
    write_file(tmp.path(), "apps/admin/src/modules/domain/.env", "SECRET=1");
    write_file(tmp.path(), "apps/admin/src/modules/domain/.git\u{200B}keep", ""); // unicode lookalike
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let empty = empty_errors(&errors);
    let domain_empty: Vec<_> = empty.iter().filter(|e| e.title.contains("domain")).copied().collect();
    // 1 error listing all junk files
    assert_eq!(domain_empty.len(), 1, "expected 1 empty-container error listing all junk: {domain_empty:#?}");
    assert!(domain_empty[0].message.contains(".DS_Store"), "expected .DS_Store");
    assert!(domain_empty[0].message.contains(".git_keep"), "expected .git_keep");
    assert!(domain_empty[0].message.contains("README.md"), "expected README.md");
    assert!(domain_empty[0].message.contains(".env"), "expected .env");
    assert_standard_admin(&domain_empty, &errors);
}
