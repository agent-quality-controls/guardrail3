use super::helpers::{
    arch_errors, assert_no_landing, assert_no_packages, assert_no_rust_apps, copy_fixture,
    run_check, write_file,
};
use guardrail3_domain_report::CheckResult;

// ============================================================================
// Rule 04: Loose files in structural/container dirs (only .gitkeep allowed)
//
// Loose files are checked at ALL hex arch directory levels:
// - modules/ root (via check_exact_subdirs)
// - adapters/, ports/ structural dirs (via check_exact_subdirs in check_ts_inbound_outbound)
// - Container dirs: domain, application, adapters/{in,out}, ports/{in,out}
//   (via check_container_not_empty, which only calls check_loose_files when container has subdirs)
// - Inner hex modules/ and their structural/container dirs (recursion)
//
// check_container_not_empty design: if container is empty (no subdirs, no .gitkeep),
// fires "empty container" with file listing — does NOT also fire "loose files" (double-fire fix).
// "loose files" only fires when container HAS subdirs but also has stray files.
// ============================================================================

const TS_SERVICE_APPS: &[&str] = &["admin", "portal"];

/// Container suffixes relative to an app's modules/ dir.
const CONTAINER_SUFFIXES: &[&str] = &[
    "application",
    "domain",
    "adapters/inbound",
    "adapters/outbound",
    "ports/inbound",
    "ports/outbound",
];

/// All outer container paths for admin only.
fn admin_container_paths() -> Vec<String> {
    CONTAINER_SUFFIXES
        .iter()
        .map(|s| format!("apps/admin/src/modules/{s}"))
        .collect()
}

/// All container paths across BOTH TS service apps (outer hex only).
fn all_outer_container_paths() -> Vec<String> {
    let mut paths = Vec::new();
    for app in TS_SERVICE_APPS {
        for suffix in CONTAINER_SUFFIXES {
            paths.push(format!("apps/{app}/src/modules/{suffix}"));
        }
    }
    paths
}

/// Portal inner hex container paths.
const PORTAL_PAYMENTS_BASE: &str = "apps/portal/src/modules/adapters/inbound/payments/modules";
const PORTAL_AICHAT_BASE: &str = "apps/portal/src/modules/adapters/outbound/ai-chat/modules";

fn portal_inner_hex_container_paths() -> Vec<String> {
    let mut paths = Vec::new();
    for base in &[PORTAL_PAYMENTS_BASE, PORTAL_AICHAT_BASE] {
        for suffix in CONTAINER_SUFFIXES {
            paths.push(format!("{base}/{suffix}"));
        }
    }
    paths
}

/// Filter to only loose-file errors.
fn loose_file_errors<'a>(errors: &[&'a CheckResult]) -> Vec<&'a CheckResult> {
    errors
        .iter()
        .filter(|e| e.title.contains("loose files"))
        .copied()
        .collect()
}

/// Assert file field points to a modules-related path.
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

/// Assert every error mentions one of the TS service apps.
fn assert_mentions_ts_app(errors: &[&CheckResult]) {
    for err in errors {
        assert!(
            TS_SERVICE_APPS.iter().any(|app| err.title.contains(app)),
            "expected error to mention a TS service app, got: '{}'",
            err.title
        );
    }
}

/// Standard assertion battery for multi-app loose-file tests.
fn assert_standard(loose: &[&CheckResult], all_errors: &[&CheckResult]) {
    assert_mentions_ts_app(loose);
    assert_file_field_modules(loose);
    assert_no_rust_apps(all_errors);
    assert_no_packages(all_errors);
    assert_no_landing(all_errors);
}

/// Standard assertion battery for admin-only tests.
fn assert_standard_admin(loose: &[&CheckResult], all_errors: &[&CheckResult]) {
    for err in loose {
        assert!(
            err.title.contains("admin"),
            "expected error to mention 'admin', got: '{}'",
            err.title
        );
    }
    assert_file_field_modules(loose);
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
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        0,
        "golden should have 0 loose-file errors, got {}: {loose:#?}",
        loose.len()
    );
}

// ============================================================================
// GROUP B: Loose files in each container type (admin only)
// ============================================================================

#[test]
fn loose_file_in_domain() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/src/modules/domain/stray.ts",
        "// stray",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        1,
        "expected 1 loose-file error, got {}: {loose:#?}",
        loose.len()
    );
    assert!(
        loose[0].message.contains("stray.ts"),
        "expected stray.ts in message"
    );
    assert_standard_admin(&loose, &errors);
}

#[test]
fn loose_file_in_application() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/src/modules/application/stray.ts",
        "// stray",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        1,
        "expected 1 loose-file error, got {}: {loose:#?}",
        loose.len()
    );
    assert_standard_admin(&loose, &errors);
}

