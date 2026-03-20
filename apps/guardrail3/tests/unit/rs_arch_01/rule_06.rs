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
/// Note: devctl and worker have only .gitkeep in ports/inbound (no existing subdirs),
/// so adding an orphan there creates the first subdir that rule 06 checks.
const OUTER_CONTAINERS: &[&str] = &[
    "crates/app",
    "crates/domain",
    "crates/adapters/inbound",
    "crates/adapters/outbound",
    "crates/ports/outbound",
    "crates/ports/inbound",
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
/// 3 apps * 6 outer containers + 3 inner hex = 21
fn orphan_everywhere_count() -> usize {
    let outer = RUST_APPS.len() * OUTER_CONTAINERS.len(); // 3 * 6 = 18
    let inner = INNER_HEX_CONTAINERS.len(); // 3
    outer + inner // 21
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
    assert_file_field(&r6);
    assert_per_app(&r6);
    assert_inner_hex(&r6);
    assert_no_ts_apps(&r6);
    assert_no_packages(&r6);
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
    assert_file_field(&r6);
    assert_per_app(&r6);
    assert_inner_hex(&r6);
    assert_no_ts_apps(&r6);
    assert_no_packages(&r6);
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
    assert_file_field(&r6);
    assert_per_app(&r6);
    assert_inner_hex(&r6);
    assert_no_ts_apps(&r6);
    assert_no_packages(&r6);
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
        r6.len(),
        expected,
        "expected {expected} total rule6 errors (all conflicts), got {}: {r6:#?}",
        r6.len()
    );
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
    assert_file_field(&r6);
    assert_per_app(&r6);
    assert_inner_hex(&r6);
    assert_no_ts_apps(&r6);
    assert_no_packages(&r6);
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
    assert_eq!(
        r6.len(),
        0,
        "unmodified golden should have 0 rule6 errors, got {}: {r6:#?}",
        r6.len()
    );
    assert_no_ts_apps(&r6);
    assert_no_packages(&r6);
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
    assert_eq!(
        r6.len(),
        0,
        "gitkeep alongside Cargo.toml should not cause rule6 errors, got {}: {r6:#?}",
        r6.len()
    );
    assert_no_ts_apps(&r6);
    assert_no_packages(&r6);
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
    assert_eq!(
        r6.len(),
        0,
        "gitkeep alongside hex-in-hex crates/ should not cause rule6 errors, got {}: {r6:#?}",
        r6.len()
    );
    assert_no_ts_apps(&r6);
    assert_no_packages(&r6);
}

#[test]
fn hex_in_hex_valid() {
    // NOTE: Redundant with golden_baseline — golden already validates hex-in-hex passes.
    // Kept as explicit documentation that hex-in-hex structure is a rule6 concern.
    let tmp = copy_fixture();
    // Golden backend/mcp is a valid hex-in-hex structure — must pass with 0 rule6 errors.
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);
    assert_eq!(
        r6.len(),
        0,
        "golden hex-in-hex should pass with 0 rule6 errors, got {}: {r6:#?}",
        r6.len()
    );
    assert_no_ts_apps(&r6);
    assert_no_packages(&r6);
}

// ============================================================================
// GROUP D: Hex-in-hex recursion
// ============================================================================

#[test]
fn hex_in_hex_inner_broken() {
    // NOTE: This test exercises OTHER rules (rule 02/04) through hex-in-hex recursion,
    // not rule 06 directly. Removing domain/ is a structural violation (missing required
    // container), which is rule 02's responsibility. Rule 06 only checks leaf subdirs
    // within containers, not the containers themselves.
    let tmp = copy_fixture();
    remove_dir(
        tmp.path(),
        "apps/backend/crates/adapters/inbound/mcp/crates/domain",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);
    // Rule 06 should produce 0 errors — domain/ is a structural container, not a leaf.
    assert_eq!(
        r6.len(),
        0,
        "removing structural domain/ from inner hex should not produce rule6 errors, got {}: {r6:#?}",
        r6.len()
    );
    // But there should be arch errors from other rules (rule 02) about missing domain/.
    assert!(
        errors
            .iter()
            .any(|e| e.title.contains("domain") && e.file.as_deref().unwrap_or("").contains("mcp/crates")),
        "expected error about missing domain in inner hex from other rules, got: {errors:#?}"
    );
    assert_no_ts_apps(&r6);
    assert_no_packages(&r6);
}

