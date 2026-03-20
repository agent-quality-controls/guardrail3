use super::helpers::{arch_errors, assert_single_error, copy_fixture, run_check, write_file};

// ---------------------------------------------------------------------------
// Group 1: Basic src/ ban
// ---------------------------------------------------------------------------

#[test]
fn src_with_rs_files() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/src/main.rs", "fn main() {}");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_single_error(&errors, "has src/ directory");
}

#[test]
fn src_with_non_rs_files() {
    let tmp = copy_fixture();
    // The directory itself is banned, not just .rs files inside it
    write_file(tmp.path(), "apps/devctl/src/README.md", "# readme");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_single_error(&errors, "has src/ directory");
}

#[test]
fn src_empty_dir() {
    let tmp = copy_fixture();
    // Create src/ as an empty directory — does the check detect it?
    // list_dir on an empty dir returns empty vec, so behavior depends on
    // whether the check tests for directory existence vs contents.
    std::fs::create_dir_all(tmp.path().join("apps/devctl/src")).expect("mkdir"); // reason: test setup
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let src_errors: Vec<_> = errors.iter().filter(|e| e.title.contains("src/")).collect();
    // An empty src/ directory should still be flagged — the dir itself is banned
    assert_eq!(
        src_errors.len(),
        1,
        "expected 1 error for empty src/ dir, got: {src_errors:#?}"
    );
}

// ---------------------------------------------------------------------------
// Group 2: Multiple apps
// ---------------------------------------------------------------------------

#[test]
fn src_in_multiple_apps() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/src/main.rs", "fn main() {}");
    write_file(tmp.path(), "apps/worker/src/main.rs", "fn main() {}");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let src_errors: Vec<_> = errors.iter().filter(|e| e.title.contains("src/")).collect();
    assert_eq!(
        src_errors.len(),
        2,
        "expected 2 src/ errors (one per app), got: {src_errors:#?}"
    );
}

#[test]
fn src_in_one_app_not_others() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/src/main.rs", "fn main() {}");
    // worker and backend should NOT get src/ errors
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let src_errors: Vec<_> = errors.iter().filter(|e| e.title.contains("src/")).collect();
    assert_eq!(
        src_errors.len(),
        1,
        "expected exactly 1 src/ error (devctl only), got: {src_errors:#?}"
    );
    assert!(
        src_errors[0].title.contains("devctl"),
        "expected error for devctl, got: {}",
        src_errors[0].title
    );
}

// ---------------------------------------------------------------------------
// Group 3: Interaction with crates/
// ---------------------------------------------------------------------------

#[test]
fn src_alongside_valid_crates() {
    let tmp = copy_fixture();
    // devctl already has valid crates/ — adding src/ should trigger exactly 1 error
    // for the src/ ban, not mixed with crates errors
    write_file(tmp.path(), "apps/devctl/src/main.rs", "fn main() {}");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_single_error(&errors, "has src/ directory");
}

// ---------------------------------------------------------------------------
// Group 4: TS apps NOT checked by R-ARCH-01
// ---------------------------------------------------------------------------

#[test]
fn ts_app_src_not_flagged() {
    let tmp = copy_fixture();
    // admin and landing are TS apps (they have package.json, no Cargo.toml)
    // They naturally have src/ directories for Next.js — should NOT trigger R-ARCH-01
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert!(
        !errors
            .iter()
            .any(|e| e.title.contains("admin") || e.title.contains("landing")),
        "TS apps should not trigger R-ARCH-01 src/ ban, got: {errors:#?}"
    );
}

// ---------------------------------------------------------------------------
// Group 5: Edge — src as a file, not a directory
// ---------------------------------------------------------------------------

#[test]
fn src_is_file_not_dir() {
    let tmp = copy_fixture();
    // Write "src" as a plain file, not a directory
    // list_dir on a file returns empty, so the check may or may not trigger
    write_file(tmp.path(), "apps/devctl/src", "not a directory");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let src_errors: Vec<_> = errors.iter().filter(|e| e.title.contains("src")).collect();
    // Document the actual behavior: if the check uses is_dir() or list_dir(),
    // a file named "src" should NOT trigger the src/ ban
    // (the ban is about src/ *directory*, not a file named src)
    assert!(
        src_errors.is_empty(),
        "a file named 'src' (not a directory) should not trigger src/ ban, got: {src_errors:#?}"
    );
}

// ---------------------------------------------------------------------------
// Group 6: Inner hex — src/ inside hex-in-hex leaf
// ---------------------------------------------------------------------------

#[test]
fn src_inside_hex_in_hex() {
    let tmp = copy_fixture();
    // The src/ ban is at app level (apps/{name}/src/), not inside hex-in-hex leaves.
    // mcp/ is a hex-in-hex at apps/backend/crates/adapters/inbound/mcp/ — writing
    // src/ directly inside mcp/ should NOT trigger the top-level src/ ban.
    // However, mcp/ has crates/ so adding src/ alongside crates/ might trigger
    // the "both Cargo.toml and crates/" check or be silently ignored.
    write_file(
        tmp.path(),
        "apps/backend/crates/adapters/inbound/mcp/src/main.rs",
        "fn main() {}",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    // The src/ ban should NOT fire (it only checks apps/{name}/src/, not deeper)
    let src_ban_errors: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("has src/ directory"))
        .collect();
    assert!(
        src_ban_errors.is_empty(),
        "src/ inside hex-in-hex should not trigger app-level src/ ban, got: {src_ban_errors:#?}"
    );
}
