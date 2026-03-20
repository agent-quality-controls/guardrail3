use super::helpers::{arch_01_errors, copy_golden, run_check, write_file};
use guardrail3::domain::report::CheckResult;

const RUST_APPS: &[&str] = &["devctl", "backend", "worker"];
const INNER_HEX: &str = "apps/backend/crates/adapters/inbound/mcp/crates";

const CONTAINER_SUFFIXES: &[&str] = &[
    "app",
    "domain",
    "adapters/inbound",
    "adapters/outbound",
    "ports/inbound",
    "ports/outbound",
];

/// All 24 container locations: 3 outer apps x 6 + 1 inner hex x 6.
fn all_container_paths() -> Vec<String> {
    let mut paths = Vec::new();
    for app in RUST_APPS {
        for suffix in CONTAINER_SUFFIXES {
            paths.push(format!("apps/{app}/crates/{suffix}"));
        }
    }
    for suffix in CONTAINER_SUFFIXES {
        paths.push(format!("{INNER_HEX}/{suffix}"));
    }
    paths
}

/// Container paths for a single container type across all 4 hex locations.
fn container_paths_for(suffix: &str) -> Vec<String> {
    let mut paths = Vec::new();
    for app in RUST_APPS {
        paths.push(format!("apps/{app}/crates/{suffix}"));
    }
    paths.push(format!("{INNER_HEX}/{suffix}"));
    paths
}

/// Filter to only loose-file errors (title contains "loose files").
fn loose_file_errors<'a>(errors: &'a [&CheckResult]) -> Vec<&'a &'a CheckResult> {
    errors
        .iter()
        .filter(|e| e.title.contains("loose files"))
        .collect()
}

fn assert_per_app(errors: &[&CheckResult]) {
    for app in RUST_APPS {
        assert!(
            errors.iter().any(|e| e.title.contains(app)),
            "expected error for app `{app}`, got: {errors:#?}"
        );
    }
}

fn assert_inner_hex(errors: &[&CheckResult]) {
    assert!(
        errors
            .iter()
            .any(|e| e.file.as_deref().unwrap_or("").contains("mcp/crates")),
        "expected at least one error from inner hex (mcp/crates), got: {errors:#?}"
    );
}

fn assert_no_ts_apps(errors: &[&CheckResult]) {
    assert!(
        !errors
            .iter()
            .any(|e| e.title.contains("admin") || e.title.contains("landing")),
        "TS apps should not be flagged, got: {errors:#?}"
    );
}

fn assert_no_packages(errors: &[&CheckResult]) {
    assert!(
        !errors.iter().any(|e| {
            let t = &e.title;
            t.contains("shared-types") || t.contains("ui-kit")
        }),
        "packages should not be flagged, got: {errors:#?}"
    );
}

fn assert_file_field(errors: &[&CheckResult]) {
    for err in errors {
        assert!(
            err.file.is_some(),
            "expected file field to be set, got None for: {err:#?}"
        );
    }
}

// ============================================================================
// GROUP A: Loose files in each container type
// ============================================================================

