use super::helpers::{
    arch_01_errors, assert_single_error, copy_golden, remove_dir, run_check, write_file,
};

// ---------------------------------------------------------------------------
// Group 1: Missing Cargo.toml (no crates/ either) — invalid leaf subdir
// ---------------------------------------------------------------------------

#[test]
fn subdir_no_cargo_toml() {
    let tmp = copy_golden();
    std::fs::create_dir_all(tmp.path().join("apps/devctl/crates/app/orphan/src")).expect("mkdir"); // reason: test setup
    write_file(tmp.path(), "apps/devctl/crates/app/orphan/src/lib.rs", "");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "crates/app/orphan/ missing Cargo.toml");
}

#[test]
fn subdir_completely_empty() {
    let tmp = copy_golden();
    std::fs::create_dir_all(tmp.path().join("apps/devctl/crates/app/orphan")).expect("mkdir"); // reason: test setup
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    // An empty subdir inside a container has neither Cargo.toml nor crates/ — must error
    assert!(
        !errors.is_empty(),
        "expected error for empty subdir without Cargo.toml or crates/, got none"
    );
    assert!(
        errors
            .iter()
            .any(|e| e.title.contains("orphan") && e.title.contains("missing Cargo.toml")),
        "expected error about orphan missing Cargo.toml, got: {errors:#?}"
    );
}

// ---------------------------------------------------------------------------
// Group 2: Conflict — both Cargo.toml AND crates/ in same subdir
// ---------------------------------------------------------------------------

#[test]
fn subdir_has_both_cargo_and_crates() {
    let tmp = copy_golden();
    // Create a subdir that has BOTH Cargo.toml and a crates/ directory
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/hybrid/Cargo.toml",
        "[package]\nname = \"hybrid\"\nversion = \"0.1.0\"\n",
    );
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/hybrid/crates/domain/types/Cargo.toml",
        "[package]\nname = \"inner\"\nversion = \"0.1.0\"\n",
    );
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/hybrid/crates/domain/types/src/lib.rs",
        "",
    );
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        errors
            .iter()
            .any(|e| e.title.contains("hybrid") && e.title.contains("both")),
        "expected error about hybrid having both Cargo.toml and crates/, got: {errors:#?}"
    );
}

// ---------------------------------------------------------------------------
// Group 3: .gitkeep as placeholder for leaf subdir
// ---------------------------------------------------------------------------

#[test]
fn subdir_with_only_gitkeep() {
    let tmp = copy_golden();
    // A leaf subdir that has only .gitkeep should be treated as a valid placeholder
    // NOTE: If the current code does NOT support .gitkeep-only leaf subdirs,
    // this test will FAIL — that is intentional, proving the gap.
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/placeholder/.gitkeep",
        "",
    );
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let placeholder_errors: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("placeholder"))
        .collect();
    assert!(
        placeholder_errors.is_empty(),
        "a .gitkeep-only leaf subdir should be valid (placeholder), got errors: {placeholder_errors:#?}"
    );
}

// ---------------------------------------------------------------------------
// Group 4: .gitkeep alongside valid structures (harmless)
// ---------------------------------------------------------------------------

#[test]
fn gitkeep_alongside_cargo_toml() {
    let tmp = copy_golden();
    // domain/types already has Cargo.toml — adding .gitkeep should be harmless
    write_file(
        tmp.path(),
        "apps/devctl/crates/domain/types/.gitkeep",
        "",
    );
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        errors.is_empty(),
        "gitkeep alongside Cargo.toml should not cause errors, got: {errors:#?}"
    );
}

#[test]
fn gitkeep_alongside_hex_in_hex() {
    let tmp = copy_golden();
    // mcp/ already has crates/ — adding .gitkeep should be harmless
    write_file(
        tmp.path(),
        "apps/backend/crates/adapters/inbound/mcp/.gitkeep",
        "",
    );
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        errors.is_empty(),
        "gitkeep alongside hex-in-hex crates/ should not cause errors, got: {errors:#?}"
    );
}

// ---------------------------------------------------------------------------
// Group 5: Hex-in-hex valid (golden baseline)
// ---------------------------------------------------------------------------

#[test]
fn hex_in_hex_valid() {
    let tmp = copy_golden();
    // The golden backend/mcp is a valid hex-in-hex structure — must pass with 0 errors
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        errors.is_empty(),
        "golden hex-in-hex should pass with 0 errors, got: {errors:#?}"
    );
}

// ---------------------------------------------------------------------------
// Group 6: Hex-in-hex with broken inner structure
// ---------------------------------------------------------------------------

#[test]
fn hex_in_hex_inner_missing_domain() {
    let tmp = copy_golden();
    remove_dir(
        tmp.path(),
        "apps/backend/crates/adapters/inbound/mcp/crates/domain",
    );
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(
        &errors,
        "missing crates/adapters/inbound/mcp/crates/domain/",
    );
}

#[test]
fn hex_in_hex_inner_missing_app() {
    let tmp = copy_golden();
    remove_dir(
        tmp.path(),
        "apps/backend/crates/adapters/inbound/mcp/crates/app",
    );
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(
        &errors,
        "missing crates/adapters/inbound/mcp/crates/app/",
    );
}

// ---------------------------------------------------------------------------
// Group 7: Multiple invalid subdirs in same container
// ---------------------------------------------------------------------------

#[test]
fn multiple_invalid_in_same_container() {
    let tmp = copy_golden();
    // Two orphan subdirs without Cargo.toml in the same container (app/)
    std::fs::create_dir_all(tmp.path().join("apps/devctl/crates/app/orphan1")).expect("mkdir"); // reason: test setup
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/orphan1/src/lib.rs",
        "",
    );
    std::fs::create_dir_all(tmp.path().join("apps/devctl/crates/app/orphan2")).expect("mkdir"); // reason: test setup
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/orphan2/src/lib.rs",
        "",
    );
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let orphan_errors: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("orphan"))
        .collect();
    assert_eq!(
        orphan_errors.len(),
        2,
        "expected 2 errors (one per orphan), got: {orphan_errors:#?}"
    );
}

// ---------------------------------------------------------------------------
// Group 8: Cross-container invalid subdirs
// ---------------------------------------------------------------------------

#[test]
fn invalid_in_different_containers() {
    let tmp = copy_golden();
    // Orphan in app/ container
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/orphan_app/src/lib.rs",
        "",
    );
    // Orphan in domain/ container
    write_file(
        tmp.path(),
        "apps/devctl/crates/domain/orphan_dom/src/lib.rs",
        "",
    );
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let orphan_errors: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("orphan"))
        .collect();
    assert_eq!(
        orphan_errors.len(),
        2,
        "expected 2 errors (one per container), got: {orphan_errors:#?}"
    );
}

// ---------------------------------------------------------------------------
// Group 9: Inner hex-in-hex leaf issues
// ---------------------------------------------------------------------------

#[test]
fn inner_hex_leaf_no_cargo() {
    let tmp = copy_golden();
    // Create orphan inside the inner hex app/ container (backend mcp)
    write_file(
        tmp.path(),
        "apps/backend/crates/adapters/inbound/mcp/crates/app/orphan_inner/src/lib.rs",
        "",
    );
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let inner_orphan_errors: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("orphan_inner"))
        .collect();
    assert_eq!(
        inner_orphan_errors.len(),
        1,
        "expected 1 error for orphan inside inner hex-in-hex app/ container, got: {inner_orphan_errors:#?}"
    );
}