#[test]
fn hex_in_hex_inner_missing_multiple() {
    // NOTE: This test exercises OTHER rules (rule 02/04) through hex-in-hex recursion,
    // not rule 06 directly. Removing domain/ and app/ are structural violations (missing
    // required containers), which is rule 02's responsibility. Rule 06 only checks leaf
    // subdirs within containers, not the containers themselves.
    let tmp = copy_fixture();
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
    let r6 = rule6_errors(&errors);
    // Rule 06 should produce 0 errors — domain/ and app/ are structural containers, not leaves.
    assert_eq!(
        r6.len(),
        0,
        "removing structural dirs from inner hex should not produce rule6 errors, got {}: {r6:#?}",
        r6.len()
    );
    // Other rules should flag the missing structural dirs.
    let inner_errors: Vec<_> = errors
        .iter()
        .filter(|e| e.file.as_deref().unwrap_or("").contains("mcp/crates"))
        .collect();
    assert_eq!(
        inner_errors.len(),
        2,
        "expected exactly 2 inner hex errors (domain + app missing), got {}: {inner_errors:#?}",
        inner_errors.len()
    );
    assert_no_ts_apps(&r6);
    assert_no_packages(&r6);
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
    // types/ is now hex-in-hex, so total rule6 count should be 0.
    assert_eq!(
        r6.len(),
        0,
        "hex-in-hex in domain/ should produce 0 rule6 errors, got {}: {r6:#?}",
        r6.len()
    );
    assert_no_ts_apps(&r6);
    assert_no_packages(&r6);
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
    // Total rule6 count should be 0 for valid triple-nested hex.
    assert_eq!(
        r6.len(),
        0,
        "triple-nested hex-in-hex should produce 0 rule6 errors, got {}: {r6:#?}",
        r6.len()
    );
    assert_no_ts_apps(&r6);
    assert_no_packages(&r6);
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
    assert_eq!(
        r6.len(),
        1,
        "expected exactly 1 total rule6 error for .gitkeep-only leaf subdir, got {}: {r6:#?}",
        r6.len()
    );
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
    assert_file_field(&r6);
    assert_no_ts_apps(&r6);
    assert_no_packages(&r6);
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
    assert_eq!(
        r6.len(),
        0,
        "expected 0 total rule6 errors when adding a file (not dir), got {}: {r6:#?}",
        r6.len()
    );
    let file_errors: Vec<_> = r6
        .iter()
        .filter(|e| e.title.contains("not_a_dir"))
        .collect();
    assert!(
        file_errors.is_empty(),
        "a file (not dir) should not be checked by rule 06, got: {file_errors:#?}"
    );
    assert_no_ts_apps(&r6);
    assert_no_packages(&r6);
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
    assert_eq!(
        r6.len(),
        0,
        "expected 0 total rule6 errors when adding a symlink, got {}: {r6:#?}",
        r6.len()
    );
    let link_errors: Vec<_> = r6
        .iter()
        .filter(|e| e.title.contains("link_to_core"))
        .collect();
    assert!(
        link_errors.is_empty(),
        "symlink should not be checked by rule 06 (not a dir), got: {link_errors:#?}"
    );
    assert_no_ts_apps(&r6);
    assert_no_packages(&r6);
}

#[test]
fn permission_denied_subdir() {
    let tmp = copy_fixture();
    // chmod 000 on a crate subdir. read_file(Cargo.toml) fails, list_dir(crates/) returns empty.
    // Both has_cargo and has_crates are false -> "missing Cargo.toml" error.
    let target = tmp.path().join("apps/devctl/crates/app/core");
    let perms = std::fs::Permissions::from_mode(0o000);
    std::fs::set_permissions(&target, perms).expect("chmod 000");
    let results = run_check(tmp.path());
    // Restore permissions so tempdir cleanup works.
    let restore = std::fs::Permissions::from_mode(0o755);
    std::fs::set_permissions(&target, restore).expect("chmod restore");
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);
    assert_eq!(
        r6.len(),
        1,
        "expected exactly 1 total rule6 error for permission-denied subdir, got {}: {r6:#?}",
        r6.len()
    );
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
    assert_file_field(&r6);
    assert_no_ts_apps(&r6);
    assert_no_packages(&r6);
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
    assert_eq!(
        r6.len(),
        0,
        "expected 0 total rule6 errors for empty Cargo.toml (content not checked), got {}: {r6:#?}",
        r6.len()
    );
    let core_errors: Vec<_> = r6
        .iter()
        .filter(|e| e.title.contains("core"))
        .collect();
    assert!(
        core_errors.is_empty(),
        "empty Cargo.toml should still satisfy rule 06 (content not checked), got: {core_errors:#?}"
    );
    assert_no_ts_apps(&r6);
    assert_no_packages(&r6);
}

