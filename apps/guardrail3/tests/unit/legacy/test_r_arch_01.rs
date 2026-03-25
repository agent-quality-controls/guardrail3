#![allow(clippy::expect_used, unused_results)] // reason: test assertions and fs::copy return values

//! Fixture-based tests for R-ARCH-01 hex arch structural enforcement.
//!
//! All tests copy the golden fixture into a temp dir, mutate it, run the check,
//! and assert exactly which error fires (count + title).
//!
//! Golden fixture apps:
//!   devctl   — Rust CLI, simple hex arch
//!   backend  — Rust server, REST + MCP (hex-in-hex on MCP adapter)
//!   worker   — Rust async worker, simple hex arch
//!   admin    — Next.js, no Cargo.toml (skipped by R-ARCH-01)
//!   landing  — Next.js, no Cargo.toml (skipped by R-ARCH-01)
//!
//! Rules tested:
//!  1. `crates/` must exist
//!  2. `crates/` must contain exactly `{adapters, app, domain, ports}` — no other files or dirs
//!  3. `adapters/` and `ports/` must contain exactly `{inbound, outbound}` — no other files or dirs
//!  4. Only `.gitkeep` allowed as a file in any structural or container dir
//!  5. Container dirs must have `.gitkeep` or at least one subdir
//!  6. Each container subdir must be a crate (Cargo.toml) or hex-in-hex (crates/ dir)
//!  7. Each crate subdir must be a member of the app's workspace (NOT YET IMPLEMENTED)
//!  8. `apps/{name}/Cargo.toml` must be a `[workspace]` (NOT YET IMPLEMENTED)
//!  9. Workspace members must match crate subdirs exactly (NOT YET IMPLEMENTED)
//! 10. Workspace members must not point outside app dir (NOT YET IMPLEMENTED)
//! 11. Root workspace must not include apps as members (NOT YET IMPLEMENTED)
//! 12. `apps/{name}/src/` is banned

use std::path::Path;

use guardrail3_adapters_outbound_fs::RealFileSystem;
use guardrail3_app_rs_legacy_validate::hex_arch_structure::check_hex_arch_structure;
use guardrail3::domain::report::{CheckResult, Severity};

const GOLDEN: &str = "tests/fixtures/r_arch_01/golden";

fn copy_golden() -> tempfile::TempDir {
    let tmp = tempfile::tempdir().expect("create tempdir");
    copy_dir_recursive(Path::new(GOLDEN), tmp.path());
    tmp
}

fn copy_dir_recursive(src: &Path, dst: &Path) {
    for entry in std::fs::read_dir(src).expect("read golden dir") {
        let entry = entry.expect("read entry");
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            std::fs::create_dir_all(&dst_path).expect("create dir");
            copy_dir_recursive(&src_path, &dst_path);
        } else {
            std::fs::copy(&src_path, &dst_path).expect("copy file");
        }
    }
}

fn run_check(root: &Path) -> Vec<CheckResult> {
    let fs = RealFileSystem;
    let mut results = Vec::new();
    check_hex_arch_structure(&fs, root, &mut results);
    results
}

fn arch_01_errors(results: &[CheckResult]) -> Vec<&CheckResult> {
    results
        .iter()
        .filter(|r| r.id == "R-ARCH-01" && r.severity == Severity::Error)
        .collect()
}

fn remove_dir(root: &Path, rel: &str) {
    std::fs::remove_dir_all(root.join(rel)).expect("remove dir");
}

fn remove_file(root: &Path, rel: &str) {
    std::fs::remove_file(root.join(rel)).expect("remove file");
}

fn write_file(root: &Path, rel: &str, content: &str) {
    let path = root.join(rel);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create parent");
    }
    std::fs::write(&path, content).expect("write file");
}

fn assert_single_error(errors: &[&CheckResult], expected_title_fragment: &str) {
    assert_eq!(errors.len(), 1, "expected exactly 1 error, got {}: {errors:#?}", errors.len());
    assert!(
        errors[0].title.contains(expected_title_fragment),
        "expected title containing '{expected_title_fragment}', got: '{}'",
        errors[0].title
    );
}

