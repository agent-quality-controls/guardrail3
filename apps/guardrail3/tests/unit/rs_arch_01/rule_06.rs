use super::helpers::{
    arch_errors, assert_file_field, assert_inner_hex, assert_no_packages, assert_no_ts_apps,
    assert_per_app, copy_fixture, remove_dir, run_check, write_file, INNER_HEX, RUST_APPS,
};
use guardrail3::domain::report::CheckResult;
use std::os::unix::fs::PermissionsExt;
#[allow(unused_imports)] // reason: symlink tests use this
use std::os::unix::fs::symlink;

/// Containers that have leaf subdirs in the golden fixture, per app.
/// These are the paths where adding an orphan/ triggers a "missing Cargo.toml" error.
const OUTER_CONTAINERS: &[&str] = &[
    "crates/app",
    "crates/domain",
    "crates/adapters/inbound",
    "crates/adapters/outbound",
    "crates/ports/outbound",
];

/// Inner hex containers that have leaf subdirs in the golden fixture.
const INNER_HEX_CONTAINERS: &[&str] = &[
    "app",
    "domain",
    "adapters/inbound",
];

/// Filter to only rule-6 errors (missing Cargo.toml or both Cargo.toml and crates/).
fn rule6_errors<'a>(errors: &'a [&'a CheckResult]) -> Vec<&'a CheckResult> {
    errors
        .iter()
        .filter(|e| {
            e.title.contains("missing Cargo.toml")
                || e.title.contains("both Cargo.toml and crates/")
        })
        .copied()
        .collect()
}

/// Add orphan/ dir (no Cargo.toml, no crates/) in every container with subdirs across all apps + inner hex.
fn add_orphan_everywhere(root: &std::path::Path, name: &str) {
    for app in RUST_APPS {
        for c in OUTER_CONTAINERS {
            write_file(
                root,
                &format!("apps/{app}/{c}/{name}/src/lib.rs"),
                "",
            );
        }
        // backend also has ports/inbound/api, so add orphan there too
        if *app == "backend" {
            write_file(
                root,
                &format!("apps/backend/crates/ports/inbound/{name}/src/lib.rs"),
                "",
            );
        }
    }
    for c in INNER_HEX_CONTAINERS {
        write_file(
            root,
            &format!("{INNER_HEX}/{c}/{name}/src/lib.rs"),
            "",
        );
    }
}

/// Count how many locations orphan_everywhere touches.
/// 3 apps * 5 outer containers + 1 backend ports/inbound + 3 inner hex = 19
fn orphan_everywhere_count() -> usize {
    let outer = RUST_APPS.len() * OUTER_CONTAINERS.len(); // 3 * 5 = 15
    let backend_ports_inbound = 1;
    let inner = INNER_HEX_CONTAINERS.len(); // 3
    outer + backend_ports_inbound + inner // 19
}

// ============================================================================
// GROUP A: Missing Cargo.toml (no crates/ either)
// ============================================================================

#[test]
fn subdir_missing_cargo_toml_everywhere() {
    let tmp = copy_fixture();
    add_orphan_everywhere(tmp.path(), "orphan");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);
    let expected = orphan_everywhere_count();
    assert_eq!(
        r6.len(),
        expected,
        "expected {expected} missing-Cargo.toml errors (one per container with subdirs), got {}: {r6:#?}",
        r6.len()
    );
    for err in &r6 {
        assert!(
            err.title.contains("missing Cargo.toml"),
            "expected 'missing Cargo.toml' in title, got: '{}'",
            err.title
        );
        assert!(
            err.title.contains("orphan"),
            "expected 'orphan' in title, got: '{}'",
            err.title
        );
        assert!(
            err.message.contains("no `Cargo.toml`") || err.message.contains("no `crates/`"),
            "expected message about missing Cargo.toml or crates/, got: '{}'",
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
fn subdir_completely_empty() {
    let tmp = copy_fixture();
    // Create empty subdirs (no files at all) in all containers across all apps + inner hex.
    for app in RUST_APPS {
        for c in OUTER_CONTAINERS {
            std::fs::create_dir_all(
                tmp.path().join(format!("apps/{app}/{c}/empty_sub")),
            )
            .expect("mkdir");
        }
        if *app == "backend" {
            std::fs::create_dir_all(
                tmp.path().join("apps/backend/crates/ports/inbound/empty_sub"),
            )
            .expect("mkdir");
        }
    }
    for c in INNER_HEX_CONTAINERS {
        std::fs::create_dir_all(
            tmp.path().join(format!("{INNER_HEX}/{c}/empty_sub")),
        )
        .expect("mkdir");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);
    let expected = orphan_everywhere_count();
    assert_eq!(
        r6.len(),
        expected,
        "expected {expected} errors for empty subdirs, got {}: {r6:#?}",
        r6.len()
    );
    for err in &r6 {
        assert!(
            err.title.contains("empty_sub") && err.title.contains("missing Cargo.toml"),
            "expected 'empty_sub' and 'missing Cargo.toml' in title, got: '{}'",
            err.title
        );
    }
    assert_file_field(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn multiple_invalid_subdirs() {
    let tmp = copy_fixture();
    add_orphan_everywhere(tmp.path(), "orphan1");
    add_orphan_everywhere(tmp.path(), "orphan2");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);
    let expected = orphan_everywhere_count() * 2;
    assert_eq!(
        r6.len(),
        expected,
        "expected {expected} errors (2 orphans * {count} locations), got {}: {r6:#?}",
        r6.len(),
        count = orphan_everywhere_count()
    );
    let orphan1: Vec<_> = r6.iter().filter(|e| e.title.contains("orphan1")).collect();
    let orphan2: Vec<_> = r6.iter().filter(|e| e.title.contains("orphan2")).collect();
    assert_eq!(
        orphan1.len(),
        orphan_everywhere_count(),
        "expected {} orphan1 errors, got {}: {orphan1:#?}",
        orphan_everywhere_count(),
        orphan1.len()
    );
    assert_eq!(
        orphan2.len(),
        orphan_everywhere_count(),
        "expected {} orphan2 errors, got {}: {orphan2:#?}",
        orphan_everywhere_count(),
        orphan2.len()
    );
    assert_file_field(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

// ============================================================================
// GROUP B: Conflicting (both Cargo.toml AND crates/)
// ============================================================================

#[test]
fn subdir_has_both_cargo_and_crates() {
    let tmp = copy_fixture();
    // Create hybrid/ with both Cargo.toml and crates/{domain,app,ports,adapters} in all containers.
    for app in RUST_APPS {
        for c in OUTER_CONTAINERS {
            let base = format!("apps/{app}/{c}/hybrid");
            write_file(
                tmp.path(),
                &format!("{base}/Cargo.toml"),
                "[package]\nname = \"hybrid\"\nversion = \"0.1.0\"\n",
            );
            // Create crates/ with enough inner structure to make has_crates = true
            for inner in &["domain", "app", "ports", "adapters"] {
                write_file(
                    tmp.path(),
                    &format!("{base}/crates/{inner}/.gitkeep"),
                    "",
                );
            }
        }
        if *app == "backend" {
            let base = "apps/backend/crates/ports/inbound/hybrid";
            write_file(
                tmp.path(),
                &format!("{base}/Cargo.toml"),
                "[package]\nname = \"hybrid\"\nversion = \"0.1.0\"\n",
            );
            for inner in &["domain", "app", "ports", "adapters"] {
                write_file(
                    tmp.path(),
                    &format!("{base}/crates/{inner}/.gitkeep"),
                    "",
                );
            }
        }
    }
    for c in INNER_HEX_CONTAINERS {
        let base = format!("{INNER_HEX}/{c}/hybrid");
        write_file(
            tmp.path(),
            &format!("{base}/Cargo.toml"),
            "[package]\nname = \"hybrid\"\nversion = \"0.1.0\"\n",
        );
        for inner in &["domain", "app", "ports", "adapters"] {
            write_file(
                tmp.path(),
                &format!("{base}/crates/{inner}/.gitkeep"),
                "",
            );
        }
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);
    let conflict_errors: Vec<_> = r6
        .iter()
        .filter(|e| e.title.contains("both Cargo.toml and crates/"))
        .collect();
    let expected = orphan_everywhere_count();
    assert_eq!(
        conflict_errors.len(),
        expected,
        "expected {expected} conflict errors (one per container), got {}: {conflict_errors:#?}",
        conflict_errors.len()
    );
    for err in &conflict_errors {
        assert!(
            err.title.contains("hybrid"),
            "expected 'hybrid' in title, got: '{}'",
            err.title
        );
        assert!(
            err.message.contains("not both"),
            "expected 'not both' in message, got: '{}'",
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
// GROUP C: Valid cases (should produce 0 rule6 errors)
// ============================================================================

#[test]
fn golden_baseline() {
    let tmp = copy_fixture();
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);
    assert!(
        r6.is_empty(),
        "unmodified golden should have 0 rule6 errors, got {}: {r6:#?}",
        r6.len()
    );
}

#[test]
fn gitkeep_alongside_cargo_toml() {
    let tmp = copy_fixture();
    // domain/types already has Cargo.toml — adding .gitkeep should be harmless.
    write_file(
        tmp.path(),
        "apps/devctl/crates/domain/types/.gitkeep",
        "",
    );
    write_file(
        tmp.path(),
        "apps/backend/crates/app/commands/.gitkeep",
        "",
    );
    write_file(
        tmp.path(),
        "apps/worker/crates/app/processor/.gitkeep",
        "",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);
    assert!(
        r6.is_empty(),
        "gitkeep alongside Cargo.toml should not cause rule6 errors, got: {r6:#?}"
    );
}

#[test]
fn gitkeep_alongside_hex_in_hex() {
    let tmp = copy_fixture();
    // mcp/ already has crates/ — adding .gitkeep should be harmless.
    write_file(
        tmp.path(),
        "apps/backend/crates/adapters/inbound/mcp/.gitkeep",
        "",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);
    assert!(
        r6.is_empty(),
        "gitkeep alongside hex-in-hex crates/ should not cause rule6 errors, got: {r6:#?}"
    );
}

#[test]
fn hex_in_hex_valid() {
    let tmp = copy_fixture();
    // Golden backend/mcp is a valid hex-in-hex structure — must pass with 0 rule6 errors.
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);
    assert!(
        r6.is_empty(),
        "golden hex-in-hex should pass with 0 rule6 errors, got: {r6:#?}"
    );
}

// ============================================================================
// GROUP D: Hex-in-hex recursion
// ============================================================================

#[test]
fn hex_in_hex_inner_broken() {
    let tmp = copy_fixture();
    // Remove domain/ from inner hex — triggers structural error (not rule6, but rule 02/04).
    remove_dir(
        tmp.path(),
        "apps/backend/crates/adapters/inbound/mcp/crates/domain",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert!(
        errors
            .iter()
            .any(|e| e.title.contains("domain") && e.file.as_deref().unwrap_or("").contains("mcp/crates")),
        "expected error about missing domain in inner hex, got: {errors:#?}"
    );
}

#[test]
fn hex_in_hex_inner_missing_multiple() {
    let tmp = copy_fixture();
    // Remove multiple dirs from inner hex.
    remove_dir(
        tmp.path(),
        "apps/backend/crates/adapters/inbound/mcp/crates/domain",
    );
    remove_dir(
        tmp.path(),
        "apps/backend/crates/adapters/inbound/mcp/crates/app",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let inner_errors: Vec<_> = errors
        .iter()
        .filter(|e| e.file.as_deref().unwrap_or("").contains("mcp/crates"))
        .collect();
    assert!(
        inner_errors.len() >= 2,
        "expected at least 2 inner hex errors (domain + app missing), got {}: {inner_errors:#?}",
        inner_errors.len()
    );
}

#[test]
fn hex_in_hex_at_various_containers() {
    let tmp = copy_fixture();
    // Create valid hex-in-hex in domain/ container (not just adapters/inbound).
    // Replace devctl/domain/types crate with a hex-in-hex structure.
    remove_dir(tmp.path(), "apps/devctl/crates/domain/types");
    // types/ now becomes a hex-in-hex with its own crates/ directory.
    for container in &["app", "domain", "adapters/inbound", "adapters/outbound", "ports/inbound", "ports/outbound"] {
        write_file(
            tmp.path(),
            &format!("apps/devctl/crates/domain/types/crates/{container}/.gitkeep"),
            "",
        );
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);
    // types/ is now hex-in-hex, so rule6 should not flag it as missing Cargo.toml.
    let types_errors: Vec<_> = r6
        .iter()
        .filter(|e| e.title.contains("types") && e.title.contains("missing Cargo.toml"))
        .collect();
    assert!(
        types_errors.is_empty(),
        "hex-in-hex in domain/ should be valid, got: {types_errors:#?}"
    );
}

#[test]
fn triple_nested_hex_valid() {
    let tmp = copy_fixture();
    // Create triple-nested hex: mcp -> inner adapters/inbound/transport -> deeper hex.
    // Replace transport crate with a hex-in-hex.
    remove_dir(
        tmp.path(),
        "apps/backend/crates/adapters/inbound/mcp/crates/adapters/inbound/transport",
    );
    let deep_base = "apps/backend/crates/adapters/inbound/mcp/crates/adapters/inbound/transport";
    for container in &["app", "domain", "adapters/inbound", "adapters/outbound", "ports/inbound", "ports/outbound"] {
        write_file(
            tmp.path(),
            &format!("{deep_base}/crates/{container}/.gitkeep"),
            "",
        );
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);
    let transport_errors: Vec<_> = r6
        .iter()
        .filter(|e| e.title.contains("transport") && e.title.contains("missing Cargo.toml"))
        .collect();
    assert!(
        transport_errors.is_empty(),
        "triple-nested hex-in-hex should be valid, got: {transport_errors:#?}"
    );
}

// ============================================================================
// GROUP E: Edge cases
// ============================================================================

#[test]
fn subdir_with_only_gitkeep() {
    let tmp = copy_fixture();
    // A leaf subdir that has only .gitkeep — no Cargo.toml, no crates/.
    // KNOWN GAP: .gitkeep-only leaf subdir is NOT accepted by rule 06.
    // This test expects the error to PROVE the gap exists.
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/placeholder/.gitkeep",
        "",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);
    let placeholder_errors: Vec<_> = r6
        .iter()
        .filter(|e| e.title.contains("placeholder"))
        .collect();
    // Expect failure: rule 06 sees no Cargo.toml and no crates/, so it reports an error.
    assert_eq!(
        placeholder_errors.len(),
        1,
        "expected 1 error for .gitkeep-only leaf subdir (known gap), got {}: {placeholder_errors:#?}",
        placeholder_errors.len()
    );
    assert!(
        placeholder_errors[0].title.contains("missing Cargo.toml"),
        "expected 'missing Cargo.toml' in title for .gitkeep-only leaf, got: '{}'",
        placeholder_errors[0].title
    );
}

#[test]
fn subdir_is_file_not_dir() {
    let tmp = copy_fixture();
    // Create a file named like a subdir inside a container.
    // list_dir_names only returns directories, so this file is not checked by rule 06.
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/not_a_dir",
        "I am a file, not a directory",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);
    let file_errors: Vec<_> = r6
        .iter()
        .filter(|e| e.title.contains("not_a_dir"))
        .collect();
    assert!(
        file_errors.is_empty(),
        "a file (not dir) should not be checked by rule 06, got: {file_errors:#?}"
    );
}

#[test]
fn symlink_subdir() {
    let tmp = copy_fixture();
    // Create a symlink inside a container. DirEntry::file_type returns symlink, not dir.
    // list_dir_names checks ft.is_dir() which returns false for symlinks.
    let target = tmp.path().join("apps/devctl/crates/app/core");
    let link = tmp.path().join("apps/devctl/crates/app/link_to_core");
    symlink(&target, &link).expect("create symlink");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);
    let link_errors: Vec<_> = r6
        .iter()
        .filter(|e| e.title.contains("link_to_core"))
        .collect();
    assert!(
        link_errors.is_empty(),
        "symlink should not be checked by rule 06 (not a dir), got: {link_errors:#?}"
    );
}

#[test]
fn permission_denied_subdir() {
    let tmp = copy_fixture();
    // chmod 000 on a crate subdir. read_file(Cargo.toml) fails, list_dir(crates/) returns empty.
    // Both has_cargo and has_crates are false → "missing Cargo.toml" error.
    let target = tmp.path().join("apps/devctl/crates/app/core");
    let perms = std::fs::Permissions::from_mode(0o000);
    std::fs::set_permissions(&target, perms).expect("chmod 000");
    let results = run_check(tmp.path());
    // Restore permissions so tempdir cleanup works.
    let restore = std::fs::Permissions::from_mode(0o755);
    std::fs::set_permissions(&target, restore).expect("chmod restore");
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);
    let core_errors: Vec<_> = r6
        .iter()
        .filter(|e| e.title.contains("core") && e.title.contains("missing Cargo.toml"))
        .collect();
    assert_eq!(
        core_errors.len(),
        1,
        "expected 1 error for permission-denied subdir (Cargo.toml unreadable), got {}: {core_errors:#?}",
        core_errors.len()
    );
}

#[test]
fn cargo_toml_exists_but_empty() {
    let tmp = copy_fixture();
    // Replace a valid Cargo.toml with empty content. read_file returns Some("").
    // has_cargo = true. Rule 06 passes (content validity is not its concern).
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/core/Cargo.toml",
        "",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);
    let core_errors: Vec<_> = r6
        .iter()
        .filter(|e| e.title.contains("core"))
        .collect();
    assert!(
        core_errors.is_empty(),
        "empty Cargo.toml should still satisfy rule 06 (content not checked), got: {core_errors:#?}"
    );
}

#[test]
fn crates_dir_exists_but_empty() {
    let tmp = copy_fixture();
    // Create a subdir with an empty crates/ directory (no entries inside).
    // list_dir returns empty → has_crates = false. No Cargo.toml either → "missing Cargo.toml".
    std::fs::create_dir_all(
        tmp.path().join("apps/devctl/crates/app/hollow/crates"),
    )
    .expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);
    let hollow_errors: Vec<_> = r6
        .iter()
        .filter(|e| e.title.contains("hollow"))
        .collect();
    assert_eq!(
        hollow_errors.len(),
        1,
        "expected 1 error for hollow subdir (empty crates/ dir), got {}: {hollow_errors:#?}",
        hollow_errors.len()
    );
    assert!(
        hollow_errors[0].title.contains("missing Cargo.toml"),
        "expected 'missing Cargo.toml' (empty crates/ treated as no crates/), got: '{}'",
        hollow_errors[0].title
    );
}

// ============================================================================
// GROUP F: Cross-cutting
// ============================================================================

#[test]
fn ts_apps_not_checked() {
    let tmp = copy_fixture();
    // Add invalid subdirs inside TS app modules (admin, landing).
    // These have no Cargo.toml at app root → not detected as Rust apps.
    write_file(
        tmp.path(),
        "apps/admin/crates/app/orphan_ts/src/lib.rs",
        "",
    );
    write_file(
        tmp.path(),
        "apps/landing/crates/domain/orphan_ts/src/lib.rs",
        "",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);
    let ts_errors: Vec<_> = r6
        .iter()
        .filter(|e| e.title.contains("admin") || e.title.contains("landing"))
        .collect();
    assert!(
        ts_errors.is_empty(),
        "TS apps should not produce rule 06 errors, got: {ts_errors:#?}"
    );
    assert_no_ts_apps(&errors);
}

#[test]
fn packages_not_checked() {
    let tmp = copy_fixture();
    // Add invalid dirs at packages/ level. Rule 06 only runs on apps/.
    write_file(
        tmp.path(),
        "packages/shared-types/crates/app/orphan_pkg/src/lib.rs",
        "",
    );
    write_file(
        tmp.path(),
        "packages/ui-kit/crates/domain/orphan_pkg/src/lib.rs",
        "",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);
    let pkg_errors: Vec<_> = r6
        .iter()
        .filter(|e| e.title.contains("shared-types") || e.title.contains("ui-kit"))
        .collect();
    assert!(
        pkg_errors.is_empty(),
        "packages should not produce rule 06 errors, got: {pkg_errors:#?}"
    );
    assert_no_packages(&errors);
}

#[test]
fn new_app_gets_checked() {
    let tmp = copy_fixture();
    // Create a new Rust app "scheduler" with an invalid leaf subdir.
    write_file(
        tmp.path(),
        "apps/scheduler/Cargo.toml",
        "[package]\nname = \"scheduler\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(tmp.path(), "apps/scheduler/src/main.rs", "fn main() {}");
    // Create crates structure with containers.
    for container in &["app", "domain", "adapters/inbound", "adapters/outbound", "ports/inbound", "ports/outbound"] {
        write_file(
            tmp.path(),
            &format!("apps/scheduler/crates/{container}/.gitkeep"),
            "",
        );
    }
    // Add an invalid leaf subdir in app/ container.
    write_file(
        tmp.path(),
        "apps/scheduler/crates/app/invalid_leaf/src/lib.rs",
        "",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);
    let scheduler_errors: Vec<_> = r6
        .iter()
        .filter(|e| e.title.contains("scheduler") && e.title.contains("invalid_leaf"))
        .collect();
    assert_eq!(
        scheduler_errors.len(),
        1,
        "expected 1 error for new scheduler app's invalid leaf, got {}: {scheduler_errors:#?}",
        scheduler_errors.len()
    );
    assert!(
        scheduler_errors[0].title.contains("missing Cargo.toml"),
        "expected 'missing Cargo.toml' for scheduler invalid leaf, got: '{}'",
        scheduler_errors[0].title
    );
}

#[test]
fn inner_hex_leaf_invalid_outer_clean() {
    let tmp = copy_fixture();
    // Break only inner hex leaf — add orphan in inner hex app/ container.
    write_file(
        tmp.path(),
        &format!("{INNER_HEX}/app/orphan_inner/src/lib.rs"),
        "",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);
    // Only 1 error, in inner hex.
    assert_eq!(
        r6.len(),
        1,
        "expected exactly 1 rule6 error (inner hex only), got {}: {r6:#?}",
        r6.len()
    );
    assert!(
        r6[0].title.contains("orphan_inner"),
        "expected 'orphan_inner' in title, got: '{}'",
        r6[0].title
    );
    assert!(
        r6[0].file.as_deref().unwrap_or("").contains("mcp/crates"),
        "expected inner hex path in file field, got: {:?}",
        r6[0].file
    );
    // Verify outer apps have no rule6 errors.
    let outer_errors: Vec<_> = r6
        .iter()
        .filter(|e| !e.file.as_deref().unwrap_or("").contains("mcp/crates"))
        .collect();
    assert!(
        outer_errors.is_empty(),
        "outer apps should be clean, got: {outer_errors:#?}"
    );
}

#[test]
fn inner_hex_label_prefix_correct() {
    let tmp = copy_fixture();
    // Add orphan in inner hex app/ container and verify the nested label format.
    write_file(
        tmp.path(),
        &format!("{INNER_HEX}/app/nested_orphan/src/lib.rs"),
        "",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);
    let nested: Vec<_> = r6
        .iter()
        .filter(|e| e.title.contains("nested_orphan"))
        .collect();
    assert_eq!(nested.len(), 1, "expected 1 error, got {}: {nested:#?}", nested.len());
    // The label should contain the full nested path from check_crates_dir recursion.
    // Format: crates/adapters/inbound/mcp/crates/app/nested_orphan/
    assert!(
        nested[0].title.contains("crates/adapters/inbound/mcp/crates/app/nested_orphan"),
        "expected nested label path in title, got: '{}'",
        nested[0].title
    );
}

#[test]
fn idempotent_results() {
    let tmp = copy_fixture();
    // Add one orphan to make the results non-trivial.
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/orphan_idem/src/lib.rs",
        "",
    );
    let results1 = run_check(tmp.path());
    let errors1 = arch_errors(&results1);
    let r6_1 = rule6_errors(&errors1);

    let results2 = run_check(tmp.path());
    let errors2 = arch_errors(&results2);
    let r6_2 = rule6_errors(&errors2);

    assert_eq!(
        r6_1.len(),
        r6_2.len(),
        "idempotent: expected same count, got {} vs {}",
        r6_1.len(),
        r6_2.len()
    );
    for (a, b) in r6_1.iter().zip(r6_2.iter()) {
        assert_eq!(a.title, b.title, "idempotent: title mismatch");
        assert_eq!(a.message, b.message, "idempotent: message mismatch");
        assert_eq!(a.file, b.file, "idempotent: file mismatch");
    }
}

#[test]
fn different_breakage_per_app() {
    let tmp = copy_fixture();

    // devctl: missing Cargo.toml (orphan)
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/orphan_devctl/src/lib.rs",
        "",
    );

    // worker: both conflict (hybrid with Cargo.toml + crates/)
    write_file(
        tmp.path(),
        "apps/worker/crates/app/hybrid_worker/Cargo.toml",
        "[package]\nname = \"hybrid-worker\"\nversion = \"0.1.0\"\n",
    );
    for inner in &["domain", "app", "ports", "adapters"] {
        write_file(
            tmp.path(),
            &format!("apps/worker/crates/app/hybrid_worker/crates/{inner}/.gitkeep"),
            "",
        );
    }

    // backend inner hex: orphan in inner hex domain/
    write_file(
        tmp.path(),
        &format!("{INNER_HEX}/domain/orphan_inner_be/src/lib.rs"),
        "",
    );

    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);

    // devctl: missing Cargo.toml
    let devctl_errs: Vec<_> = r6
        .iter()
        .filter(|e| e.title.contains("devctl") && e.title.contains("orphan_devctl"))
        .collect();
    assert_eq!(
        devctl_errs.len(),
        1,
        "expected 1 devctl missing-Cargo.toml error, got {}: {devctl_errs:#?}",
        devctl_errs.len()
    );
    assert!(
        devctl_errs[0].title.contains("missing Cargo.toml"),
        "devctl error should be missing Cargo.toml, got: '{}'",
        devctl_errs[0].title
    );

    // worker: both conflict
    let worker_errs: Vec<_> = r6
        .iter()
        .filter(|e| e.title.contains("worker") && e.title.contains("hybrid_worker"))
        .collect();
    assert_eq!(
        worker_errs.len(),
        1,
        "expected 1 worker conflict error, got {}: {worker_errs:#?}",
        worker_errs.len()
    );
    assert!(
        worker_errs[0].title.contains("both Cargo.toml and crates/"),
        "worker error should be conflict, got: '{}'",
        worker_errs[0].title
    );

    // backend inner hex: missing Cargo.toml
    let inner_errs: Vec<_> = r6
        .iter()
        .filter(|e| e.title.contains("orphan_inner_be"))
        .collect();
    assert_eq!(
        inner_errs.len(),
        1,
        "expected 1 inner hex missing-Cargo.toml error, got {}: {inner_errs:#?}",
        inner_errs.len()
    );
    assert!(
        inner_errs[0].file.as_deref().unwrap_or("").contains("mcp/crates"),
        "inner hex error should reference mcp/crates in file, got: {:?}",
        inner_errs[0].file
    );

    assert_file_field(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}
