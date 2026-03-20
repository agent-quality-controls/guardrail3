use super::helpers::{
    arch_01_errors, assert_single_error, copy_golden, remove_dir, remove_file, run_check,
    write_file,
};

// -----------------------------------------------------------------------
// Group 1: Empty each container type
// -----------------------------------------------------------------------

#[test]
fn empty_ports_inbound() {
    let tmp = copy_golden();
    // ports/inbound/ has only .gitkeep — remove it to make the dir empty
    remove_file(tmp.path(), "apps/devctl/crates/ports/inbound/.gitkeep");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "empty container crates/ports/inbound/");
}

#[test]
fn empty_app() {
    let tmp = copy_golden();
    // app/ has core/ as its only subdir — remove it, leaving app/ empty
    remove_dir(tmp.path(), "apps/devctl/crates/app/core");
    // Ensure app/ dir itself still exists as an empty dir
    std::fs::create_dir_all(tmp.path().join("apps/devctl/crates/app")).expect("recreate app dir"); // reason: no-op if exists
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "empty container crates/app/");
}

#[test]
fn empty_domain() {
    let tmp = copy_golden();
    // domain/ has types/ as its only subdir — remove it
    remove_dir(tmp.path(), "apps/devctl/crates/domain/types");
    std::fs::create_dir_all(tmp.path().join("apps/devctl/crates/domain")).expect("recreate domain dir"); // reason: no-op if exists
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "empty container crates/domain/");
}

#[test]
fn empty_adapters_inbound() {
    let tmp = copy_golden();
    // adapters/inbound/ has cli/ as its only subdir — remove it
    remove_dir(tmp.path(), "apps/devctl/crates/adapters/inbound/cli");
    std::fs::create_dir_all(tmp.path().join("apps/devctl/crates/adapters/inbound")).expect("recreate dir"); // reason: no-op if exists
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(errors.len(), 1, "expected 1 error, got {}: {errors:#?}", errors.len());
    assert!(
        errors[0].title.contains("empty container"),
        "expected 'empty container' in title, got: '{}'",
        errors[0].title
    );
}

#[test]
fn empty_adapters_outbound() {
    let tmp = copy_golden();
    // adapters/outbound/ has fs/ as its only subdir — remove it
    remove_dir(tmp.path(), "apps/devctl/crates/adapters/outbound/fs");
    std::fs::create_dir_all(tmp.path().join("apps/devctl/crates/adapters/outbound")).expect("recreate dir"); // reason: no-op if exists
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(errors.len(), 1, "expected 1 error, got {}: {errors:#?}", errors.len());
    assert!(
        errors[0].title.contains("empty container"),
        "expected 'empty container' in title, got: '{}'",
        errors[0].title
    );
}

#[test]
fn empty_ports_outbound() {
    let tmp = copy_golden();
    // ports/outbound/ has traits/ as its only subdir — remove it
    remove_dir(tmp.path(), "apps/devctl/crates/ports/outbound/traits");
    std::fs::create_dir_all(tmp.path().join("apps/devctl/crates/ports/outbound")).expect("recreate dir"); // reason: no-op if exists
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(errors.len(), 1, "expected 1 error, got {}: {errors:#?}", errors.len());
    assert!(
        errors[0].title.contains("empty container"),
        "expected 'empty container' in title, got: '{}'",
        errors[0].title
    );
}

// -----------------------------------------------------------------------
// Group 2: Valid cases — no errors expected
// -----------------------------------------------------------------------

#[test]
fn gitkeep_is_valid_placeholder() {
    let tmp = copy_golden();
    // devctl ports/inbound has .gitkeep — should produce 0 errors
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        errors.is_empty(),
        "expected 0 errors for golden fixture, got: {errors:#?}"
    );
}

#[test]
fn subdirs_are_valid() {
    let tmp = copy_golden();
    // devctl app/ has core/ subdir — should produce 0 errors
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        errors.is_empty(),
        "expected 0 errors for golden fixture with valid subdirs, got: {errors:#?}"
    );
}

#[test]
fn gitkeep_alongside_subdirs() {
    let tmp = copy_golden();
    // Add .gitkeep to app/ which already has core/ subdir — still valid
    write_file(tmp.path(), "apps/devctl/crates/app/.gitkeep", "");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        errors.is_empty(),
        "expected 0 errors when .gitkeep coexists with subdirs, got: {errors:#?}"
    );
}

// -----------------------------------------------------------------------
// Group 3: Multiple empty containers
// -----------------------------------------------------------------------

#[test]
fn multiple_empty_containers() {
    let tmp = copy_golden();
    // Empty ports/inbound (remove .gitkeep)
    remove_file(tmp.path(), "apps/devctl/crates/ports/inbound/.gitkeep");
    // Empty ports/outbound (remove traits/ subdir)
    remove_dir(tmp.path(), "apps/devctl/crates/ports/outbound/traits");
    std::fs::create_dir_all(tmp.path().join("apps/devctl/crates/ports/outbound")).expect("recreate dir"); // reason: no-op if exists
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(
        errors.len(),
        2,
        "expected 2 errors (one per empty container), got {}: {errors:#?}",
        errors.len()
    );
}

// -----------------------------------------------------------------------
// Group 4: Cross-app — empty in one, valid in another
// -----------------------------------------------------------------------

#[test]
fn empty_in_one_app_valid_in_another() {
    let tmp = copy_golden();
    // Empty devctl ports/inbound (remove .gitkeep)
    remove_file(tmp.path(), "apps/devctl/crates/ports/inbound/.gitkeep");
    // Worker ports/inbound still has .gitkeep — should be fine
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(errors.len(), 1, "expected 1 error for devctl only, got {}: {errors:#?}", errors.len());
    assert!(
        errors[0].title.contains("devctl"),
        "expected error to mention devctl, got: '{}'",
        errors[0].title
    );
}

// -----------------------------------------------------------------------
// Group 5: Inner hex-in-hex empty containers
// -----------------------------------------------------------------------

#[test]
fn inner_hex_empty_container() {
    let tmp = copy_golden();
    // Remove .gitkeep from inner hex ports/inbound
    remove_file(
        tmp.path(),
        "apps/backend/crates/adapters/inbound/mcp/crates/ports/inbound/.gitkeep",
    );
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(errors.len(), 1, "expected 1 error, got {}: {errors:#?}", errors.len());
    assert!(
        errors[0].title.contains("empty container"),
        "expected 'empty container' in title, got: '{}'",
        errors[0].title
    );
}

#[test]
fn inner_hex_gitkeep_valid() {
    let tmp = copy_golden();
    // Inner hex has .gitkeep in ports/outbound — golden should pass
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        errors.is_empty(),
        "expected 0 errors for golden inner hex with .gitkeep, got: {errors:#?}"
    );
}