#[test]
fn crates_dir_exists_but_empty() {
    let tmp = copy_fixture();
    // Create a subdir with an empty crates/ directory (no entries inside).
    // list_dir returns empty -> has_crates = false. No Cargo.toml either -> "missing Cargo.toml".
    std::fs::create_dir_all(
        tmp.path().join("apps/devctl/crates/app/hollow/crates"),
    )
    .expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);
    assert_eq!(
        r6.len(),
        1,
        "expected exactly 1 total rule6 error for hollow subdir (empty crates/ dir), got {}: {r6:#?}",
        r6.len()
    );
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
    assert_file_field(&r6);
    assert_no_ts_apps(&r6);
    assert_no_packages(&r6);
}

// ============================================================================
// GROUP F: Cross-cutting
// ============================================================================

#[test]
fn ts_apps_not_checked() {
    let tmp = copy_fixture();
    // Add invalid subdirs inside TS app modules (admin, landing).
    // These have no Cargo.toml at app root -> not detected as Rust apps.
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
    assert_eq!(
        r6.len(),
        0,
        "expected 0 total rule6 errors from TS apps, got {}: {r6:#?}",
        r6.len()
    );
    let ts_errors: Vec<_> = r6
        .iter()
        .filter(|e| e.title.contains("admin") || e.title.contains("landing"))
        .collect();
    assert!(
        ts_errors.is_empty(),
        "TS apps should not produce rule 06 errors, got: {ts_errors:#?}"
    );
    assert_no_ts_apps(&r6);
    assert_no_packages(&r6);
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
    assert_eq!(
        r6.len(),
        0,
        "expected 0 total rule6 errors from packages, got {}: {r6:#?}",
        r6.len()
    );
    let pkg_errors: Vec<_> = r6
        .iter()
        .filter(|e| e.title.contains("shared-types") || e.title.contains("ui-kit"))
        .collect();
    assert!(
        pkg_errors.is_empty(),
        "packages should not produce rule 06 errors, got: {pkg_errors:#?}"
    );
    assert_no_ts_apps(&r6);
    assert_no_packages(&r6);
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
    assert_eq!(
        r6.len(),
        1,
        "expected exactly 1 total rule6 error for new scheduler app, got {}: {r6:#?}",
        r6.len()
    );
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
    assert_file_field(&r6);
    assert_no_ts_apps(&r6);
    assert_no_packages(&r6);
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
    assert_no_ts_apps(&r6);
    assert_no_packages(&r6);
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
    assert_eq!(
        r6.len(),
        1,
        "expected exactly 1 total rule6 error for nested orphan, got {}: {r6:#?}",
        r6.len()
    );
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
    assert_file_field(&r6);
    assert_no_ts_apps(&r6);
    assert_no_packages(&r6);
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

    // Absolute count: exactly 1 orphan added, exactly 1 rule6 error expected.
    assert_eq!(
        r6_1.len(),
        1,
        "expected exactly 1 rule6 error for single orphan, got {}: {r6_1:#?}",
        r6_1.len()
    );
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
    assert_no_ts_apps(&r6_1);
    assert_no_packages(&r6_1);
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

    // Total: 1 devctl orphan + 1 worker conflict + 1 inner hex orphan = 3
    assert_eq!(
        r6.len(),
        3,
        "expected exactly 3 total rule6 errors (1 devctl + 1 worker + 1 inner hex), got {}: {r6:#?}",
        r6.len()
    );

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

    assert_file_field(&r6);
    assert_no_ts_apps(&r6);
    assert_no_packages(&r6);
}

// ============================================================================
// GROUP G: New edge-case and behavior-documenting tests
// ============================================================================

