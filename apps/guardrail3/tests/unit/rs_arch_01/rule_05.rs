use super::helpers::{
    arch_errors, assert_file_field, assert_inner_hex, assert_no_packages, assert_no_ts_apps,
    assert_per_app, assert_single_error, copy_fixture, remove_dir, remove_file, run_check,
    write_file, INNER_HEX, RUST_APPS,
};
use guardrail3::domain::report::CheckResult;
use std::os::unix::fs::PermissionsExt;
#[allow(unused_imports)] // reason: symlink tests use this
use std::os::unix::fs::symlink;

const CONTAINER_SUFFIXES: &[&str] = &[
    "app",
    "domain",
    "adapters/inbound",
    "adapters/outbound",
    "ports/inbound",
    "ports/outbound",
];

/// Filter to only empty-container errors (title contains "empty container").
fn rule5_errors<'a>(errors: &'a [&'a CheckResult]) -> Vec<&'a CheckResult> {
    errors
        .iter()
        .filter(|e| e.title.contains("empty container"))
        .copied()
        .collect()
}

/// Container paths for a single container type across all 4 hex locations (3 outer + 1 inner).
fn container_paths_for(suffix: &str) -> Vec<String> {
    let mut paths = Vec::new();
    for app in RUST_APPS {
        paths.push(format!("apps/{app}/crates/{suffix}"));
    }
    paths.push(format!("{INNER_HEX}/{suffix}"));
    paths
}

/// Remove all contents (subdirs + files) of a container directory, leaving the dir itself.
fn empty_container(root: &std::path::Path, container_rel: &str) {
    let dir = root.join(container_rel);
    if !dir.exists() {
        return;
    }
    for entry in std::fs::read_dir(&dir).expect("read container dir") {
        let entry = entry.expect("dir entry");
        let ft = entry.file_type().expect("file type");
        if ft.is_dir() {
            std::fs::remove_dir_all(entry.path()).expect("remove subdir");
        } else {
            std::fs::remove_file(entry.path()).expect("remove file");
        }
    }
}

// ============================================================================
// GROUP A: Empty containers (no subdirs, no .gitkeep)
// ============================================================================

#[test]
fn empty_app_containers() {
    let tmp = copy_fixture();
    for path in container_paths_for("app") {
        empty_container(tmp.path(), &path);
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    assert_eq!(r5.len(), 4, "expected 4 empty-container errors (3 outer + 1 inner), got {}: {r5:#?}", r5.len());
    for err in &r5 {
        assert!(
            err.title.contains("empty container") && err.title.contains("/app"),
            "expected 'empty container' and '/app' in title, got: '{}'",
            err.title
        );
        assert!(
            err.message.contains("is empty"),
            "expected 'is empty' in message, got: '{}'",
            err.message
        );
    }
    assert_file_field(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn empty_domain_containers() {
    let tmp = copy_fixture();
    for path in container_paths_for("domain") {
        empty_container(tmp.path(), &path);
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    assert_eq!(r5.len(), 4, "expected 4 empty-container errors, got {}: {r5:#?}", r5.len());
    for err in &r5 {
        assert!(
            err.title.contains("empty container") && err.title.contains("/domain"),
            "expected 'empty container' and '/domain' in title, got: '{}'",
            err.title
        );
        assert!(
            err.message.contains("is empty"),
            "expected 'is empty' in message, got: '{}'",
            err.message
        );
    }
    assert_file_field(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn empty_adapters_inbound_containers() {
    // Emptying backend's adapters/inbound removes mcp/, which destroys the inner hex path.
    // So inner hex containers become nonexistent (metadata returns None, early return).
    // We can only get 3 outer errors here, not 4.
    let tmp = copy_fixture();
    for app in RUST_APPS {
        empty_container(tmp.path(), &format!("apps/{app}/crates/adapters/inbound"));
    }
    // Inner hex adapters/inbound still exists (its parent mcp was just destroyed by emptying
    // backend's adapters/inbound). So we don't get inner hex errors — the dir is gone.
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    assert_eq!(r5.len(), 3, "expected 3 outer empty-container errors (inner hex path destroyed), got {}: {r5:#?}", r5.len());
    for err in &r5 {
        assert!(
            err.title.contains("empty container") && err.title.contains("adapters/inbound"),
            "expected 'empty container' and 'adapters/inbound' in title, got: '{}'",
            err.title
        );
        assert!(
            err.message.contains("is empty"),
            "expected 'is empty' in message, got: '{}'",
            err.message
        );
    }
    assert_file_field(&errors);
    assert_per_app(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn empty_adapters_inbound_inner_hex_separately() {
    // Empty only inner hex adapters/inbound (without destroying outer backend's adapters/inbound).
    let tmp = copy_fixture();
    empty_container(tmp.path(), &format!("{INNER_HEX}/adapters/inbound"));
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    assert_eq!(r5.len(), 1, "expected 1 inner-hex empty-container error, got {}: {r5:#?}", r5.len());
    assert!(
        r5[0].title.contains("empty container") && r5[0].title.contains("adapters/inbound"),
        "expected 'empty container' and 'adapters/inbound' in title, got: '{}'",
        r5[0].title
    );
    assert!(
        r5[0].message.contains("is empty"),
        "expected 'is empty' in message, got: '{}'",
        r5[0].message
    );
    assert!(
        r5[0].file.as_deref().unwrap_or("").contains("mcp/crates"),
        "expected inner hex path in file field, got: {:?}",
        r5[0].file
    );
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn empty_adapters_outbound_containers() {
    let tmp = copy_fixture();
    for path in container_paths_for("adapters/outbound") {
        empty_container(tmp.path(), &path);
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    assert_eq!(r5.len(), 4, "expected 4 empty-container errors, got {}: {r5:#?}", r5.len());
    for err in &r5 {
        assert!(
            err.title.contains("empty container") && err.title.contains("adapters/outbound"),
            "expected 'empty container' and 'adapters/outbound' in title, got: '{}'",
            err.title
        );
        assert!(
            err.message.contains("is empty"),
            "expected 'is empty' in message, got: '{}'",
            err.message
        );
    }
    assert_file_field(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn empty_ports_inbound_containers() {
    let tmp = copy_fixture();
    // ports/inbound containers have .gitkeep in golden (devctl, worker, inner hex).
    // backend ports/inbound has api/ subdir. Remove everything.
    for path in container_paths_for("ports/inbound") {
        empty_container(tmp.path(), &path);
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    assert_eq!(r5.len(), 4, "expected 4 empty-container errors, got {}: {r5:#?}", r5.len());
    for err in &r5 {
        assert!(
            err.title.contains("empty container") && err.title.contains("ports/inbound"),
            "expected 'empty container' and 'ports/inbound' in title, got: '{}'",
            err.title
        );
        assert!(
            err.message.contains("is empty"),
            "expected 'is empty' in message, got: '{}'",
            err.message
        );
    }
    assert_file_field(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn empty_ports_outbound_containers() {
    let tmp = copy_fixture();
    for path in container_paths_for("ports/outbound") {
        empty_container(tmp.path(), &path);
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    assert_eq!(r5.len(), 4, "expected 4 empty-container errors, got {}: {r5:#?}", r5.len());
    for err in &r5 {
        assert!(
            err.title.contains("empty container") && err.title.contains("ports/outbound"),
            "expected 'empty container' and 'ports/outbound' in title, got: '{}'",
            err.title
        );
        assert!(
            err.message.contains("is empty"),
            "expected 'is empty' in message, got: '{}'",
            err.message
        );
    }
    assert_file_field(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

// ============================================================================
// GROUP B: .gitkeep prevents empty-container error
// ============================================================================

#[test]
fn gitkeep_already_present_baseline() {
    // Golden fixture has .gitkeep in several containers — should produce 0 errors.
    let tmp = copy_fixture();
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    assert!(
        r5.is_empty(),
        "golden fixture should have 0 empty-container errors, got: {r5:#?}"
    );
}

#[test]
fn gitkeep_prevents_empty() {
    // Remove all subdirs from all 24 containers but add .gitkeep → 0 empty-container errors.
    let tmp = copy_fixture();
    // First empty all outer containers and add .gitkeep.
    for app in RUST_APPS {
        for c in CONTAINER_SUFFIXES {
            let path = format!("apps/{app}/crates/{c}");
            empty_container(tmp.path(), &path);
            write_file(tmp.path(), &format!("{path}/.gitkeep"), "");
        }
    }
    // Emptying backend's adapters/inbound destroyed mcp path.
    // Recreate inner hex containers with .gitkeep.
    for c in CONTAINER_SUFFIXES {
        let path = format!("{INNER_HEX}/{c}");
        std::fs::create_dir_all(tmp.path().join(&path)).expect("recreate inner hex container");
        write_file(tmp.path(), &format!("{path}/.gitkeep"), "");
    }
    // Also need mcp to have Cargo.toml for it to be recognized as a leaf crate,
    // or crates/ dir for hex-in-hex. The inner hex requires a crates/ dir under mcp/.
    // Since we just recreated the crates dirs above, we need the parent mcp/crates/ path.
    // The mcp dir itself needs to exist as a subdir of backend's adapters/inbound/.
    // We added .gitkeep to backend's adapters/inbound, but mcp needs to also exist.
    // Actually, we emptied backend's adapters/inbound which removes mcp. But then we
    // created INNER_HEX/app etc which recreates the path.
    // mcp/ has a crates/ dir which makes it hex-in-hex. The check will find mcp/ as a
    // subdir of adapters/inbound/ and recurse into it. But we also added .gitkeep to
    // adapters/inbound/ — that's fine, .gitkeep prevents the empty-container error,
    // and mcp/ being a subdir means it's not empty anyway.
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    assert!(
        r5.is_empty(),
        "expected 0 empty-container errors when .gitkeep present, got {}: {r5:#?}",
        r5.len()
    );
}

#[test]
fn gitkeep_alongside_subdirs() {
    // Add .gitkeep to containers that already have subdirs — still valid, no errors.
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/crates/app/.gitkeep", "");
    write_file(tmp.path(), "apps/backend/crates/domain/.gitkeep", "");
    write_file(tmp.path(), "apps/worker/crates/adapters/outbound/.gitkeep", "");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    assert!(
        r5.is_empty(),
        "expected 0 errors when .gitkeep coexists with subdirs, got: {r5:#?}"
    );
}

// ============================================================================
// GROUP C: Container with files but no subdirs
// ============================================================================

#[test]
fn files_but_no_subdirs() {
    // Remove all subdirs, add mod.rs to one container per app.
    // Expected: empty-container error with "contains files (mod.rs) but no crate subdirectories".
    let tmp = copy_fixture();

    // devctl app/ — remove core/, add mod.rs
    remove_dir(tmp.path(), "apps/devctl/crates/app/core");
    std::fs::create_dir_all(tmp.path().join("apps/devctl/crates/app")).expect("recreate");
    write_file(tmp.path(), "apps/devctl/crates/app/mod.rs", "// stray");

    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);

    assert_eq!(r5.len(), 1, "expected exactly 1 empty-container error total, got {}: {r5:#?}", r5.len());

    // Should have at least 1 empty-container error for devctl app/
    let devctl_r5: Vec<_> = r5.iter().filter(|e| e.title.contains("devctl") && e.title.contains("/app")).collect();
    assert_eq!(devctl_r5.len(), 1, "expected 1 error for devctl app/, got {}: {devctl_r5:#?}", devctl_r5.len());
    assert!(
        devctl_r5[0].message.contains("contains files"),
        "expected 'contains files' in message, got: '{}'",
        devctl_r5[0].message
    );
    assert!(
        devctl_r5[0].message.contains("mod.rs"),
        "expected 'mod.rs' in message, got: '{}'",
        devctl_r5[0].message
    );
    assert!(
        devctl_r5[0].message.contains("but no crate subdirectories"),
        "expected 'but no crate subdirectories' in message, got: '{}'",
        devctl_r5[0].message
    );
    assert_file_field(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn multiple_files_but_no_subdirs() {
    // Two loose files, no subdirs, no .gitkeep — message lists both files.
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/devctl/crates/app/core");
    std::fs::create_dir_all(tmp.path().join("apps/devctl/crates/app")).expect("recreate");
    write_file(tmp.path(), "apps/devctl/crates/app/mod.rs", "// stray");
    write_file(tmp.path(), "apps/devctl/crates/app/lib.rs", "// stray");

    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    assert_eq!(r5.len(), 1, "expected exactly 1 empty-container error total, got {}: {r5:#?}", r5.len());

    let devctl_r5: Vec<_> = r5.iter().filter(|e| e.title.contains("devctl") && e.title.contains("/app")).collect();
    assert_eq!(devctl_r5.len(), 1, "expected 1 error, got {}: {devctl_r5:#?}", devctl_r5.len());
    // Both filenames should appear in the message
    assert!(
        devctl_r5[0].message.contains("mod.rs") && devctl_r5[0].message.contains("lib.rs"),
        "expected both 'mod.rs' and 'lib.rs' in message, got: '{}'",
        devctl_r5[0].message
    );
    assert!(
        devctl_r5[0].message.contains("contains files"),
        "expected 'contains files' in message, got: '{}'",
        devctl_r5[0].message
    );
    assert_file_field(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

// ============================================================================
// GROUP D: Edge cases
// ============================================================================

#[test]
fn container_dir_nonexistent() {
    // Remove entire container dir. Metadata returns None → early return → 0 rule5 errors.
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/devctl/crates/app");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    let devctl_app: Vec<_> = r5.iter().filter(|e| e.title.contains("devctl") && e.title.contains("/app")).collect();
    assert!(
        devctl_app.is_empty(),
        "nonexistent container should not fire empty-container error (other rules catch it), got: {devctl_app:#?}"
    );
}

#[test]
fn container_with_only_gitkeep_and_files() {
    // .gitkeep + mod.rs, no subdirs. has_gitkeep=true so empty-container check passes.
    // But check_loose_files fires for mod.rs.
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/devctl/crates/app/core");
    std::fs::create_dir_all(tmp.path().join("apps/devctl/crates/app")).expect("recreate");
    write_file(tmp.path(), "apps/devctl/crates/app/.gitkeep", "");
    write_file(tmp.path(), "apps/devctl/crates/app/mod.rs", "// stray");

    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    assert_eq!(r5.len(), 0, "expected 0 empty-container errors (.gitkeep suppresses), got {}: {r5:#?}", r5.len());
    let devctl_app_r5: Vec<_> = r5.iter().filter(|e| e.title.contains("devctl") && e.title.contains("/app")).collect();
    assert!(
        devctl_app_r5.is_empty(),
        ".gitkeep present → no empty-container error, got: {devctl_app_r5:#?}"
    );
    // But loose-file error should fire for mod.rs
    let loose: Vec<_> = errors.iter().filter(|e| e.title.contains("loose files") && e.title.contains("devctl") && e.title.contains("/app")).collect();
    assert_eq!(
        loose.len(),
        1,
        "expected 1 loose-file error for mod.rs, got {}: {loose:#?}",
        loose.len()
    );
}

#[test]
fn inner_hex_empty_outer_clean() {
    // Empty only inner hex containers. Assert outer apps have 0 empty-container errors.
    let tmp = copy_fixture();
    for c in CONTAINER_SUFFIXES {
        empty_container(tmp.path(), &format!("{INNER_HEX}/{c}"));
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    assert_eq!(r5.len(), 6, "expected 6 inner hex empty-container errors, got {}: {r5:#?}", r5.len());
    // All r5 errors should be from inner hex, none from outer apps
    for err in &r5 {
        assert!(
            err.file.as_deref().unwrap_or("").contains("mcp/crates"),
            "expected only inner hex errors, but got outer error: {err:#?}"
        );
        assert!(
            err.title.contains("empty container"),
            "expected 'empty container' in title, got: '{}'",
            err.title
        );
        assert!(
            err.message.contains("is empty"),
            "expected 'is empty' in message, got: '{}'",
            err.message
        );
    }
    // Outer apps should have no empty-container errors
    let outer_r5: Vec<_> = r5.iter().filter(|e| !e.file.as_deref().unwrap_or("").contains("mcp/crates")).collect();
    assert!(
        outer_r5.is_empty(),
        "expected 0 outer empty-container errors, got: {outer_r5:#?}"
    );
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn inner_hex_label_prefix_in_title() {
    // Verify inner hex errors have correct nested label (contains "mcp" in file path).
    let tmp = copy_fixture();
    // Empty inner hex ports/inbound (which had .gitkeep in golden)
    remove_file(
        tmp.path(),
        &format!("{INNER_HEX}/ports/inbound/.gitkeep"),
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    assert_eq!(r5.len(), 1, "expected 1 error, got {}: {r5:#?}", r5.len());
    // Title should reference backend service
    assert!(
        r5[0].title.contains("backend"),
        "expected 'backend' in title (inner hex is part of backend), got: '{}'",
        r5[0].title
    );
    // File field should contain the inner hex path
    assert!(
        r5[0].file.as_deref().unwrap_or("").contains("mcp/crates/ports/inbound"),
        "expected 'mcp/crates/ports/inbound' in file field, got: {:?}",
        r5[0].file
    );
    // Title should contain "empty container" and ports/inbound label
    assert!(
        r5[0].title.contains("empty container") && r5[0].title.contains("ports/inbound"),
        "expected 'empty container' and 'ports/inbound' in title, got: '{}'",
        r5[0].title
    );
    assert!(
        r5[0].message.contains("is empty"),
        "expected 'is empty' in message, got: '{}'",
        r5[0].message
    );
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn permission_denied_container() {
    // chmod 000 on a container. list_dir_names returns empty, has_gitkeep returns false.
    // dir_names.is_empty() && !has_gitkeep → error (false positive, but expected behavior).
    let tmp = copy_fixture();
    let container = tmp.path().join("apps/devctl/crates/app");
    // Save original perms, chmod 000, run check, restore perms
    {
        let mut perms = std::fs::metadata(&container).expect("metadata").permissions();
        perms.set_mode(0o000);
        std::fs::set_permissions(&container, perms).expect("chmod 000");
    }

    let results = run_check(tmp.path());

    // Restore perms before assertions (so cleanup works)
    {
        let mut perms = std::fs::metadata(&container).expect("metadata").permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&container, perms).expect("restore perms");
    }

    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    assert_eq!(r5.len(), 1, "expected exactly 1 empty-container error total, got {}: {r5:#?}", r5.len());
    let devctl_app: Vec<_> = r5.iter().filter(|e| e.title.contains("devctl") && e.title.contains("/app")).collect();
    assert_eq!(
        devctl_app.len(),
        1,
        "expected 1 empty-container error (false positive from unreadable dir), got {}: {devctl_app:#?}",
        devctl_app.len()
    );
    assert!(
        devctl_app[0].message.contains("is empty"),
        "expected 'is empty' in message, got: '{}'",
        devctl_app[0].message
    );
    assert_file_field(&errors);
}

// ============================================================================
// GROUP E: Cross-cutting
// ============================================================================

#[test]
fn ts_apps_not_checked() {
    // Break TS app module structure. 0 RS empty-container errors for TS apps.
    let tmp = copy_fixture();
    // Create a broken crates/ structure under admin (TS app) — should not be checked
    // because admin has no Cargo.toml.
    write_file(tmp.path(), "apps/admin/crates/app/.gitkeep", "");
    // Also break the golden fixture to trigger errors for actual Rust apps
    remove_file(tmp.path(), "apps/devctl/crates/ports/inbound/.gitkeep");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    assert_no_ts_apps(&r5.iter().map(|&e| e).collect::<Vec<_>>());
    // Verify devctl did fire
    assert!(
        r5.iter().any(|e| e.title.contains("devctl")),
        "expected devctl error, got: {r5:#?}"
    );
}

#[test]
fn packages_not_checked() {
    // Packages should not be checked for hex arch structure.
    // Trigger a real Rust error first so the list is non-empty.
    let tmp = copy_fixture();
    remove_file(tmp.path(), "apps/devctl/crates/ports/inbound/.gitkeep");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    assert!(!r5.is_empty(), "expected non-empty error list to meaningfully test packages absence");
    assert_no_packages(&r5.iter().map(|&e| e).collect::<Vec<_>>());
}

#[test]
fn new_app_gets_checked() {
    // Create a new Rust app "scheduler" with Cargo.toml and empty containers.
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/scheduler/Cargo.toml", "[package]\nname = \"scheduler\"");
    // Create crates/ with the 6 container dirs, all empty
    for c in CONTAINER_SUFFIXES {
        std::fs::create_dir_all(tmp.path().join(format!("apps/scheduler/crates/{c}"))).expect("create container");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    let scheduler_r5: Vec<_> = r5.iter().filter(|e| e.title.contains("scheduler")).collect();
    assert_eq!(
        scheduler_r5.len(),
        6,
        "expected 6 empty-container errors for new scheduler app, got {}: {scheduler_r5:#?}",
        scheduler_r5.len()
    );
    for err in &scheduler_r5 {
        assert!(
            err.title.contains("empty container"),
            "expected 'empty container' in title, got: '{}'",
            err.title
        );
        assert!(
            err.message.contains("is empty"),
            "expected 'is empty' in message, got: '{}'",
            err.message
        );
    }
    // Verify each container name appears in at least one title
    for c in CONTAINER_SUFFIXES {
        assert!(
            scheduler_r5.iter().any(|e| e.title.contains(c)),
            "expected container '{c}' in scheduler errors, got: {scheduler_r5:#?}"
        );
    }
    assert_file_field(&scheduler_r5.iter().map(|&&e| e).collect::<Vec<_>>());
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn idempotent_results() {
    // Same check twice on the same fixture → same results.
    let tmp = copy_fixture();
    remove_file(tmp.path(), "apps/devctl/crates/ports/inbound/.gitkeep");

    let results1 = run_check(tmp.path());
    let errors1 = arch_errors(&results1);
    let r5_1 = rule5_errors(&errors1);

    let results2 = run_check(tmp.path());
    let errors2 = arch_errors(&results2);
    let r5_2 = rule5_errors(&errors2);

    assert_eq!(r5_1.len(), r5_2.len(), "idempotent: count mismatch");
    for (a, b) in r5_1.iter().zip(r5_2.iter()) {
        assert_eq!(a.title, b.title, "idempotent: title mismatch");
        assert_eq!(a.message, b.message, "idempotent: message mismatch");
        assert_eq!(a.file, b.file, "idempotent: file mismatch");
    }
}

#[test]
fn different_breakage_per_app() {
    // devctl: truly empty app/. worker: files but no subdirs in domain/. backend: inner hex empty.
    let tmp = copy_fixture();

    // devctl: empty app/ container (remove core/)
    empty_container(tmp.path(), "apps/devctl/crates/app");

    // worker: add mod.rs to domain/ but remove subdirs
    empty_container(tmp.path(), "apps/worker/crates/domain");
    write_file(tmp.path(), "apps/worker/crates/domain/mod.rs", "// stray");

    // backend inner hex: empty ports/outbound (remove .gitkeep)
    remove_file(
        tmp.path(),
        &format!("{INNER_HEX}/ports/outbound/.gitkeep"),
    );

    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    assert_eq!(r5.len(), 3, "expected 3 total empty-container errors, got {}: {r5:#?}", r5.len());

    // devctl app/ error: "is empty"
    let devctl: Vec<_> = r5.iter().filter(|e| e.title.contains("devctl")).collect();
    assert_eq!(devctl.len(), 1, "expected 1 devctl error, got {}: {devctl:#?}", devctl.len());
    assert!(
        devctl[0].message.contains("is empty"),
        "devctl should say 'is empty', got: '{}'",
        devctl[0].message
    );

    // worker domain/ error: "contains files"
    let worker: Vec<_> = r5.iter().filter(|e| e.title.contains("worker")).collect();
    assert_eq!(worker.len(), 1, "expected 1 worker error, got {}: {worker:#?}", worker.len());
    assert!(
        worker[0].message.contains("contains files") && worker[0].message.contains("mod.rs"),
        "worker should say 'contains files' with 'mod.rs', got: '{}'",
        worker[0].message
    );

    // backend inner hex ports/outbound error: "is empty"
    let backend_inner: Vec<_> = r5.iter().filter(|e| {
        e.title.contains("backend") && e.file.as_deref().unwrap_or("").contains("mcp/crates")
    }).collect();
    assert_eq!(backend_inner.len(), 1, "expected 1 backend inner hex error, got {}: {backend_inner:#?}", backend_inner.len());
    assert!(
        backend_inner[0].message.contains("is empty"),
        "backend inner hex should say 'is empty', got: '{}'",
        backend_inner[0].message
    );
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

// ============================================================================
// GROUP F: Single-container single-app focused tests
// ============================================================================

#[test]
fn empty_ports_inbound_single_app() {
    // Remove .gitkeep from devctl ports/inbound → single error
    let tmp = copy_fixture();
    remove_file(tmp.path(), "apps/devctl/crates/ports/inbound/.gitkeep");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    assert_single_error(
        &r5.iter().map(|&e| e).collect::<Vec<_>>(),
        "empty container",
    );
    assert!(
        r5[0].title.contains("devctl"),
        "expected 'devctl' in title, got: '{}'",
        r5[0].title
    );
    assert!(
        r5[0].title.contains("ports/inbound"),
        "expected 'ports/inbound' in title, got: '{}'",
        r5[0].title
    );
    assert!(r5[0].file.is_some(), "expected file field set");
    assert!(
        r5[0].message.contains("is empty"),
        "expected 'is empty' in message, got: '{}'",
        r5[0].message
    );
}

#[test]
fn empty_in_one_app_valid_in_another() {
    // Empty devctl ports/inbound (remove .gitkeep). Worker ports/inbound still has .gitkeep.
    let tmp = copy_fixture();
    remove_file(tmp.path(), "apps/devctl/crates/ports/inbound/.gitkeep");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    assert_eq!(r5.len(), 1, "expected 1 error for devctl only, got {}: {r5:#?}", r5.len());
    assert!(
        r5[0].title.contains("devctl"),
        "expected error for devctl, got: '{}'",
        r5[0].title
    );
    // Ensure other apps NOT flagged
    assert!(
        !r5.iter().any(|e| e.title.contains("worker")),
        "worker should not be flagged"
    );
    assert!(
        !r5.iter().any(|e| e.title.contains("backend")),
        "backend should not be flagged"
    );
    assert!(
        r5[0].message.contains("is empty"),
        "expected 'is empty' in message, got: '{}'",
        r5[0].message
    );
    assert_file_field(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn inner_hex_empty_container_single() {
    // Remove .gitkeep from inner hex ports/inbound → single error
    let tmp = copy_fixture();
    remove_file(
        tmp.path(),
        &format!("{INNER_HEX}/ports/inbound/.gitkeep"),
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    assert_eq!(r5.len(), 1, "expected 1 error, got {}: {r5:#?}", r5.len());
    assert!(
        r5[0].title.contains("empty container"),
        "expected 'empty container' in title, got: '{}'",
        r5[0].title
    );
    assert!(
        r5[0].message.contains("is empty"),
        "expected 'is empty' in message, got: '{}'",
        r5[0].message
    );
    assert_file_field(&errors);
    assert_inner_hex(&r5.iter().map(|&e| e).collect::<Vec<_>>());
}

#[test]
fn multiple_empty_containers_same_app() {
    // Empty two containers in devctl
    let tmp = copy_fixture();
    remove_file(tmp.path(), "apps/devctl/crates/ports/inbound/.gitkeep");
    remove_dir(tmp.path(), "apps/devctl/crates/ports/outbound/traits");
    std::fs::create_dir_all(tmp.path().join("apps/devctl/crates/ports/outbound")).expect("recreate");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    assert_eq!(r5.len(), 2, "expected exactly 2 empty-container errors total, got {}: {r5:#?}", r5.len());

    let devctl_r5: Vec<_> = r5.iter().filter(|e| e.title.contains("devctl")).collect();
    assert_eq!(
        devctl_r5.len(),
        2,
        "expected 2 empty-container errors for devctl, got {}: {devctl_r5:#?}",
        devctl_r5.len()
    );
    // Verify both ports/inbound and ports/outbound appear in titles
    assert!(
        devctl_r5.iter().any(|e| e.title.contains("ports/inbound")),
        "expected 'ports/inbound' in one of the titles, got: {devctl_r5:#?}"
    );
    assert!(
        devctl_r5.iter().any(|e| e.title.contains("ports/outbound")),
        "expected 'ports/outbound' in one of the titles, got: {devctl_r5:#?}"
    );
    for err in &devctl_r5 {
        assert!(
            err.message.contains("is empty"),
            "expected 'is empty' in message, got: '{}'",
            err.message
        );
    }
    assert_file_field(&errors);
}

// ============================================================================
// GROUP G: Additional TS/packages cross-checks with breakage
// ============================================================================

#[test]
fn ts_apps_broken_zero_rs_errors() {
    // Break TS apps in various ways. Verify 0 RS-ARCH-01 empty-container errors from them.
    // Also trigger a real Rust error so the list is non-empty.
    let tmp = copy_fixture();
    // Add crates/ structure to TS apps (they lack Cargo.toml so won't be scanned)
    std::fs::create_dir_all(tmp.path().join("apps/admin/crates/app")).expect("create");
    std::fs::create_dir_all(tmp.path().join("apps/landing/crates/domain")).expect("create");
    // Trigger a real Rust error: remove .gitkeep from worker ports/inbound
    remove_file(tmp.path(), "apps/worker/crates/ports/inbound/.gitkeep");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    assert!(!r5.is_empty(), "expected non-empty error list to meaningfully test TS apps absence");
    assert!(
        !r5.iter().any(|e| e.title.contains("admin") || e.title.contains("landing")),
        "TS apps should not produce empty-container errors, got: {r5:#?}"
    );
}

// ============================================================================
// GROUP H: Unicode, near-miss, symlink, and edge-case content tests
// ============================================================================

#[test]
fn unicode_lookalike_gitkeep() {
    // Write ".gitke\u{200B}ep" (zero-width space) — has_gitkeep reads exact ".gitkeep" path, so
    // this unicode lookalike won't match. Container should fire "contains files" error.
    let tmp = copy_fixture();
    let container = "apps/devctl/crates/ports/inbound";
    remove_file(tmp.path(), &format!("{container}/.gitkeep"));
    write_file(tmp.path(), &format!("{container}/.gitke\u{200B}ep"), "");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    let devctl_pi: Vec<_> = r5
        .iter()
        .filter(|e| e.title.contains("devctl") && e.title.contains("ports/inbound"))
        .collect();
    assert_eq!(
        devctl_pi.len(),
        1,
        "expected 1 empty-container error for unicode lookalike .gitkeep, got {}: {devctl_pi:#?}",
        devctl_pi.len()
    );
    assert!(
        devctl_pi[0].message.contains("contains files"),
        "expected 'contains files' in message, got: '{}'",
        devctl_pi[0].message
    );
}

#[test]
fn near_miss_gitkeep_names() {
    // Write ".git_keep" and ".gitkee" — neither is ".gitkeep". Should fire "contains files".
    let tmp = copy_fixture();
    let container = "apps/devctl/crates/ports/inbound";
    remove_file(tmp.path(), &format!("{container}/.gitkeep"));
    write_file(tmp.path(), &format!("{container}/.git_keep"), "");
    write_file(tmp.path(), &format!("{container}/.gitkee"), "");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    let devctl_pi: Vec<_> = r5
        .iter()
        .filter(|e| e.title.contains("devctl") && e.title.contains("ports/inbound"))
        .collect();
    assert_eq!(
        devctl_pi.len(),
        1,
        "expected 1 empty-container error for near-miss .gitkeep, got {}: {devctl_pi:#?}",
        devctl_pi.len()
    );
    assert!(
        devctl_pi[0].message.contains("contains files"),
        "expected 'contains files' in message, got: '{}'",
        devctl_pi[0].message
    );
    assert!(
        devctl_pi[0].message.contains(".git_keep") && devctl_pi[0].message.contains(".gitkee"),
        "expected both '.git_keep' and '.gitkee' in message, got: '{}'",
        devctl_pi[0].message
    );
    assert!(
        devctl_pi[0].message.contains("but no crate subdirectories"),
        "expected 'but no crate subdirectories' in message, got: '{}'",
        devctl_pi[0].message
    );
}

#[test]
fn symlink_as_only_content() {
    // Remove all subdirs, create a symlink "link" pointing to a valid dir.
    // DirEntry::file_type returns symlink (not dir on macOS), so dir_names empty.
    // The symlink shows up as a file in list_file_names → "contains files (link)".
    let tmp = copy_fixture();
    let container = "apps/devctl/crates/ports/inbound";
    remove_file(tmp.path(), &format!("{container}/.gitkeep"));
    // Create a symlink to a valid directory
    let target = tmp.path().join("apps/devctl/crates/app");
    let link_path = tmp.path().join(format!("{container}/link"));
    symlink(&target, &link_path).expect("create symlink");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    let devctl_pi: Vec<_> = r5
        .iter()
        .filter(|e| e.title.contains("devctl") && e.title.contains("ports/inbound"))
        .collect();
    assert_eq!(
        devctl_pi.len(),
        1,
        "expected 1 empty-container error for symlink-only content, got {}: {devctl_pi:#?}",
        devctl_pi.len()
    );
    assert!(
        devctl_pi[0].message.contains("contains files") && devctl_pi[0].message.contains("link"),
        "expected 'contains files' with 'link' in message, got: '{}'",
        devctl_pi[0].message
    );
}

#[test]
fn dangling_symlink_as_only_content() {
    // Same as symlink_as_only_content but the symlink points to /nonexistent.
    // DirEntry::file_type() on macOS still returns Ok(symlink), so it still shows up.
    let tmp = copy_fixture();
    let container = "apps/devctl/crates/ports/inbound";
    remove_file(tmp.path(), &format!("{container}/.gitkeep"));
    let link_path = tmp.path().join(format!("{container}/dangling"));
    symlink("/nonexistent/path/that/does/not/exist", &link_path).expect("create dangling symlink");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    let devctl_pi: Vec<_> = r5
        .iter()
        .filter(|e| e.title.contains("devctl") && e.title.contains("ports/inbound"))
        .collect();
    assert_eq!(
        devctl_pi.len(),
        1,
        "expected 1 empty-container error for dangling symlink, got {}: {devctl_pi:#?}",
        devctl_pi.len()
    );
    assert!(
        devctl_pi[0].message.contains("contains files") && devctl_pi[0].message.contains("dangling"),
        "expected 'contains files' with 'dangling' in message, got: '{}'",
        devctl_pi[0].message
    );
}

#[test]
fn gitkeep_as_directory() {
    // mkdir .gitkeep (not touch) in emptied container.
    // list_dir_names returns [".gitkeep"]. dir_names is NOT empty → check_05 passes.
    // But check_06 then catches it (no Cargo.toml in .gitkeep dir).
    // Assert 0 rule5 errors for this container.
    let tmp = copy_fixture();
    let container = "apps/devctl/crates/ports/inbound";
    remove_file(tmp.path(), &format!("{container}/.gitkeep"));
    std::fs::create_dir_all(tmp.path().join(format!("{container}/.gitkeep"))).expect("mkdir .gitkeep");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    let devctl_pi: Vec<_> = r5
        .iter()
        .filter(|e| e.title.contains("devctl") && e.title.contains("ports/inbound"))
        .collect();
    assert!(
        devctl_pi.is_empty(),
        ".gitkeep as directory means dir_names is non-empty → 0 rule5 errors, got: {devctl_pi:#?}"
    );
}

#[test]
fn gitkeep_wrong_case_in_container() {
    // Write ".GITKEEP" as only content. On case-insensitive FS (macOS default):
    // has_gitkeep reads ".gitkeep" which resolves to ".GITKEEP" → true → no error.
    // On case-sensitive FS: has_gitkeep returns false → fires "contains files".
    let tmp = copy_fixture();
    let container = "apps/devctl/crates/ports/inbound";
    remove_file(tmp.path(), &format!("{container}/.gitkeep"));
    write_file(tmp.path(), &format!("{container}/.GITKEEP"), "");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    let devctl_pi: Vec<_> = r5
        .iter()
        .filter(|e| e.title.contains("devctl") && e.title.contains("ports/inbound"))
        .collect();
    // Detect FS case sensitivity: try reading the wrong-case file
    let case_insensitive = tmp
        .path()
        .join(format!("{container}/.gitkeep"))
        .exists();
    if case_insensitive {
        // macOS default: .GITKEEP is found as .gitkeep → no error
        assert!(
            devctl_pi.is_empty(),
            "case-insensitive FS: .GITKEEP found as .gitkeep → 0 errors, got: {devctl_pi:#?}"
        );
    } else {
        // Case-sensitive FS: .GITKEEP is NOT .gitkeep → fires error
        assert_eq!(
            devctl_pi.len(),
            1,
            "case-sensitive FS: .GITKEEP not recognized → 1 error, got {}: {devctl_pi:#?}",
            devctl_pi.len()
        );
        assert!(
            devctl_pi[0].message.contains("contains files"),
            "expected 'contains files' in message, got: '{}'",
            devctl_pi[0].message
        );
    }
}

#[test]
fn file_replacing_subdir() {
    // Remove core/ from devctl app/, create a file named "core" (not a directory).
    // dir_names empty (file not dir), has_gitkeep false. Fires "contains files (core)".
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/devctl/crates/app/core");
    std::fs::create_dir_all(tmp.path().join("apps/devctl/crates/app")).expect("recreate");
    write_file(tmp.path(), "apps/devctl/crates/app/core", "// I am a file, not a dir");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    let devctl_app: Vec<_> = r5
        .iter()
        .filter(|e| e.title.contains("devctl") && e.title.contains("/app"))
        .collect();
    assert_eq!(
        devctl_app.len(),
        1,
        "expected 1 empty-container error for file-replacing-subdir, got {}: {devctl_app:#?}",
        devctl_app.len()
    );
    assert!(
        devctl_app[0].message.contains("contains files") && devctl_app[0].message.contains("core"),
        "expected 'contains files' with 'core' in message, got: '{}'",
        devctl_app[0].message
    );
}

#[test]
fn hidden_file_as_sole_content() {
    // Write ".DS_Store" as only content. Fires "contains files (.DS_Store)".
    let tmp = copy_fixture();
    let container = "apps/devctl/crates/ports/inbound";
    remove_file(tmp.path(), &format!("{container}/.gitkeep"));
    write_file(tmp.path(), &format!("{container}/.DS_Store"), "");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    let devctl_pi: Vec<_> = r5
        .iter()
        .filter(|e| e.title.contains("devctl") && e.title.contains("ports/inbound"))
        .collect();
    assert_eq!(
        devctl_pi.len(),
        1,
        "expected 1 empty-container error for .DS_Store, got {}: {devctl_pi:#?}",
        devctl_pi.len()
    );
    assert!(
        devctl_pi[0].message.contains("contains files") && devctl_pi[0].message.contains(".DS_Store"),
        "expected 'contains files' with '.DS_Store' in message, got: '{}'",
        devctl_pi[0].message
    );
}

#[test]
fn non_empty_gitkeep_prevents_error() {
    // Empty container, add .gitkeep with content. has_gitkeep returns true. 0 rule5 errors.
    let tmp = copy_fixture();
    let container = "apps/devctl/crates/ports/inbound";
    remove_file(tmp.path(), &format!("{container}/.gitkeep"));
    write_file(tmp.path(), &format!("{container}/.gitkeep"), "This file has content");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    let devctl_pi: Vec<_> = r5
        .iter()
        .filter(|e| e.title.contains("devctl") && e.title.contains("ports/inbound"))
        .collect();
    assert!(
        devctl_pi.is_empty(),
        ".gitkeep with content still prevents error, got: {devctl_pi:#?}"
    );
}

#[test]
fn maximally_complex_empty_container() {
    // In one container: no subdirs, add unicode .gitkeep (not real), add dangling symlink,
    // add .DS_Store, add .git_keep near-miss. dir_names empty, has_gitkeep false,
    // files = [all the junk]. Fires "contains files (...) but no crate subdirectories".
    let tmp = copy_fixture();
    let container = "apps/devctl/crates/ports/inbound";
    remove_file(tmp.path(), &format!("{container}/.gitkeep"));
    // Unicode lookalike
    write_file(tmp.path(), &format!("{container}/.gitke\u{200B}ep"), "");
    // Near-miss
    write_file(tmp.path(), &format!("{container}/.git_keep"), "");
    // Hidden file
    write_file(tmp.path(), &format!("{container}/.DS_Store"), "");
    // Dangling symlink
    let link_path = tmp.path().join(format!("{container}/broken_link"));
    symlink("/nonexistent/target", &link_path).expect("create dangling symlink");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r5 = rule5_errors(&errors);
    let devctl_pi: Vec<_> = r5
        .iter()
        .filter(|e| e.title.contains("devctl") && e.title.contains("ports/inbound"))
        .collect();
    assert_eq!(
        devctl_pi.len(),
        1,
        "expected 1 empty-container error for maximally complex case, got {}: {devctl_pi:#?}",
        devctl_pi.len()
    );
    assert!(
        devctl_pi[0].message.contains("contains files"),
        "expected 'contains files' in message, got: '{}'",
        devctl_pi[0].message
    );
    assert!(
        devctl_pi[0].message.contains("but no crate subdirectories"),
        "expected 'but no crate subdirectories' in message, got: '{}'",
        devctl_pi[0].message
    );
    // Verify at least some of the junk filenames appear in the message
    assert!(
        devctl_pi[0].message.contains(".DS_Store"),
        "expected '.DS_Store' in message, got: '{}'",
        devctl_pi[0].message
    );
    assert!(
        devctl_pi[0].message.contains(".git_keep"),
        "expected '.git_keep' in message, got: '{}'",
        devctl_pi[0].message
    );
}