// =======================================================================
// Golden: must produce 0 R-ARCH-01 errors
// =======================================================================

#[test]
fn golden_passes() {
    let tmp = copy_golden();
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(errors.is_empty(), "golden should have 0 R-ARCH-01 errors, got: {errors:#?}");
}

// =======================================================================
// Rule 1: crates/ must exist
// =======================================================================

#[test]
fn rule_01_missing_crates_dir() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/devctl/crates");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "missing crates/");
}

// =======================================================================
// Rule 2: crates/ contents must be exactly {adapters, app, domain, ports}
// =======================================================================

#[test]
fn rule_02_missing_required_dir() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/devctl/crates/ports");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "missing crates/ports/");
}

#[test]
fn rule_02_unexpected_dir_in_crates() {
    let tmp = copy_golden();
    write_file(tmp.path(), "apps/devctl/crates/utils/Cargo.toml", "[package]\nname = \"utils\"");
    write_file(tmp.path(), "apps/devctl/crates/utils/src/lib.rs", "");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "unexpected directory crates/utils/");
}

#[test]
fn rule_02_loose_file_in_crates() {
    let tmp = copy_golden();
    write_file(tmp.path(), "apps/devctl/crates/lib.rs", "// stray");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "loose files in crates/");
}

// =======================================================================
// Rule 3: adapters/ and ports/ must contain exactly {inbound, outbound}
// =======================================================================

#[test]
fn rule_03_missing_outbound_in_adapters() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/devctl/crates/adapters/outbound");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "missing crates/adapters/outbound/");
}

#[test]
fn rule_03_unexpected_dir_in_ports() {
    let tmp = copy_golden();
    std::fs::create_dir_all(tmp.path().join("apps/devctl/crates/ports/shared")).expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "unexpected directory crates/ports/shared/");
}

// =======================================================================
// Rule 4: only .gitkeep allowed as file in structural/container dirs
// =======================================================================

#[test]
fn rule_04_loose_file_in_structural_dir() {
    let tmp = copy_golden();
    write_file(tmp.path(), "apps/devctl/crates/adapters/mod.rs", "pub mod inbound;");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "loose files in crates/adapters/");
}

#[test]
fn rule_04_loose_file_in_container_dir() {
    let tmp = copy_golden();
    write_file(tmp.path(), "apps/devctl/crates/domain/mod.rs", "pub mod types;");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "loose files in crates/domain/");
}

// =======================================================================
// Rule 5: container dirs must have .gitkeep or at least one subdir
// =======================================================================

#[test]
fn rule_05_empty_container_no_gitkeep() {
    let tmp = copy_golden();
    remove_file(tmp.path(), "apps/devctl/crates/ports/inbound/.gitkeep");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "empty container crates/ports/inbound/");
}

// =======================================================================
// Rule 6: each container subdir must be a crate or hex-in-hex
// =======================================================================

#[test]
fn rule_06_subdir_missing_cargo_toml() {
    let tmp = copy_golden();
    std::fs::create_dir_all(tmp.path().join("apps/devctl/crates/app/orphan/src")).expect("mkdir");
    write_file(tmp.path(), "apps/devctl/crates/app/orphan/src/lib.rs", "");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "crates/app/orphan/ missing Cargo.toml");
}

#[test]
fn rule_06_hex_in_hex_broken_inner() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/backend/crates/adapters/inbound/mcp/crates/domain");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(
        &errors,
        "missing crates/adapters/inbound/mcp/crates/domain/",
    );
}

// =======================================================================
// Rule 7: each crate subdir must be a member of the app's workspace
// (NOT YET IMPLEMENTED — these tests MUST fail)
// =======================================================================