#[test]
fn missing_container_early_return() {
    // Remove entire devctl/crates/app/ dir. When metadata is None (container doesn't exist),
    // the check should early-return with 0 rule6 errors. Other rules catch the missing dir.
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/devctl/crates/app");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);
    assert_eq!(
        r6.len(),
        0,
        "missing container dir should produce 0 rule6 errors (early return), got {}: {r6:#?}",
        r6.len()
    );
    // Other rules should catch the missing app/ container.
    assert!(
        errors.iter().any(|e| e.title.contains("app") && e.file.as_deref().unwrap_or("").contains("devctl")),
        "expected other rules to flag missing app/ in devctl, got: {errors:#?}"
    );
    assert_no_ts_apps(&r6);
    assert_no_packages(&r6);
}

#[test]
fn malformed_cargo_toml_still_valid() {
    // Create a subdir with Cargo.toml containing garbage (not valid TOML).
    // has_cargo = true (file exists and is readable), so rule 06 passes.
    // Documents that rule 06 is content-agnostic — it only checks file existence.
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/malformed_crate/Cargo.toml",
        "this is garbage not toml }{}{}{",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);
    assert_eq!(
        r6.len(),
        0,
        "malformed Cargo.toml should still satisfy rule 06 (content-agnostic), got {}: {r6:#?}",
        r6.len()
    );
    let malformed_errors: Vec<_> = r6
        .iter()
        .filter(|e| e.title.contains("malformed_crate"))
        .collect();
    assert!(
        malformed_errors.is_empty(),
        "malformed Cargo.toml content should not produce rule6 errors, got: {malformed_errors:#?}"
    );
    assert_no_ts_apps(&r6);
    assert_no_packages(&r6);
}

#[test]
fn hidden_dir_as_leaf() {
    // Create .hidden/ inside containers everywhere (no Cargo.toml, no crates/).
    // Documents that hidden dirs ARE checked by rule 06 (list_dir includes them).
    let tmp = copy_fixture();
    // Add .hidden/ in a few containers across apps
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/.hidden/src/lib.rs",
        "",
    );
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/.hidden/src/lib.rs",
        "",
    );
    write_file(
        tmp.path(),
        "apps/worker/crates/adapters/inbound/.hidden/src/lib.rs",
        "",
    );
    write_file(
        tmp.path(),
        &format!("{INNER_HEX}/app/.hidden/src/lib.rs"),
        "",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);
    let hidden_errors: Vec<_> = r6
        .iter()
        .filter(|e| e.title.contains(".hidden"))
        .collect();
    // Hidden dirs should fire "missing Cargo.toml" because they have no Cargo.toml or crates/
    assert_eq!(
        hidden_errors.len(),
        4,
        "expected 4 errors for .hidden dirs (one per location), got {}: {hidden_errors:#?}",
        hidden_errors.len()
    );
    assert_eq!(
        r6.len(),
        4,
        "expected exactly 4 total rule6 errors for hidden dirs, got {}: {r6:#?}",
        r6.len()
    );
    for err in &hidden_errors {
        assert!(
            err.title.contains("missing Cargo.toml"),
            "expected 'missing Cargo.toml' for hidden dir, got: '{}'",
            err.title
        );
    }
    assert_file_field(&r6);
    assert_no_ts_apps(&r6);
    assert_no_packages(&r6);
}

#[test]
fn cargo_toml_wrong_case() {
    // Create subdir with `cargo.toml` (lowercase) instead of `Cargo.toml`.
    // On case-sensitive FS: read_file("Cargo.toml") returns None -> fires.
    // On case-insensitive FS (macOS default): resolves -> passes.
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/wrong_case/cargo.toml",
        "[package]\nname = \"wrong-case\"\nversion = \"0.1.0\"\n",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);
    let wrong_case_errors: Vec<_> = r6
        .iter()
        .filter(|e| e.title.contains("wrong_case"))
        .collect();
    // Case-insensitive FS (macOS): cargo.toml resolves as Cargo.toml -> 0 errors
    // Case-sensitive FS (Linux): read_file("Cargo.toml") returns None -> 1 error
    assert!(
        wrong_case_errors.is_empty() || wrong_case_errors.len() == 1,
        "expected 0 (case-insensitive) or 1 (case-sensitive) error for wrong_case, got {}: {wrong_case_errors:#?}",
        wrong_case_errors.len()
    );
    assert_eq!(
        r6.len(),
        wrong_case_errors.len(),
        "total r6 count should match wrong_case errors, got r6={} vs wrong_case={}: {r6:#?}",
        r6.len(),
        wrong_case_errors.len()
    );
    assert_no_ts_apps(&r6);
    assert_no_packages(&r6);
}