#[test]
fn loose_file_in_app_containers() {
    let tmp = copy_golden();
    for path in container_paths_for("app") {
        write_file(tmp.path(), &format!("{path}/mod.rs"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        4,
        "expected 4 loose-file errors (3 outer + 1 inner), got {}: {loose:#?}",
        loose.len()
    );
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn loose_file_in_domain_containers() {
    let tmp = copy_golden();
    for path in container_paths_for("domain") {
        write_file(tmp.path(), &format!("{path}/mod.rs"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        4,
        "expected 4 loose-file errors, got {}: {loose:#?}",
        loose.len()
    );
}

#[test]
fn loose_file_in_adapters_inbound_containers() {
    let tmp = copy_golden();
    for path in container_paths_for("adapters/inbound") {
        write_file(tmp.path(), &format!("{path}/mod.rs"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        4,
        "expected 4 loose-file errors, got {}: {loose:#?}",
        loose.len()
    );
}

#[test]
fn loose_file_in_adapters_outbound_containers() {
    let tmp = copy_golden();
    for path in container_paths_for("adapters/outbound") {
        write_file(tmp.path(), &format!("{path}/mod.rs"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        4,
        "expected 4 loose-file errors, got {}: {loose:#?}",
        loose.len()
    );
}

#[test]
fn loose_file_in_ports_inbound_containers() {
    let tmp = copy_golden();
    for path in container_paths_for("ports/inbound") {
        write_file(tmp.path(), &format!("{path}/mod.rs"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        4,
        "expected 4 loose-file errors, got {}: {loose:#?}",
        loose.len()
    );
}

#[test]
fn loose_file_in_ports_outbound_containers() {
    let tmp = copy_golden();
    for path in container_paths_for("ports/outbound") {
        write_file(tmp.path(), &format!("{path}/mod.rs"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        4,
        "expected 4 loose-file errors, got {}: {loose:#?}",
        loose.len()
    );
}

#[test]
fn loose_file_in_all_containers() {
    let tmp = copy_golden();
    for path in all_container_paths() {
        write_file(tmp.path(), &format!("{path}/mod.rs"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        24,
        "expected 24 loose-file errors (3 apps x 6 + 1 inner hex x 6), got {}: {loose:#?}",
        loose.len()
    );
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
    assert_file_field(&errors);
}

// ============================================================================
// GROUP B: Various file types
// ============================================================================

#[test]
fn loose_rs_file() {
    let tmp = copy_golden();
    for path in all_container_paths() {
        write_file(tmp.path(), &format!("{path}/stray.rs"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        24,
        "expected 24 errors for .rs files, got {}: {loose:#?}",
        loose.len()
    );
    for err in &loose {
        assert!(
            err.message.contains("stray.rs"),
            "expected message to mention stray.rs, got: '{}'",
            err.message
        );
    }
}

#[test]
fn loose_toml_file() {
    let tmp = copy_golden();
    for path in all_container_paths() {
        write_file(tmp.path(), &format!("{path}/Cargo.toml"), "[package]");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        24,
        "expected 24 errors for Cargo.toml files, got {}: {loose:#?}",
        loose.len()
    );
}

#[test]
fn loose_markdown_file() {
    let tmp = copy_golden();
    for path in all_container_paths() {
        write_file(tmp.path(), &format!("{path}/README.md"), "# Readme");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        24,
        "expected 24 errors for README.md files, got {}: {loose:#?}",
        loose.len()
    );
}

#[test]
fn loose_env_file() {
    let tmp = copy_golden();
    for path in all_container_paths() {
        write_file(tmp.path(), &format!("{path}/.env"), "SECRET=123");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        24,
        "expected 24 errors for .env files, got {}: {loose:#?}",
        loose.len()
    );
}

#[test]
fn loose_gitignore_not_gitkeep() {
    let tmp = copy_golden();
    for path in all_container_paths() {
        write_file(tmp.path(), &format!("{path}/.gitignore"), "target/");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        24,
        ".gitignore is NOT .gitkeep — expected 24 errors, got {}: {loose:#?}",
        loose.len()
    );
    for err in &loose {
        assert!(
            err.message.contains(".gitignore"),
            "expected message to mention .gitignore, got: '{}'",
            err.message
        );
    }
}

// ============================================================================
// GROUP C: .gitkeep behavior
// ============================================================================

#[test]
fn gitkeep_allowed_in_containers() {
    let tmp = copy_golden();
    for path in all_container_paths() {
        write_file(tmp.path(), &format!("{path}/.gitkeep"), "");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        errors.is_empty(),
        "expected 0 errors when only .gitkeep is present, got {}: {errors:#?}",
        errors.len()
    );
}

#[test]
fn gitkeep_alongside_loose_file() {
    let tmp = copy_golden();
    for path in all_container_paths() {
        write_file(tmp.path(), &format!("{path}/.gitkeep"), "");
        write_file(tmp.path(), &format!("{path}/mod.rs"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        24,
        ".gitkeep is allowed but mod.rs is not — expected 24 errors, got {}: {loose:#?}",
        loose.len()
    );
    for err in &loose {
        assert!(
            err.message.contains("mod.rs"),
            "expected mod.rs in message, got: '{}'",
            err.message
        );
        // The message template contains the string ".gitkeep" in its static text
        // ("Only `.gitkeep` is allowed..."), so we check that `.gitkeep` does not
        // appear in the bad-files list portion: "that don't belong: {files}."
        let bad_files_section = err
            .message
            .split("that don't belong: ")
            .nth(1)
            .and_then(|s| s.split(". Only").next())
            .unwrap_or("");
        assert!(
            !bad_files_section.contains(".gitkeep"),
            ".gitkeep should NOT appear in bad files list, got: '{bad_files_section}'"
        );
    }
}

#[test]
fn multiple_loose_files_single_error_per_dir() {
    let tmp = copy_golden();
    for path in all_container_paths() {
        write_file(tmp.path(), &format!("{path}/mod.rs"), "// stray 1");
        write_file(tmp.path(), &format!("{path}/lib.rs"), "// stray 2");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        24,
        "expected 1 error per dir (24 total), not 1 per file, got {}: {loose:#?}",
        loose.len()
    );
    for err in &loose {
        assert!(
            err.message.contains("mod.rs") && err.message.contains("lib.rs"),
            "expected both files listed in message, got: '{}'",
            err.message
        );
    }
}

// ============================================================================
// GROUP D: Edge cases
// ============================================================================

#[test]
fn symlink_as_loose_file() {
    let tmp = copy_golden();
    for path in all_container_paths() {
        let dir = tmp.path().join(&path);
        std::fs::create_dir_all(&dir).expect("create dir"); // reason: ensure dir exists
        let target = dir.join(".."); // valid target
        let link = dir.join("stray");
        #[cfg(unix)]
        std::os::unix::fs::symlink(&target, &link).expect("create symlink"); // reason: test symlink detection
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let loose = loose_file_errors(&errors);
    // Symlinks pointing to dirs have is_dir() = true via follow, so they may not be flagged.
    // Symlinks pointing to files or non-dirs would be flagged.
    // The ".." target is a directory, but symlink file_type() without follow reports symlink.
    // DirEntry::file_type() does NOT follow symlinks — so a symlink is !is_dir() → flagged.
    assert_eq!(
        loose.len(),
        24,
        "symlinks should be flagged as loose files, got {}: {loose:#?}",
        loose.len()
    );
}

#[test]
fn dangling_symlink_as_loose_file() {
    let tmp = copy_golden();
    for path in all_container_paths() {
        let dir = tmp.path().join(&path);
        std::fs::create_dir_all(&dir).expect("create dir"); // reason: ensure dir exists
        let link = dir.join("dangling");
        #[cfg(unix)]
        std::os::unix::fs::symlink("/nonexistent/target", &link).expect("create dangling symlink"); // reason: test dangling symlink detection
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let loose = loose_file_errors(&errors);
    // On macOS, DirEntry::file_type() succeeds for dangling symlinks (returns symlink type).
    // Since a symlink is !is_dir(), it gets flagged as a loose file.
    assert_eq!(
        loose.len(),
        24,
        "dangling symlinks should be flagged as loose files, got {}: {loose:#?}",
        loose.len()
    );
}

#[test]
fn hidden_file_not_gitkeep() {
    let tmp = copy_golden();
    for path in all_container_paths() {
        write_file(tmp.path(), &format!("{path}/.hidden"), "secret");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        24,
        ".hidden is not .gitkeep — expected 24 errors, got {}: {loose:#?}",
        loose.len()
    );
}

#[test]
fn empty_container_no_loose_files() {
    let tmp = copy_golden();
    // Remove all contents from devctl ports/inbound (which has only .gitkeep)
    // This makes it empty — check_05 will fire "empty container" but check_loose_files
    // should produce 0 loose-file errors (no files to flag).
    std::fs::remove_file(tmp.path().join("apps/devctl/crates/ports/inbound/.gitkeep"))
        .expect("remove .gitkeep"); // reason: make container empty
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        0,
        "empty container should have 0 loose-file errors, got {}: {loose:#?}",
        loose.len()
    );
    // But there should be an "empty container" error
    assert!(
        errors.iter().any(|e| e.title.contains("empty container")),
        "expected an 'empty container' error, got: {errors:#?}"
    );
}

// ============================================================================
// GROUP E: Cross-cutting
// ============================================================================

#[test]
fn loose_files_across_all_dir_types() {
    let tmp = copy_golden();

    // Rule 2 locations: crates/ root (4 locations: 3 outer + 1 inner hex)
    let crates_roots = [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        INNER_HEX,
    ];
    for root in &crates_roots {
        write_file(tmp.path(), &format!("{root}/stray.rs"), "// stray");
    }

    // Rule 3 locations: adapters/ and ports/ structural dirs (8 locations)
    let structural_dirs = [
        "apps/devctl/crates/adapters",
        "apps/backend/crates/adapters",
        "apps/worker/crates/adapters",
        &format!("{INNER_HEX}/adapters"),
        "apps/devctl/crates/ports",
        "apps/backend/crates/ports",
        "apps/worker/crates/ports",
        &format!("{INNER_HEX}/ports"),
    ];
    for dir in &structural_dirs {
        write_file(tmp.path(), &format!("{dir}/stray.rs"), "// stray");
    }

    // Rule 4/5 locations: all 24 containers
    for path in all_container_paths() {
        write_file(tmp.path(), &format!("{path}/stray.rs"), "// stray");
    }

    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let loose = loose_file_errors(&errors);
    // 4 (crates root) + 8 (structural) + 24 (containers) = 36
    assert_eq!(
        loose.len(),
        36,
        "expected 36 total loose-file errors (4 + 8 + 24), got {}: {loose:#?}",
        loose.len()
    );
}

#[test]
fn ts_apps_not_checked() {
    let tmp = copy_golden();
    // Add loose files into TS app directories that mirror Rust hex structure
    write_file(
        tmp.path(),
        "apps/admin/src/modules/domain/stray.rs",
        "// stray",
    );
    write_file(
        tmp.path(),
        "apps/landing/src/types/stray.rs",
        "// stray",
    );
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        errors.is_empty(),
        "TS apps should not produce RS-ARCH-01 errors, got: {errors:#?}"
    );
}

#[test]
fn packages_not_checked() {
    let tmp = copy_golden();
    write_file(
        tmp.path(),
        "packages/shared-types/stray.rs",
        "// stray",
    );
    write_file(
        tmp.path(),
        "packages/ui-kit/stray.rs",
        "// stray",
    );
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        errors.is_empty(),
        "packages should not produce RS-ARCH-01 errors, got: {errors:#?}"
    );
}

#[test]
fn inner_hex_loose_file_outer_clean() {
    let tmp = copy_golden();
    // Only add loose files to inner hex containers
    for suffix in CONTAINER_SUFFIXES {
        write_file(
            tmp.path(),
            &format!("{INNER_HEX}/{suffix}/stray.rs"),
            "// stray",
        );
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        6,
        "expected 6 errors from inner hex only, got {}: {loose:#?}",
        loose.len()
    );
    // All errors should reference mcp/crates in their file field
    for err in &loose {
        assert!(
            err.file.as_deref().unwrap_or("").contains("mcp/crates"),
            "expected inner hex path in file field, got: '{}'",
            err.file.as_deref().unwrap_or("")
        );
    }
    // No errors for devctl or worker
    assert!(
        !loose.iter().any(|e| e.title.contains("devctl")),
        "devctl should have no errors"
    );
    assert!(
        !loose.iter().any(|e| e.title.contains("worker")),
        "worker should have no errors"
    );
}

#[test]
fn inner_hex_label_prefix_in_title() {
    let tmp = copy_golden();
    write_file(
        tmp.path(),
        &format!("{INNER_HEX}/domain/stray.rs"),
        "// stray",
    );
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(loose.len(), 1, "expected 1 error, got {}: {loose:#?}", loose.len());
    // The title should reference "backend" as the service name
    assert!(
        loose[0].title.contains("backend"),
        "expected 'backend' in title, got: '{}'",
        loose[0].title
    );
    // The label should include the nested path through the hex-in-hex recursion
    // Label is built as: crates/adapters/inbound/mcp/crates/domain
    assert!(
        loose[0].title.contains("crates/adapters/inbound/mcp/crates/domain"),
        "expected nested hex label in title, got: '{}'",
        loose[0].title
    );
}

#[test]
fn idempotent_results() {
    let tmp = copy_golden();
    for path in all_container_paths() {
        write_file(tmp.path(), &format!("{path}/mod.rs"), "// stray");
    }
    let results_1 = run_check(tmp.path());
    let errors_1 = arch_01_errors(&results_1);
    let loose_1 = loose_file_errors(&errors_1);

    let results_2 = run_check(tmp.path());
    let errors_2 = arch_01_errors(&results_2);
    let loose_2 = loose_file_errors(&errors_2);

    assert_eq!(
        loose_1.len(),
        loose_2.len(),
        "idempotent check failed: first run {} errors, second run {} errors",
        loose_1.len(),
        loose_2.len()
    );
}

#[test]
fn per_app_attribution() {
    let tmp = copy_golden();
    for path in all_container_paths() {
        write_file(tmp.path(), &format!("{path}/mod.rs"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let loose = loose_file_errors(&errors);

    // Each Rust app must appear in error titles
    for app in RUST_APPS {
        let app_errors: Vec<_> = loose.iter().filter(|e| e.title.contains(app)).collect();
        assert!(
            !app_errors.is_empty(),
            "expected at least one error for app `{app}`, got none. All: {loose:#?}"
        );
    }
    // Inner hex errors should be attributed to "backend"
    let backend_errors: Vec<_> = loose.iter().filter(|e| e.title.contains("backend")).collect();
    // backend has 6 outer + 6 inner = 12 errors
    assert_eq!(
        backend_errors.len(),
        12,
        "expected 12 errors for backend (6 outer + 6 inner hex), got {}: {backend_errors:#?}",
        backend_errors.len()
    );
}

#[test]
fn new_app_gets_checked() {
    let tmp = copy_golden();
    // Create a new Rust app with Cargo.toml and hex arch structure
    write_file(tmp.path(), "apps/scheduler/Cargo.toml", "[workspace]");
    for suffix in CONTAINER_SUFFIXES {
        let dir = format!("apps/scheduler/crates/{suffix}");
        // Add .gitkeep so the container is not empty, plus a loose file
        write_file(tmp.path(), &format!("{dir}/.gitkeep"), "");
        write_file(tmp.path(), &format!("{dir}/stray.rs"), "// stray");
    }
    // Also create the required structural dirs to avoid missing-dir errors
    write_file(
        tmp.path(),
        "apps/scheduler/crates/adapters/inbound/.gitkeep",
        "",
    );
    write_file(
        tmp.path(),
        "apps/scheduler/crates/adapters/outbound/.gitkeep",
        "",
    );
    write_file(
        tmp.path(),
        "apps/scheduler/crates/ports/inbound/.gitkeep",
        "",
    );
    write_file(
        tmp.path(),
        "apps/scheduler/crates/ports/outbound/.gitkeep",
        "",
    );

    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let loose = loose_file_errors(&errors);
    let scheduler_loose: Vec<_> = loose
        .iter()
        .filter(|e| e.title.contains("scheduler"))
        .collect();
    assert_eq!(
        scheduler_loose.len(),
        6,
        "expected 6 loose-file errors for new scheduler app, got {}: {scheduler_loose:#?}",
        scheduler_loose.len()
    );
}