#[test]
fn loose_file_in_adapters_inbound() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/src/modules/adapters/inbound/stray.ts",
        "// stray",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        1,
        "expected 1 loose-file error, got {}: {loose:#?}",
        loose.len()
    );
    assert_standard_admin(&loose, &errors);
}

#[test]
fn loose_file_in_adapters_outbound() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/src/modules/adapters/outbound/stray.ts",
        "// stray",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        1,
        "expected 1 loose-file error, got {}: {loose:#?}",
        loose.len()
    );
    assert_standard_admin(&loose, &errors);
}

#[test]
fn loose_file_in_ports_inbound() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/src/modules/ports/inbound/stray.ts",
        "// stray",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        1,
        "expected 1 loose-file error, got {}: {loose:#?}",
        loose.len()
    );
    assert_standard_admin(&loose, &errors);
}

#[test]
fn loose_file_in_ports_outbound() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/src/modules/ports/outbound/stray.ts",
        "// stray",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        1,
        "expected 1 loose-file error, got {}: {loose:#?}",
        loose.len()
    );
    assert_standard_admin(&loose, &errors);
}

#[test]
fn loose_file_in_all_admin_containers() {
    let tmp = copy_fixture();
    for path in admin_container_paths() {
        write_file(tmp.path(), &format!("{path}/stray.ts"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        6,
        "expected 6 loose-file errors (1 per container), got {}: {loose:#?}",
        loose.len()
    );
    assert_standard_admin(&loose, &errors);
}

// ============================================================================
// GROUP C: Loose files across ALL apps (admin + portal outer)
// ============================================================================

#[test]
fn loose_files_in_all_outer_containers() {
    let tmp = copy_fixture();
    for path in all_outer_container_paths() {
        write_file(tmp.path(), &format!("{path}/stray.ts"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    // 6 containers * 2 apps = 12 loose-file errors
    assert_eq!(
        loose.len(),
        12,
        "expected 12 loose-file errors (6 per app * 2 apps), got {}: {loose:#?}",
        loose.len()
    );
    // Per-app attribution
    for app in TS_SERVICE_APPS {
        assert!(
            loose.iter().any(|e| e.title.contains(app)),
            "expected error for app '{app}', got: {loose:#?}"
        );
    }
    assert_standard(&loose, &errors);
}

// ============================================================================
// GROUP D: Loose files in portal inner hex
// ============================================================================

#[test]
fn loose_file_in_portal_payments_inner_hex() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        &format!("{PORTAL_PAYMENTS_BASE}/domain/stray.ts"),
        "// stray",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        1,
        "expected 1 loose-file error in payments inner hex, got {}: {loose:#?}",
        loose.len()
    );
    assert!(
        loose[0].title.contains("portal"),
        "expected error to mention 'portal', got: '{}'",
        loose[0].title
    );
    let file = loose[0].file.as_deref().unwrap_or("");
    assert!(
        file.contains("payments/modules/domain"),
        "expected file field to reference payments inner hex, got: '{file}'"
    );
}

#[test]
fn loose_file_in_portal_aichat_inner_hex() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        &format!("{PORTAL_AICHAT_BASE}/application/stray.ts"),
        "// stray",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        1,
        "expected 1 loose-file error in ai-chat inner hex, got {}: {loose:#?}",
        loose.len()
    );
    assert!(
        loose[0].title.contains("portal"),
        "expected portal in title"
    );
}

#[test]
fn loose_files_in_all_inner_hex_containers() {
    let tmp = copy_fixture();
    for path in portal_inner_hex_container_paths() {
        write_file(tmp.path(), &format!("{path}/stray.ts"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    // 6 containers * 2 inner hexes = 12 loose-file errors
    assert_eq!(
        loose.len(),
        12,
        "expected 12 loose-file errors from inner hexes, got {}: {loose:#?}",
        loose.len()
    );
    for err in &loose {
        assert!(
            err.title.contains("portal"),
            "expected portal in title, got: '{}'",
            err.title
        );
    }
}

// ============================================================================
// GROUP E: Various file types
// ============================================================================

#[test]
fn loose_tsx_file() {
    let tmp = copy_fixture();
    for path in admin_container_paths() {
        write_file(
            tmp.path(),
            &format!("{path}/Component.tsx"),
            "export default function() {}",
        );
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        6,
        "expected 6 errors for .tsx files, got {}: {loose:#?}",
        loose.len()
    );
    assert_standard_admin(&loose, &errors);
}

#[test]
fn loose_json_file() {
    let tmp = copy_fixture();
    for path in admin_container_paths() {
        write_file(tmp.path(), &format!("{path}/config.json"), "{}");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        6,
        "expected 6 errors for .json files, got {}: {loose:#?}",
        loose.len()
    );
    assert_standard_admin(&loose, &errors);
}

#[test]
fn loose_env_file() {
    let tmp = copy_fixture();
    for path in admin_container_paths() {
        write_file(tmp.path(), &format!("{path}/.env"), "SECRET=123");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        6,
        "expected 6 errors for .env files, got {}: {loose:#?}",
        loose.len()
    );
    assert_standard_admin(&loose, &errors);
}

#[test]
fn loose_gitignore_not_gitkeep() {
    let tmp = copy_fixture();
    for path in admin_container_paths() {
        write_file(tmp.path(), &format!("{path}/.gitignore"), "node_modules/");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        6,
        ".gitignore is NOT .gitkeep — expected 6 errors, got {}: {loose:#?}",
        loose.len()
    );
    for err in &loose {
        assert!(
            err.message.contains(".gitignore"),
            "expected .gitignore in message, got: '{}'",
            err.message
        );
    }
    assert_standard_admin(&loose, &errors);
}

#[test]
fn hidden_file_not_gitkeep() {
    let tmp = copy_fixture();
    for path in admin_container_paths() {
        write_file(tmp.path(), &format!("{path}/.hidden"), "secret");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        6,
        ".hidden is not .gitkeep — expected 6 errors, got {}: {loose:#?}",
        loose.len()
    );
    assert_standard_admin(&loose, &errors);
}

// ============================================================================
// GROUP F: .gitkeep behavior
// ============================================================================

#[test]
fn gitkeep_allowed_in_containers() {
    let tmp = copy_fixture();
    for path in admin_container_paths() {
        write_file(tmp.path(), &format!("{path}/.gitkeep"), "");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        0,
        "expected 0 loose-file errors when only .gitkeep, got {}: {loose:#?}",
        loose.len()
    );
}

#[test]
fn gitkeep_alongside_loose_file() {
    let tmp = copy_fixture();
    for path in admin_container_paths() {
        write_file(tmp.path(), &format!("{path}/.gitkeep"), "");
        write_file(tmp.path(), &format!("{path}/stray.ts"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        6,
        ".gitkeep allowed but stray.ts not — expected 6, got {}: {loose:#?}",
        loose.len()
    );
    for err in &loose {
        assert!(
            err.message.contains("stray.ts"),
            "expected stray.ts in message"
        );
        let bad_files_section = err
            .message
            .split("that don't belong: ")
            .nth(1)
            .and_then(|s| s.split(". Only").next())
            .unwrap_or("");
        assert!(
            !bad_files_section.contains(".gitkeep"),
            ".gitkeep should not be in bad files list, got: '{bad_files_section}'"
        );
    }
    assert_standard_admin(&loose, &errors);
}

#[test]
fn multiple_loose_files_single_error_per_dir() {
    let tmp = copy_fixture();
    for path in admin_container_paths() {
        write_file(tmp.path(), &format!("{path}/index.ts"), "// stray 1");
        write_file(tmp.path(), &format!("{path}/types.ts"), "// stray 2");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        6,
        "expected 1 error per dir (6 total), got {}: {loose:#?}",
        loose.len()
    );
    for err in &loose {
        assert!(
            err.message.contains("index.ts") && err.message.contains("types.ts"),
            "expected both files listed in message, got: '{}'",
            err.message
        );
    }
    assert_standard_admin(&loose, &errors);
}

// ============================================================================
// GROUP G: Loose files in structural dirs (modules/, adapters/, ports/)
// ============================================================================

#[test]
fn loose_file_in_modules_root() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/admin/src/modules/index.ts", "// stray");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        1,
        "expected 1 loose-file error in modules/ root, got {}: {loose:#?}",
        loose.len()
    );
    assert_standard_admin(&loose, &errors);
}

#[test]
fn loose_file_in_adapters_root() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/src/modules/adapters/index.ts",
        "// stray",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        1,
        "expected 1 loose-file error in adapters/ root, got {}: {loose:#?}",
        loose.len()
    );
    assert_standard_admin(&loose, &errors);
}

#[test]
fn loose_file_in_ports_root() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/src/modules/ports/index.ts",
        "// stray",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        1,
        "expected 1 loose-file error in ports/ root, got {}: {loose:#?}",
        loose.len()
    );
    assert_standard_admin(&loose, &errors);
}