#[test]
fn nested_garbage_no_recursion() {
    // Create orphan/ with orphan/level2/level3/level4/lib.rs.
    // Assert exactly 1 error for orphan/ (missing Cargo.toml). No recursion into garbage.
    // Assert no error mentions "level2", "level3", or "level4" — rule 06 does not recurse into non-hex subdirs.
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/junk_orphan/level2/level3/level4/lib.rs",
        "",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);
    assert_eq!(
        r6.len(),
        1,
        "expected exactly 1 total rule6 error for junk_orphan (no recursion into garbage), got {}: {r6:#?}",
        r6.len()
    );
    assert!(
        r6[0].title.contains("junk_orphan"),
        "expected 'junk_orphan' in title, got: '{}'",
        r6[0].title
    );
    assert!(
        r6[0].title.contains("missing Cargo.toml"),
        "expected 'missing Cargo.toml' in title, got: '{}'",
        r6[0].title
    );
    // Verify no error mentions the nested garbage paths
    for err in &r6 {
        assert!(
            !err.title.contains("level2") && !err.title.contains("level3") && !err.title.contains("level4"),
            "rule 06 should not recurse into non-hex garbage, but found level2/3/4 in error: '{}'",
            err.title
        );
        assert!(
            !err.message.contains("level2") && !err.message.contains("level3") && !err.message.contains("level4"),
            "rule 06 should not recurse into non-hex garbage, but found level2/3/4 in message: '{}'",
            err.message
        );
    }
    assert_file_field(&r6);
    assert_no_ts_apps(&r6);
    assert_no_packages(&r6);
}

#[test]
fn dangling_symlink_in_crates_dir() {
    // Create a subdir with crates/ containing only a dangling symlink.
    // list_dir returns the symlink entry (non-empty) -> has_crates=true -> treated as hex-in-hex.
    // The recursion will then fail on the inner hex structure.
    // CHECK BUG: dangling symlink makes list_dir non-empty, so has_crates=true,
    // even though the symlink target doesn't exist. The test documents this behavior.
    let tmp = copy_fixture();
    let crates_dir = tmp.path().join("apps/devctl/crates/app/phantom/crates");
    std::fs::create_dir_all(&crates_dir).expect("mkdir");
    let dangling_target = tmp.path().join("nonexistent_target");
    symlink(&dangling_target, crates_dir.join("ghost_link")).expect("create dangling symlink");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);
    // The dangling symlink makes has_crates=true (list_dir is non-empty),
    // so this is treated as hex-in-hex. The recursion into the inner hex
    // may or may not produce rule6 errors depending on how the check handles
    // the phantom inner structure. Document whatever happens.
    // Key assertion: no panic, and the error (if any) is about phantom/, not garbage.
    let phantom_errors: Vec<_> = r6
        .iter()
        .filter(|e| e.title.contains("phantom"))
        .collect();
    // phantom/ has crates/ with a dangling symlink -> has_crates=true -> hex-in-hex path
    // The inner hex has no valid structure, but that's OTHER rules' problem.
    // Rule 06 should produce 0 errors for phantom itself (it has crates/).
    assert_eq!(
        r6.len(),
        phantom_errors.len(),
        "all rule6 errors should be about phantom, got r6={}: {r6:#?}",
        r6.len()
    );
    assert_no_ts_apps(&r6);
    assert_no_packages(&r6);
}