#[test]
fn rule_07_crate_not_in_workspace_members() {
    let tmp = copy_golden();
    // Add a valid crate that is NOT listed in the workspace members
    write_file(
        tmp.path(),
        "apps/devctl/crates/domain/events/Cargo.toml",
        "[package]\nname = \"devctl-domain-events\"\nversion = \"0.1.0\"\nedition = \"2024\"",
    );
    write_file(tmp.path(), "apps/devctl/crates/domain/events/src/lib.rs", "// events");
    // devctl's Cargo.toml workspace members does NOT include crates/domain/events
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        errors.iter().any(|e| e.title.contains("not a workspace member") || e.title.contains("not in workspace")),
        "expected error about crate not being a workspace member, got: {errors:#?}"
    );
}

// =======================================================================
// Rule 8: apps/{name}/Cargo.toml must be a [workspace]
// (NOT YET IMPLEMENTED — these tests MUST fail)
// =======================================================================

#[test]
fn rule_08_app_cargo_toml_not_workspace() {
    let tmp = copy_golden();
    // Replace devctl's workspace Cargo.toml with a plain [package]
    write_file(
        tmp.path(),
        "apps/devctl/Cargo.toml",
        "[package]\nname = \"devctl\"\nversion = \"0.1.0\"\nedition = \"2024\"",
    );
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        errors.iter().any(|e| e.title.contains("not a workspace") || e.title.contains("must be a workspace")),
        "expected error about app Cargo.toml not being a workspace, got: {errors:#?}"
    );
}

// =======================================================================
// Rule 9: workspace members must match crate subdirs exactly
// (NOT YET IMPLEMENTED — these tests MUST fail)
// =======================================================================

#[test]
fn rule_09_workspace_has_extra_member() {
    let tmp = copy_golden();
    // Add a nonexistent member to devctl's workspace
    write_file(
        tmp.path(),
        "apps/devctl/Cargo.toml",
        "[workspace]\nmembers = [\n    \"crates/domain/types\",\n    \"crates/app/core\",\n    \"crates/ports/outbound/traits\",\n    \"crates/adapters/inbound/cli\",\n    \"crates/adapters/outbound/fs\",\n    \"crates/domain/phantom\",\n]\nresolver = \"2\"",
    );
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        errors.iter().any(|e| e.title.contains("phantom") || e.title.contains("extra member") || e.title.contains("does not exist")),
        "expected error about phantom workspace member, got: {errors:#?}"
    );
}

// =======================================================================
// Rule 10: workspace members must not point outside app dir
// (NOT YET IMPLEMENTED — these tests MUST fail)
// =======================================================================

#[test]
fn rule_10_workspace_member_outside_app() {
    let tmp = copy_golden();
    // Add a member pointing outside devctl's directory
    write_file(
        tmp.path(),
        "apps/devctl/Cargo.toml",
        "[workspace]\nmembers = [\n    \"crates/domain/types\",\n    \"crates/app/core\",\n    \"crates/ports/outbound/traits\",\n    \"crates/adapters/inbound/cli\",\n    \"crates/adapters/outbound/fs\",\n    \"../../packages/shared-types\",\n]\nresolver = \"2\"",
    );
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        errors.iter().any(|e| e.title.contains("outside") || e.title.contains("shared-types")),
        "expected error about workspace member pointing outside app dir, got: {errors:#?}"
    );
}

// =======================================================================
// Rule 11: root workspace must not include apps as members
// (NOT YET IMPLEMENTED — these tests MUST fail)
// =======================================================================

#[test]
fn rule_11_root_workspace_includes_app() {
    let tmp = copy_golden();
    // Root Cargo.toml lists an app as a workspace member
    write_file(
        tmp.path(),
        "Cargo.toml",
        "[workspace]\nmembers = [\"packages/shared-types\", \"apps/devctl\"]\nresolver = \"2\"",
    );
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        errors.iter().any(|e| e.title.contains("root workspace") || e.title.contains("apps/devctl")),
        "expected error about root workspace including app, got: {errors:#?}"
    );
}

// =======================================================================
// Rule 12: apps/{name}/src/ is banned
// =======================================================================

#[test]
fn rule_12_src_dir_banned() {
    let tmp = copy_golden();
    write_file(tmp.path(), "apps/devctl/src/main.rs", "fn main() {}");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "has src/ directory");
}