// ============================================================================
// GROUP H: Cross-cutting — all dir types
// ============================================================================

#[test]
fn loose_files_across_all_dir_types() {
    let tmp = copy_fixture();
    // modules/ root
    write_file(tmp.path(), "apps/admin/src/modules/stray.ts", "// stray");
    // structural dirs (adapters/, ports/)
    write_file(
        tmp.path(),
        "apps/admin/src/modules/adapters/stray.ts",
        "// stray",
    );
    write_file(
        tmp.path(),
        "apps/admin/src/modules/ports/stray.ts",
        "// stray",
    );
    // all 6 containers
    for path in admin_container_paths() {
        write_file(tmp.path(), &format!("{path}/stray.ts"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    // 1 (modules root) + 2 (structural) + 6 (containers) = 9
    assert_eq!(
        loose.len(),
        9,
        "expected 9 total loose-file errors (1 + 2 + 6), got {}: {loose:#?}",
        loose.len()
    );
    assert_standard_admin(&loose, &errors);
}

// ============================================================================
// GROUP I: Everywhere — all apps, all locations, including inner hex
// ============================================================================

#[test]
fn loose_files_everywhere() {
    let tmp = copy_fixture();
    // All outer containers in both apps
    for path in all_outer_container_paths() {
        write_file(tmp.path(), &format!("{path}/stray.ts"), "// stray");
    }
    // All inner hex containers in portal
    for path in portal_inner_hex_container_paths() {
        write_file(tmp.path(), &format!("{path}/stray.ts"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    // 12 outer (6 * 2 apps) + 12 inner hex (6 * 2 inner hexes) = 24
    assert_eq!(
        loose.len(),
        24,
        "expected 24 loose-file errors (12 outer + 12 inner), got {}: {loose:#?}",
        loose.len()
    );
    for app in TS_SERVICE_APPS {
        assert!(
            loose.iter().any(|e| e.title.contains(app)),
            "expected error for app '{app}', got: {loose:#?}"
        );
    }
    assert_file_field_modules(&loose);
    assert_no_rust_apps(&errors);
    assert_no_packages(&errors);
    assert_no_landing(&errors);
}

// ============================================================================
// GROUP J: Inner hex label_prefix correctness
// ============================================================================

#[test]
fn inner_hex_label_prefix_in_title() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        &format!("{PORTAL_PAYMENTS_BASE}/domain/stray.ts"),
        "// stray",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        1,
        "expected 1 error, got {}: {loose:#?}",
        loose.len()
    );
    // Title should reference the nested path through hex-in-hex recursion
    assert!(
        loose[0].title.contains("payments/modules/domain"),
        "expected nested hex label in title, got: '{}'",
        loose[0].title
    );
}

#[test]
fn inner_hex_loose_outer_clean() {
    let tmp = copy_fixture();
    // Only add loose files to inner hex containers — outer should be clean
    for path in portal_inner_hex_container_paths() {
        write_file(tmp.path(), &format!("{path}/stray.ts"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        12,
        "expected 12 inner hex loose errors, got {}: {loose:#?}",
        loose.len()
    );
    // All errors should reference inner hex paths
    for err in &loose {
        let file = err.file.as_deref().unwrap_or("");
        assert!(
            file.contains("payments/modules") || file.contains("ai-chat/modules"),
            "expected inner hex path in file field, got: '{file}'"
        );
    }
    // Admin should have 0 loose errors
    assert!(
        !loose.iter().any(|e| e.title.contains("admin")),
        "admin should have no loose-file errors, got: {loose:#?}"
    );
}

// ============================================================================
// GROUP K: Edge cases — symlinks
// ============================================================================

#[cfg(unix)]
#[test]
fn symlink_as_loose_file() {
    let tmp = copy_fixture();
    for path in admin_container_paths() {
        let dir = tmp.path().join(&path);
        let target = dir.join("..");
        let link = dir.join("stray");
        std::os::unix::fs::symlink(&target, &link).expect("create symlink");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    // DirEntry::file_type() without follow reports symlink → !is_dir() → flagged as loose
    assert_eq!(
        loose.len(),
        6,
        "symlinks should be flagged as loose files, got {}: {loose:#?}",
        loose.len()
    );
    assert_standard_admin(&loose, &errors);
}

// ============================================================================
// GROUP L: Cross-language isolation
// ============================================================================

#[test]
fn rust_apps_not_flagged() {
    let tmp = copy_fixture();
    for path in admin_container_paths() {
        write_file(tmp.path(), &format!("{path}/stray.ts"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_no_rust_apps(&errors);
}

#[test]
fn packages_not_flagged() {
    let tmp = copy_fixture();
    for path in admin_container_paths() {
        write_file(tmp.path(), &format!("{path}/stray.ts"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_no_packages(&errors);
}

#[test]
fn landing_not_flagged() {
    let tmp = copy_fixture();
    for path in admin_container_paths() {
        write_file(tmp.path(), &format!("{path}/stray.ts"), "// stray");
    }
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
    for path in admin_container_paths() {
        write_file(tmp.path(), &format!("{path}/stray.ts"), "// stray");
    }
    let results_1 = run_check(tmp.path());
    let errors_1 = arch_errors(&results_1);
    let loose_1 = loose_file_errors(&errors_1);
    assert!(
        loose_1.len() > 0,
        "precondition: expected loose errors from stray files"
    );
    let results_2 = run_check(tmp.path());
    let errors_2 = arch_errors(&results_2);
    let loose_2 = loose_file_errors(&errors_2);
    assert_eq!(
        loose_1.len(),
        loose_2.len(),
        "idempotent: run 1 = {}, run 2 = {}",
        loose_1.len(),
        loose_2.len()
    );
    for e1 in &loose_1 {
        assert!(
            loose_2.iter().any(|e2| e2.title == e1.title),
            "error '{}' from run 1 missing in run 2",
            e1.title
        );
    }
    assert_standard_admin(&loose_1, &errors_1);
}

// ============================================================================
// GROUP N: Per-app attribution
// ============================================================================

#[test]
fn per_app_attribution_admin() {
    let tmp = copy_fixture();
    for path in admin_container_paths() {
        write_file(tmp.path(), &format!("{path}/stray.ts"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        6,
        "precondition: expected 6 loose errors from admin containers"
    );
    for err in &loose {
        assert!(
            err.title.contains("admin"),
            "expected 'admin' in title, got: '{}'",
            err.title
        );
    }
}

#[test]
fn per_app_attribution_portal() {
    let tmp = copy_fixture();
    for suffix in CONTAINER_SUFFIXES {
        write_file(
            tmp.path(),
            &format!("apps/portal/src/modules/{suffix}/stray.ts"),
            "// stray",
        );
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    let portal_loose: Vec<_> = loose
        .iter()
        .filter(|e| e.title.contains("portal"))
        .collect();
    assert_eq!(
        portal_loose.len(),
        6,
        "expected 6 portal loose-file errors, got {}: {portal_loose:#?}",
        portal_loose.len()
    );
}

// ============================================================================
// GROUP O: Filesystem permissions
// ============================================================================

#[cfg(unix)]
#[test]
fn container_no_read_permission() {
    use std::os::unix::fs::PermissionsExt;
    let tmp = copy_fixture();
    // Make admin domain/ unreadable — check_container_not_empty can't list contents
    let domain = tmp.path().join("apps/admin/src/modules/domain");
    std::fs::set_permissions(&domain, std::fs::Permissions::from_mode(0o000)).expect("chmod");
    let results = run_check(tmp.path());
    std::fs::set_permissions(&domain, std::fs::Permissions::from_mode(0o755)).expect("chmod");
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    // Unreadable dir: list_dir returns empty → check_container_not_empty fires "empty container"
    // (not "loose files"), so no loose-file errors for this dir
    let domain_loose: Vec<_> = loose
        .iter()
        .filter(|e| e.file.as_deref().unwrap_or("").contains("domain"))
        .collect();
    assert_eq!(
        domain_loose.len(),
        0,
        "unreadable domain/ should not produce loose-file error (empty container instead), got: {domain_loose:#?}"
    );
}

// ============================================================================
// GROUP P: Double-fire fix
// ============================================================================

#[test]
fn container_with_only_loose_files_no_double_fire() {
    let tmp = copy_fixture();
    // Remove all subdirs from admin domain/ (make it "empty" with only loose files)
    super::helpers::remove_dir(tmp.path(), "apps/admin/src/modules/domain");
    std::fs::create_dir_all(tmp.path().join("apps/admin/src/modules/domain")).expect("mkdir");
    write_file(
        tmp.path(),
        "apps/admin/src/modules/domain/stray.ts",
        "// stray",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    // check_container_not_empty: no subdirs, no .gitkeep, has files → "empty container" error
    // Does NOT also fire "loose files" (double-fire fix)
    let loose = loose_file_errors(&errors);
    let domain_loose: Vec<_> = loose
        .iter()
        .filter(|e| e.file.as_deref().unwrap_or("").contains("domain"))
        .collect();
    assert_eq!(
        domain_loose.len(),
        0,
        "should NOT have loose-file error when container has only files (empty container handles it), got: {domain_loose:#?}"
    );
    // But there should be an "empty container" error that lists the file
    let empty: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("empty container") && e.title.contains("domain"))
        .collect();
    assert_eq!(
        empty.len(),
        1,
        "expected 1 empty-container error for domain/, got: {empty:#?}"
    );
    assert!(
        empty[0].message.contains("stray.ts"),
        "empty-container message should list 'stray.ts', got: '{}'",
        empty[0].message
    );
}

#[test]
fn empty_container_no_loose_files() {
    let tmp = copy_fixture();
    // Remove all subdirs from admin domain/ (truly empty)
    super::helpers::remove_dir(tmp.path(), "apps/admin/src/modules/domain");
    std::fs::create_dir_all(tmp.path().join("apps/admin/src/modules/domain")).expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    let domain_loose: Vec<_> = loose
        .iter()
        .filter(|e| e.file.as_deref().unwrap_or("").contains("domain"))
        .collect();
    // Empty container (no files, no subdirs, no .gitkeep) → "empty container" error
    // Not "loose files" (nothing to list)
    assert_eq!(
        domain_loose.len(),
        0,
        "truly empty container should produce 0 loose-file errors, got: {domain_loose:#?}"
    );
}

// ============================================================================
// GROUP Q: .gitkeep edge cases
// ============================================================================

#[test]
fn near_miss_gitkeep_names() {
    let tmp = copy_fixture();
    // These are NOT .gitkeep — should be flagged as loose
    write_file(tmp.path(), "apps/admin/src/modules/domain/.git_keep", "");
    write_file(tmp.path(), "apps/admin/src/modules/application/.gitkee", "");
    write_file(
        tmp.path(),
        "apps/admin/src/modules/adapters/inbound/.gitkeeps",
        "",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        3,
        "expected 3 loose-file errors for near-miss .gitkeep names, got {}: {loose:#?}",
        loose.len()
    );
    assert!(
        loose.iter().any(|e| e.message.contains(".git_keep")),
        "expected .git_keep in message"
    );
    assert!(
        loose.iter().any(|e| e.message.contains(".gitkee")),
        "expected .gitkee in message"
    );
    assert!(
        loose.iter().any(|e| e.message.contains(".gitkeeps")),
        "expected .gitkeeps in message"
    );
}

#[test]
fn gitkeep_wrong_case() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/admin/src/modules/domain/.GITKEEP", "");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    // On case-sensitive FS: .GITKEEP != .gitkeep → flagged as loose
    // On case-insensitive FS (macOS): .GITKEEP may resolve as .gitkeep → not flagged
    assert!(
        loose.len() == 0 || loose.len() == 1,
        "expected 0 (case-insensitive) or 1 (case-sensitive) loose-file errors, got {}: {loose:#?}",
        loose.len()
    );
}

#[test]
fn unicode_lookalike_gitkeep() {
    let tmp = copy_fixture();
    // Zero-width space: ".git\u{200B}keep" looks like ".gitkeep" but isn't
    write_file(
        tmp.path(),
        "apps/admin/src/modules/domain/.git\u{200B}keep",
        "",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        1,
        "unicode lookalike .gitkeep should be flagged, got {}: {loose:#?}",
        loose.len()
    );
}

#[test]
fn non_empty_gitkeep_allowed() {
    let tmp = copy_fixture();
    // .gitkeep with content — should still be recognized as .gitkeep
    write_file(
        tmp.path(),
        "apps/admin/src/modules/domain/.gitkeep",
        "this file has content",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    let domain_loose: Vec<_> = loose
        .iter()
        .filter(|e| e.file.as_deref().unwrap_or("").contains("domain"))
        .collect();
    assert_eq!(
        domain_loose.len(),
        0,
        ".gitkeep with content should still be allowed, got: {domain_loose:#?}"
    );
}

// ============================================================================
// GROUP R: Inner hex structural dirs
// ============================================================================

#[test]
fn loose_file_in_inner_hex_modules_root() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        &format!("{PORTAL_PAYMENTS_BASE}/stray.ts"),
        "// stray",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    let inner_loose: Vec<_> = loose
        .iter()
        .filter(|e| e.file.as_deref().unwrap_or("").contains("payments/modules"))
        .collect();
    assert_eq!(
        inner_loose.len(),
        1,
        "expected 1 loose error in inner hex modules/ root, got {}: {inner_loose:#?}",
        inner_loose.len()
    );
}

#[test]
fn loose_file_in_inner_hex_adapters_root() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        &format!("{PORTAL_PAYMENTS_BASE}/adapters/stray.ts"),
        "// stray",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    let inner_loose: Vec<_> = loose
        .iter()
        .filter(|e| {
            e.file
                .as_deref()
                .unwrap_or("")
                .contains("payments/modules/adapters")
        })
        .collect();
    assert_eq!(
        inner_loose.len(),
        1,
        "expected 1 loose error in inner hex adapters/ root, got {}: {inner_loose:#?}",
        inner_loose.len()
    );
}

// ============================================================================
// GROUP S: Additional edge cases
// ============================================================================

#[cfg(unix)]
#[test]
fn dangling_symlink_as_loose_file() {
    let tmp = copy_fixture();
    for path in admin_container_paths() {
        let dir = tmp.path().join(&path);
        let link = dir.join("dangling");
        std::os::unix::fs::symlink("/nonexistent/target", &link).expect("create dangling symlink");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    // Dangling symlinks: file_type() succeeds on macOS (returns symlink type), !is_dir() → flagged
    assert_eq!(
        loose.len(),
        6,
        "dangling symlinks should be flagged as loose files, got {}: {loose:#?}",
        loose.len()
    );
    assert_standard_admin(&loose, &errors);
}

#[test]
fn new_app_gets_checked() {
    let tmp = copy_fixture();
    // Create new app with valid hex arch + loose file
    write_file(
        tmp.path(),
        "apps/dashboard/package.json",
        r#"{"name": "dashboard"}"#,
    );
    for dir in CONTAINER_SUFFIXES {
        std::fs::create_dir_all(tmp.path().join(format!("apps/dashboard/src/modules/{dir}")))
            .expect("mkdir");
        write_file(
            tmp.path(),
            &format!("apps/dashboard/src/modules/{dir}/.gitkeep"),
            "",
        );
    }
    std::fs::create_dir_all(
        tmp.path()
            .join("apps/dashboard/src/modules/adapters/inbound"),
    )
    .expect("mkdir");
    std::fs::create_dir_all(
        tmp.path()
            .join("apps/dashboard/src/modules/adapters/outbound"),
    )
    .expect("mkdir");
    std::fs::create_dir_all(tmp.path().join("apps/dashboard/src/modules/ports/inbound"))
        .expect("mkdir");
    std::fs::create_dir_all(tmp.path().join("apps/dashboard/src/modules/ports/outbound"))
        .expect("mkdir");
    write_file(
        tmp.path(),
        "apps/dashboard/src/modules/adapters/inbound/.gitkeep",
        "",
    );
    write_file(
        tmp.path(),
        "apps/dashboard/src/modules/adapters/outbound/.gitkeep",
        "",
    );
    write_file(
        tmp.path(),
        "apps/dashboard/src/modules/ports/inbound/.gitkeep",
        "",
    );
    write_file(
        tmp.path(),
        "apps/dashboard/src/modules/ports/outbound/.gitkeep",
        "",
    );
    // Add loose file
    write_file(
        tmp.path(),
        "apps/dashboard/src/modules/domain/stray.ts",
        "// stray",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    let dashboard_loose: Vec<_> = loose
        .iter()
        .filter(|e| e.title.contains("dashboard"))
        .collect();
    assert_eq!(
        dashboard_loose.len(),
        1,
        "expected 1 loose-file error for new dashboard app, got {}: {dashboard_loose:#?}",
        dashboard_loose.len()
    );
}

#[test]
fn container_dir_absent_no_loose_error() {
    let tmp = copy_fixture();
    // Remove domain/ entirely — check_container_not_empty early-returns (metadata None)
    // No loose-file error should fire for a nonexistent dir
    super::helpers::remove_dir(tmp.path(), "apps/admin/src/modules/domain");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    let domain_loose: Vec<_> = loose
        .iter()
        .filter(|e| e.file.as_deref().unwrap_or("").contains("domain"))
        .collect();
    assert_eq!(
        domain_loose.len(),
        0,
        "absent container should produce 0 loose-file errors, got: {domain_loose:#?}"
    );
}

// ============================================================================
// GROUP T: RS-parity edge cases (round 2)
// ============================================================================

#[test]
fn file_coexists_with_same_named_subdir() {
    let tmp = copy_fixture();
    // admin domain/ has a "types" subdir. Add a file named "types.bak" alongside it.
    write_file(
        tmp.path(),
        "apps/admin/src/modules/domain/types.bak",
        "backup",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    let domain_loose: Vec<_> = loose
        .iter()
        .filter(|e| e.file.as_deref().unwrap_or("").contains("domain"))
        .copied()
        .collect();
    assert_eq!(
        domain_loose.len(),
        1,
        "expected 1 loose error for types.bak, got: {domain_loose:#?}"
    );
    assert!(
        domain_loose[0].message.contains("types.bak"),
        "expected types.bak in message"
    );
    // No missing-subdir error (types/ dir still exists)
    assert_standard_admin(&domain_loose, &errors);
}

#[test]
fn different_breakage_per_app() {
    let tmp = copy_fixture();
    // admin: .ts loose file in domain/
    write_file(
        tmp.path(),
        "apps/admin/src/modules/domain/stray.ts",
        "// ts stray",
    );
    // portal: .json loose file in application/
    write_file(
        tmp.path(),
        "apps/portal/src/modules/application/config.json",
        "{}",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    let admin_loose: Vec<_> = loose.iter().filter(|e| e.title.contains("admin")).collect();
    let portal_loose: Vec<_> = loose
        .iter()
        .filter(|e| e.title.contains("portal"))
        .collect();
    assert_eq!(
        admin_loose.len(),
        1,
        "expected 1 admin loose error: {admin_loose:#?}"
    );
    assert_eq!(
        portal_loose.len(),
        1,
        "expected 1 portal loose error: {portal_loose:#?}"
    );
    assert!(
        admin_loose[0].message.contains("stray.ts"),
        "admin message should list stray.ts"
    );
    assert!(
        portal_loose[0].message.contains("config.json"),
        "portal message should list config.json"
    );
    assert_no_rust_apps(&errors);
    assert_no_packages(&errors);
    assert_no_landing(&errors);
}

#[test]
fn maximally_complex_single_container() {
    let tmp = copy_fixture();
    // Admin domain/: .gitkeep (allowed) + multiple bad files of different types
    write_file(tmp.path(), "apps/admin/src/modules/domain/.gitkeep", "");
    write_file(
        tmp.path(),
        "apps/admin/src/modules/domain/mod.ts",
        "// stray",
    );
    write_file(
        tmp.path(),
        "apps/admin/src/modules/domain/.hidden",
        "secret",
    );
    write_file(tmp.path(), "apps/admin/src/modules/domain/.env", "SECRET=1");
    write_file(
        tmp.path(),
        "apps/admin/src/modules/domain/README.md",
        "# docs",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    let domain_loose: Vec<_> = loose
        .iter()
        .filter(|e| e.file.as_deref().unwrap_or("").contains("domain"))
        .copied()
        .collect();
    // 1 error listing 4 bad files (.gitkeep excluded)
    assert_eq!(
        domain_loose.len(),
        1,
        "expected 1 loose error listing all bad files: {domain_loose:#?}"
    );
    assert!(
        domain_loose[0].message.contains("mod.ts"),
        "expected mod.ts"
    );
    assert!(
        domain_loose[0].message.contains(".hidden"),
        "expected .hidden"
    );
    assert!(domain_loose[0].message.contains(".env"), "expected .env");
    assert!(
        domain_loose[0].message.contains("README.md"),
        "expected README.md"
    );
    // .gitkeep must NOT be in bad files list
    let bad_files_section = domain_loose[0]
        .message
        .split("that don't belong: ")
        .nth(1)
        .and_then(|s| s.split(". Only").next())
        .unwrap_or("");
    assert!(
        !bad_files_section.contains(".gitkeep"),
        ".gitkeep should not be in bad files list, got: '{bad_files_section}'"
    );
    assert_standard_admin(&domain_loose, &errors);
}

#[test]
fn gitkeep_plus_valid_subdir_plus_loose_all_containers() {
    let tmp = copy_fixture();
    // Add .gitkeep + stray file to ALL outer containers (both apps)
    for path in all_outer_container_paths() {
        write_file(tmp.path(), &format!("{path}/.gitkeep"), "");
        write_file(tmp.path(), &format!("{path}/stray.ts"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    // 12 loose errors (6 containers * 2 apps), .gitkeep excluded from each
    assert_eq!(
        loose.len(),
        12,
        "expected 12 loose errors (all containers, both apps), got {}: {loose:#?}",
        loose.len()
    );
    for err in &loose {
        assert!(
            err.message.contains("stray.ts"),
            "expected stray.ts in message"
        );
        let bad_files_section = err
            .message
            .split("that don't belong: ")
            .nth(1)
            .and_then(|s| s.split(". Only").next())
            .unwrap_or("");
        assert!(
            !bad_files_section.contains(".gitkeep"),
            ".gitkeep should not be in bad files list, got: '{bad_files_section}'"
        );
    }
    assert_standard(&loose, &errors);
}

#[test]
fn inner_hex_multiple_file_types() {
    let tmp = copy_fixture();
    // Add 3 different file types to each payments inner hex container
    for suffix in CONTAINER_SUFFIXES {
        let base = format!("{PORTAL_PAYMENTS_BASE}/{suffix}");
        write_file(tmp.path(), &format!("{base}/stray.ts"), "// ts");
        write_file(tmp.path(), &format!("{base}/.env"), "SECRET=1");
        write_file(tmp.path(), &format!("{base}/README.md"), "# docs");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    let payments_loose: Vec<_> = loose
        .iter()
        .filter(|e| e.file.as_deref().unwrap_or("").contains("payments/modules"))
        .collect();
    // 6 containers, 1 error each (listing 3 files)
    assert_eq!(
        payments_loose.len(),
        6,
        "expected 6 loose errors in payments inner hex: {payments_loose:#?}"
    );
    for err in &payments_loose {
        assert!(
            err.message.contains("stray.ts"),
            "expected stray.ts in message"
        );
        assert!(err.message.contains(".env"), "expected .env in message");
        assert!(
            err.message.contains("README.md"),
            "expected README.md in message"
        );
    }
    // Outer containers should be clean
    let admin_loose: Vec<_> = loose.iter().filter(|e| e.title.contains("admin")).collect();
    assert_eq!(
        admin_loose.len(),
        0,
        "admin should have 0 loose errors: {admin_loose:#?}"
    );
}