#[test]
fn maximally_complex_single_container() {
    // One container (devctl/crates/app) with multiple violation types:
    // - valid_crate: has Cargo.toml -> passes
    // - orphan_no_cargo: no Cargo.toml, no crates/ -> "missing Cargo.toml"
    // - conflict_both: Cargo.toml + crates/ -> "both Cargo.toml and crates/"
    // - hex_sub: crates/ with full hex structure -> passes (hex-in-hex)
    // - loose_file: plain file (not dir) -> ignored by list_dir
    // - link_valid: symlink to valid crate -> ignored (not a dir)
    // - gitkeep_only_sub: only .gitkeep -> "missing Cargo.toml" (CHECK BUG)
    let tmp = copy_fixture();

    // valid_crate: has Cargo.toml
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/valid_crate/Cargo.toml",
        "[package]\nname = \"valid\"\nversion = \"0.1.0\"\n",
    );

    // orphan_no_cargo: no Cargo.toml, no crates/
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/orphan_no_cargo/src/lib.rs",
        "",
    );

    // conflict_both: Cargo.toml + crates/
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/conflict_both/Cargo.toml",
        "[package]\nname = \"conflict\"\nversion = \"0.1.0\"\n",
    );
    for inner in &["domain", "app", "ports", "adapters"] {
        write_file(
            tmp.path(),
            &format!("apps/devctl/crates/app/conflict_both/crates/{inner}/.gitkeep"),
            "",
        );
    }

    // hex_sub: valid hex-in-hex
    for container in &["app", "domain", "adapters/inbound", "adapters/outbound", "ports/inbound", "ports/outbound"] {
        write_file(
            tmp.path(),
            &format!("apps/devctl/crates/app/hex_sub/crates/{container}/.gitkeep"),
            "",
        );
    }

    // loose_file: plain file, not a directory
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/loose_file",
        "I am a file",
    );

    // link_valid: symlink to valid crate (symlinks are not dirs)
    let target = tmp.path().join("apps/devctl/crates/app/core");
    let link = tmp.path().join("apps/devctl/crates/app/link_valid");
    symlink(&target, &link).expect("create symlink");

    // gitkeep_only_sub: only .gitkeep (CHECK BUG: fires missing Cargo.toml)
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/gitkeep_only_sub/.gitkeep",
        "",
    );

    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);

    // Expected violations:
    // 1. orphan_no_cargo: missing Cargo.toml
    // 2. conflict_both: both Cargo.toml and crates/
    // 3. gitkeep_only_sub: missing Cargo.toml (CHECK BUG)
    let orphan_errs: Vec<_> = r6.iter().filter(|e| e.title.contains("orphan_no_cargo")).collect();
    let conflict_errs: Vec<_> = r6.iter().filter(|e| e.title.contains("conflict_both")).collect();
    let gitkeep_errs: Vec<_> = r6.iter().filter(|e| e.title.contains("gitkeep_only_sub")).collect();
    let valid_errs: Vec<_> = r6.iter().filter(|e| e.title.contains("valid_crate")).collect();
    let hex_errs: Vec<_> = r6.iter().filter(|e| e.title.contains("hex_sub")).collect();
    let loose_errs: Vec<_> = r6.iter().filter(|e| e.title.contains("loose_file")).collect();
    let link_errs: Vec<_> = r6.iter().filter(|e| e.title.contains("link_valid")).collect();

    assert_eq!(orphan_errs.len(), 1, "orphan_no_cargo should fire: {orphan_errs:#?}");
    assert!(orphan_errs[0].title.contains("missing Cargo.toml"));
    assert_eq!(conflict_errs.len(), 1, "conflict_both should fire: {conflict_errs:#?}");
    assert!(conflict_errs[0].title.contains("both Cargo.toml and crates/"));
    assert_eq!(gitkeep_errs.len(), 1, "gitkeep_only_sub should fire (CHECK BUG): {gitkeep_errs:#?}");
    assert!(gitkeep_errs[0].title.contains("missing Cargo.toml"));
    assert!(valid_errs.is_empty(), "valid_crate should pass: {valid_errs:#?}");
    assert!(hex_errs.is_empty(), "hex_sub should pass: {hex_errs:#?}");
    assert!(loose_errs.is_empty(), "loose_file should be ignored: {loose_errs:#?}");
    assert!(link_errs.is_empty(), "link_valid should be ignored: {link_errs:#?}");

    assert_eq!(
        r6.len(),
        3,
        "expected exactly 3 total rule6 errors (orphan + conflict + gitkeep), got {}: {r6:#?}",
        r6.len()
    );
    assert_file_field(&r6);
    assert_no_ts_apps(&r6);
    assert_no_packages(&r6);
}

