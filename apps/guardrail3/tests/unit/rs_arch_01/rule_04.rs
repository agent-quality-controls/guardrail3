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
    // Title content: each error mentions "loose files" and its container label
    for err in &loose {
        assert!(
            err.title.contains("loose files") && err.title.contains("app"),
            "expected title with 'loose files' and 'app', got: '{}'",
            err.title
        );
    }
    assert_file_field(&errors);
    assert_per_app(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
    // Message content: offending filename appears
    for err in &loose {
        assert!(
            err.message.contains("mod.rs"),
            "expected 'mod.rs' in message, got: '{}'",
            err.message
        );
    }
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
    for err in &loose {
        assert!(
            err.title.contains("loose files") && err.title.contains("domain"),
            "expected title with 'loose files' and 'domain', got: '{}'",
            err.title
        );
    }
    assert_file_field(&errors);
    assert_per_app(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
    for err in &loose {
        assert!(
            err.message.contains("mod.rs"),
            "expected 'mod.rs' in message, got: '{}'",
            err.message
        );
    }
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
    for err in &loose {
        assert!(
            err.title.contains("loose files") && err.title.contains("adapters/inbound"),
            "expected title with 'loose files' and 'adapters/inbound', got: '{}'",
            err.title
        );
    }
    assert_file_field(&errors);
    assert_per_app(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
    for err in &loose {
        assert!(
            err.message.contains("mod.rs"),
            "expected 'mod.rs' in message, got: '{}'",
            err.message
        );
    }
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
    for err in &loose {
        assert!(
            err.title.contains("loose files") && err.title.contains("adapters/outbound"),
            "expected title with 'loose files' and 'adapters/outbound', got: '{}'",
            err.title
        );
    }
    assert_file_field(&errors);
    assert_per_app(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
    for err in &loose {
        assert!(
            err.message.contains("mod.rs"),
            "expected 'mod.rs' in message, got: '{}'",
            err.message
        );
    }
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
    for err in &loose {
        assert!(
            err.title.contains("loose files") && err.title.contains("ports/inbound"),
            "expected title with 'loose files' and 'ports/inbound', got: '{}'",
            err.title
        );
    }
    assert_file_field(&errors);
    assert_per_app(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
    for err in &loose {
        assert!(
            err.message.contains("mod.rs"),
            "expected 'mod.rs' in message, got: '{}'",
            err.message
        );
    }
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
    for err in &loose {
        assert!(
            err.title.contains("loose files") && err.title.contains("ports/outbound"),
            "expected title with 'loose files' and 'ports/outbound', got: '{}'",
            err.title
        );
    }
    assert_file_field(&errors);
    assert_per_app(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
    for err in &loose {
        assert!(
            err.message.contains("mod.rs"),
            "expected 'mod.rs' in message, got: '{}'",
            err.message
        );
    }
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
    // Title content
    for err in &loose {
        assert!(
            err.title.contains("loose files"),
            "expected 'loose files' in title, got: '{}'",
            err.title
        );
    }
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
    assert_file_field(&errors);
    // Message content
    for err in &loose {
        assert!(
            err.message.contains("mod.rs"),
            "expected 'mod.rs' in message, got: '{}'",
            err.message
        );
    }
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
    assert_file_field(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
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
    assert_file_field(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
    for err in &loose {
        assert!(
            err.message.contains("Cargo.toml"),
            "expected 'Cargo.toml' in message, got: '{}'",
            err.message
        );
    }
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
    assert_file_field(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
    for err in &loose {
        assert!(
            err.message.contains("README.md"),
            "expected 'README.md' in message, got: '{}'",
            err.message
        );
    }
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
    assert_file_field(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
    for err in &loose {
        assert!(
            err.message.contains(".env"),
            "expected '.env' in message, got: '{}'",
            err.message
        );
    }
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
    assert_file_field(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
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
    assert_file_field(&errors);
    assert_per_app(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
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
    // DirEntry::file_type() does NOT follow symlinks — so a symlink is !is_dir() -> flagged.
    assert_eq!(
        loose.len(),
        24,
        "symlinks should be flagged as loose files, got {}: {loose:#?}",
        loose.len()
    );
    assert_file_field(&errors);
    assert_per_app(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
    for err in &loose {
        assert!(
            err.message.contains("stray"),
            "expected 'stray' in message, got: '{}'",
            err.message
        );
    }
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

/// chmod 000 on a container directory. `list_dir` returns empty ->
/// check_loose_files returns 0 errors. Restore perms after test.
#[test]
#[cfg(unix)]
fn permission_denied_container() {
    use std::os::unix::fs::PermissionsExt;

    let tmp = copy_golden();
    let domain_dir = tmp.path().join("apps/devctl/crates/domain");

    // Remove all perms so list_dir returns empty
    let perms = std::fs::Permissions::from_mode(0o000);
    std::fs::set_permissions(&domain_dir, perms).expect("chmod 000"); // reason: simulate permission denied

    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let loose = loose_file_errors(&errors);

    // No loose-file errors for devctl/crates/domain because list_dir is empty
    let devctl_domain_loose: Vec<_> = loose
        .iter()
        .filter(|e| {
            e.title.contains("devctl")
                && e.file
                    .as_deref()
                    .unwrap_or("")
                    .contains("devctl/crates/domain")
        })
        .collect();
    assert_eq!(
        devctl_domain_loose.len(),
        0,
        "permission-denied container should produce 0 loose-file errors, got {}: {devctl_domain_loose:#?}",
        devctl_domain_loose.len()
    );

    // Restore permissions so temp dir cleanup works
    let restore = std::fs::Permissions::from_mode(0o755);
    std::fs::set_permissions(&domain_dir, restore).expect("restore perms"); // reason: allow temp dir cleanup
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
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
    // Message content
    for err in &loose {
        assert!(
            err.message.contains("stray.rs"),
            "expected 'stray.rs' in message, got: '{}'",
            err.message
        );
    }
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

    // Compare sorted titles, not just counts
    let mut titles_1: Vec<&str> = loose_1.iter().map(|e| e.title.as_str()).collect();
    let mut titles_2: Vec<&str> = loose_2.iter().map(|e| e.title.as_str()).collect();
    titles_1.sort();
    titles_2.sort();
    assert_eq!(
        titles_1, titles_2,
        "idempotent check failed: titles differ between runs"
    );

    assert_no_ts_apps(&errors_1);
    assert_no_packages(&errors_1);
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
    assert_file_field(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
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
    assert_file_field(&errors);
    // Message content
    for err in &scheduler_loose {
        assert!(
            err.message.contains("stray.rs"),
            "expected 'stray.rs' in message, got: '{}'",
            err.message
        );
    }
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

// ============================================================================
// GROUP F: Parity tests
// ============================================================================

/// A file named "core.bak" coexists with the real "core/" subdir.
/// Expected: 24 loose file errors (the .bak file), 0 missing crate errors.
#[test]
fn file_coexists_with_same_named_crate_subdir() {
    let tmp = copy_golden();
    for path in all_container_paths() {
        write_file(tmp.path(), &format!("{path}/core.bak"), "backup");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        24,
        "expected 24 loose-file errors for core.bak, got {}: {loose:#?}",
        loose.len()
    );
    // No "missing" errors from this change
    let missing: Vec<_> = errors.iter().filter(|e| e.title.contains("missing")).collect();
    assert_eq!(
        missing.len(),
        0,
        "expected 0 missing-crate errors, got {}: {missing:#?}",
        missing.len()
    );
    assert_file_field(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
    for err in &loose {
        assert!(
            err.message.contains("core.bak"),
            "expected 'core.bak' in message, got: '{}'",
            err.message
        );
    }
}

/// A file named ".gitke\u{200B}ep" (with zero-width space) should be flagged
/// as a loose file — it is NOT the same as ".gitkeep".
#[test]
fn unicode_lookalike_gitkeep() {
    let tmp = copy_golden();
    for path in all_container_paths() {
        write_file(
            tmp.path(),
            &format!("{path}/.gitke\u{200B}ep"),
            "",
        );
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        24,
        "unicode lookalike .gitkeep should be flagged as loose, got {}: {loose:#?}",
        loose.len()
    );
    assert_file_field(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

/// Near-miss gitkeep names: ".git_keep", ".gitkee" should be flagged as loose.
/// Note: ".gitKeep" is excluded because on case-insensitive FS (macOS default)
/// it collides with ".gitkeep" and the FS treats them as the same file.
#[test]
fn near_miss_gitkeep_names() {
    let tmp = copy_golden();
    // Only names that are unambiguously distinct from ".gitkeep" on ALL filesystems
    let near_misses = [".git_keep", ".gitkee"];
    for path in all_container_paths() {
        for name in &near_misses {
            write_file(tmp.path(), &format!("{path}/{name}"), "");
        }
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let loose = loose_file_errors(&errors);
    // One error per container (both near-miss files grouped into a single error)
    assert_eq!(
        loose.len(),
        24,
        "near-miss gitkeep names should be flagged, got {}: {loose:#?}",
        loose.len()
    );
    assert_file_field(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
    // Each near-miss name appears in the message
    for err in &loose {
        for name in &near_misses {
            assert!(
                err.message.contains(name),
                "expected '{name}' in message, got: '{}'",
                err.message
            );
        }
    }
}

/// Remove all subdirs from devctl/crates/app/ (leaving it empty or with only a
/// loose file). Both check_05 "empty container" AND check_loose_files fire.
#[test]
fn container_with_only_loose_files_double_error() {
    let tmp = copy_golden();
    // Remove the "core" subdir from devctl/crates/app/
    super::helpers::remove_dir(tmp.path(), "apps/devctl/crates/app/core");
    // Ensure app/ dir still exists
    std::fs::create_dir_all(tmp.path().join("apps/devctl/crates/app"))
        .expect("recreate app dir"); // reason: ensure it exists after removing core
    // Add a loose file (no subdirs, no .gitkeep)
    write_file(tmp.path(), "apps/devctl/crates/app/mod.rs", "// stray");

    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);

    // check_05 fires: "empty container" (no subdirs, no .gitkeep, but has files)
    let empty: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("empty container") && e.title.contains("devctl"))
        .collect();
    assert_eq!(
        empty.len(),
        1,
        "expected 1 empty-container error for devctl app/, got {}: {empty:#?}",
        empty.len()
    );

    // check_loose_files also fires: "loose files"
    let loose: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("loose files") && e.title.contains("devctl") && e.title.contains("app"))
        .collect();
    assert_eq!(
        loose.len(),
        1,
        "expected 1 loose-file error for devctl app/, got {}: {loose:#?}",
        loose.len()
    );

    // Document: both fire for the same directory
    assert!(
        empty.len() + loose.len() >= 2,
        "both empty-container and loose-file errors should fire for a container with \
         only loose files (no subdirs, no .gitkeep). empty={}, loose={}",
        empty.len(),
        loose.len()
    );
}

// ============================================================================
// GROUP G: Scenario tests
// ============================================================================

/// ".GITKEEP" (wrong case) should be flagged as loose on case-sensitive FS.
/// On case-insensitive FS (macOS default), the file may merge with existing
/// .gitkeep. We test the behavior by checking: if .GITKEEP is a distinct
/// file from .gitkeep, it must be flagged.
#[test]
fn gitkeep_wrong_case() {
    let tmp = copy_golden();
    for path in all_container_paths() {
        write_file(tmp.path(), &format!("{path}/.GITKEEP"), "");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let loose = loose_file_errors(&errors);

    // On case-sensitive FS: .GITKEEP != .gitkeep -> flagged (24 errors)
    // On case-insensitive FS (macOS HFS+/APFS): .GITKEEP overwrites .gitkeep
    //   -> if the container had .gitkeep it's now gone (replaced), and we wrote
    //      ".GITKEEP" which the filesystem may store as ".gitkeep" -> 0 errors
    //   -> if the container did not have .gitkeep, .GITKEEP is loose -> flagged
    //
    // We verify: either all 24 are flagged (case-sensitive) or the count matches
    // the case-insensitive behavior.
    let is_case_sensitive = {
        let test_dir = tmp.path().join("__case_test__");
        std::fs::create_dir_all(&test_dir).expect("create test dir"); // reason: probe case sensitivity
        write_file(tmp.path(), "__case_test__/lower", "");
        write_file(tmp.path(), "__case_test__/LOWER", "");
        let count = std::fs::read_dir(&test_dir)
            .expect("read test dir") // reason: count files
            .count();
        count == 2 // 2 files = case-sensitive
    };

    if is_case_sensitive {
        assert_eq!(
            loose.len(),
            24,
            "on case-sensitive FS, .GITKEEP should be flagged, got {}: {loose:#?}",
            loose.len()
        );
        assert_file_field(&errors);
        assert_per_app(&errors);
        assert_no_ts_apps(&errors);
        assert_no_packages(&errors);
    }
    // On case-insensitive FS we accept whatever the system produces —
    // the important thing is the test runs without panic.
}

/// A .gitkeep file with content should still be allowed — the check only
/// compares the filename, not the file contents.
#[test]
fn non_empty_gitkeep_allowed() {
    let tmp = copy_golden();
    for path in all_container_paths() {
        write_file(tmp.path(), &format!("{path}/.gitkeep"), "This file has content");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        0,
        ".gitkeep with content should still be allowed, got {}: {loose:#?}",
        loose.len()
    );
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}