#[test]
fn unicode_lookalike_subdir() {
    // Create a subdir with a zero-width space in the name alongside a valid crate subdir.
    // The lookalike has no Cargo.toml -> fires "missing Cargo.toml".
    let tmp = copy_fixture();
    // Unicode zero-width space: U+200B
    let unicode_name = "zw\u{200B}s_crate";
    write_file(
        tmp.path(),
        &format!("apps/devctl/crates/app/{unicode_name}/src/lib.rs"),
        "",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);
    assert_eq!(
        r6.len(),
        1,
        "expected exactly 1 total rule6 error for unicode lookalike, got {}: {r6:#?}",
        r6.len()
    );
    assert!(
        r6[0].title.contains("missing Cargo.toml"),
        "expected 'missing Cargo.toml' for unicode subdir, got: '{}'",
        r6[0].title
    );
    assert_file_field(&r6);
    assert_no_ts_apps(&r6);
    assert_no_packages(&r6);
}

#[test]
fn cargo_toml_is_directory() {
    // Create a subdir where Cargo.toml is a directory, not a file.
    // read_file returns None (can't read a directory) -> "missing Cargo.toml".
    // Documents confusing-but-correct behavior: the name exists but it's not a file.
    let tmp = copy_fixture();
    // Create Cargo.toml as a directory (not a file)
    std::fs::create_dir_all(
        tmp.path().join("apps/devctl/crates/app/dir_cargo/Cargo.toml"),
    )
    .expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);
    let dir_cargo_errors: Vec<_> = r6
        .iter()
        .filter(|e| e.title.contains("dir_cargo"))
        .collect();
    assert_eq!(
        dir_cargo_errors.len(),
        1,
        "expected 1 error for Cargo.toml-as-directory (read_file returns None), got {}: {dir_cargo_errors:#?}",
        dir_cargo_errors.len()
    );
    assert!(
        dir_cargo_errors[0].title.contains("missing Cargo.toml"),
        "expected 'missing Cargo.toml' when Cargo.toml is a dir, got: '{}'",
        dir_cargo_errors[0].title
    );
    assert_eq!(
        r6.len(),
        1,
        "expected exactly 1 total rule6 error, got {}: {r6:#?}",
        r6.len()
    );
    assert_file_field(&r6);
    assert_no_ts_apps(&r6);
    assert_no_packages(&r6);
}

#[test]
fn subdirs_plus_loose_alongside_valid() {
    // Container has both valid crate subdirs AND an orphan. The orphan fires, valid ones don't.
    // Tests independent per-subdir checking — each subdir is evaluated on its own merits.
    let tmp = copy_fixture();
    // devctl/crates/app already has core/ (valid). Add an orphan alongside it.
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/loose_orphan/src/lib.rs",
        "",
    );
    // Also add another valid crate alongside both
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/another_valid/Cargo.toml",
        "[package]\nname = \"another-valid\"\nversion = \"0.1.0\"\n",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r6 = rule6_errors(&errors);

    // Only the orphan should fire
    let orphan_errs: Vec<_> = r6.iter().filter(|e| e.title.contains("loose_orphan")).collect();
    let valid_errs: Vec<_> = r6.iter().filter(|e| e.title.contains("another_valid")).collect();
    let core_errs: Vec<_> = r6.iter().filter(|e| e.title.contains("core")).collect();

    assert_eq!(
        orphan_errs.len(),
        1,
        "orphan should fire exactly once, got {}: {orphan_errs:#?}",
        orphan_errs.len()
    );
    assert!(
        orphan_errs[0].title.contains("missing Cargo.toml"),
        "orphan should be 'missing Cargo.toml', got: '{}'",
        orphan_errs[0].title
    );
    assert!(
        valid_errs.is_empty(),
        "another_valid should pass (has Cargo.toml), got: {valid_errs:#?}"
    );
    assert!(
        core_errs.is_empty(),
        "core should pass (has Cargo.toml), got: {core_errs:#?}"
    );
    assert_eq!(
        r6.len(),
        1,
        "expected exactly 1 total rule6 error (only the orphan), got {}: {r6:#?}",
        r6.len()
    );
    assert_file_field(&r6);
    assert_no_ts_apps(&r6);
    assert_no_packages(&r6);
}
