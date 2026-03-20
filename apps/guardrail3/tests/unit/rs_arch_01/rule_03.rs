use super::helpers::{arch_01_errors, assert_single_error, copy_golden, remove_dir, run_check, write_file};

// ---------------------------------------------------------------------------
// Group 1: missing dirs in adapters/
// ---------------------------------------------------------------------------

#[test]
fn missing_inbound_in_adapters() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/devctl/crates/adapters/inbound");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "missing crates/adapters/inbound/");
}

#[test]
fn missing_outbound_in_adapters() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/devctl/crates/adapters/outbound");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "missing crates/adapters/outbound/");
}

#[test]
fn missing_both_in_adapters() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/devctl/crates/adapters/inbound");
    remove_dir(tmp.path(), "apps/devctl/crates/adapters/outbound");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(errors.len(), 2, "expected 2 errors (inbound + outbound), got: {errors:#?}");
}

// ---------------------------------------------------------------------------
// Group 2: missing dirs in ports/
// ---------------------------------------------------------------------------

#[test]
fn missing_inbound_in_ports() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/devctl/crates/ports/inbound");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "missing crates/ports/inbound/");
}

#[test]
fn missing_outbound_in_ports() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/devctl/crates/ports/outbound");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "missing crates/ports/outbound/");
}

#[test]
fn missing_both_in_ports() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/devctl/crates/ports/inbound");
    remove_dir(tmp.path(), "apps/devctl/crates/ports/outbound");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(errors.len(), 2, "expected 2 errors (inbound + outbound), got: {errors:#?}");
}

// ---------------------------------------------------------------------------
// Group 3: unexpected dirs
// ---------------------------------------------------------------------------

#[test]
fn unexpected_dir_in_adapters() {
    let tmp = copy_golden();
    std::fs::create_dir_all(tmp.path().join("apps/devctl/crates/adapters/shared")).expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "unexpected directory crates/adapters/shared/");
}

#[test]
fn unexpected_dir_in_ports() {
    let tmp = copy_golden();
    std::fs::create_dir_all(tmp.path().join("apps/devctl/crates/ports/common")).expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "unexpected directory crates/ports/common/");
}

#[test]
fn multiple_unexpected_in_adapters() {
    let tmp = copy_golden();
    std::fs::create_dir_all(tmp.path().join("apps/devctl/crates/adapters/shared")).expect("mkdir");
    std::fs::create_dir_all(tmp.path().join("apps/devctl/crates/adapters/misc")).expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(errors.len(), 2, "expected 2 errors (shared + misc), got: {errors:#?}");
}

// ---------------------------------------------------------------------------
// Group 4: loose files
// ---------------------------------------------------------------------------

#[test]
fn loose_file_in_adapters() {
    let tmp = copy_golden();
    write_file(tmp.path(), "apps/devctl/crates/adapters/mod.rs", "// stray");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "loose files in crates/adapters/");
}

#[test]
fn loose_file_in_ports() {
    let tmp = copy_golden();
    write_file(tmp.path(), "apps/devctl/crates/ports/mod.rs", "// stray");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "loose files in crates/ports/");
}

// ---------------------------------------------------------------------------
// Group 5: .gitkeep allowed
// ---------------------------------------------------------------------------

#[test]
fn gitkeep_allowed_in_adapters() {
    let tmp = copy_golden();
    write_file(tmp.path(), "apps/devctl/crates/adapters/.gitkeep", "");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(errors.len(), 0, "expected 0 errors (golden + .gitkeep in adapters/), got: {errors:#?}");
}

#[test]
fn gitkeep_allowed_in_ports() {
    let tmp = copy_golden();
    write_file(tmp.path(), "apps/devctl/crates/ports/.gitkeep", "");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(errors.len(), 0, "expected 0 errors (golden + .gitkeep in ports/), got: {errors:#?}");
}

// ---------------------------------------------------------------------------
// Group 6: cross-app
// ---------------------------------------------------------------------------

#[test]
fn missing_in_one_app_unexpected_in_another() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/devctl/crates/adapters/inbound");
    std::fs::create_dir_all(tmp.path().join("apps/worker/crates/ports/shared")).expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(errors.len(), 2, "expected 2 errors (one per app), got: {errors:#?}");
}

// ---------------------------------------------------------------------------
// Group 7: both adapters/ and ports/ broken in same app
// ---------------------------------------------------------------------------

#[test]
fn adapters_and_ports_both_broken() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/devctl/crates/adapters/inbound");
    remove_dir(tmp.path(), "apps/devctl/crates/ports/outbound");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(errors.len(), 2, "expected 2 errors (adapters + ports), got: {errors:#?}");
}

// ---------------------------------------------------------------------------
// Group 8: inner hex-in-hex
// ---------------------------------------------------------------------------

#[test]
fn inner_hex_missing_inbound() {
    let tmp = copy_golden();
    // Inner hex adapters/inbound/ contains transport/ crate — removing it removes the crate too
    remove_dir(
        tmp.path(),
        "apps/backend/crates/adapters/inbound/mcp/crates/adapters/inbound",
    );
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        errors.iter().any(|e| e.title.contains("inbound")),
        "expected error about missing inner adapters/inbound/, got: {errors:#?}"
    );
}

#[test]
fn inner_hex_unexpected_dir_in_ports() {
    let tmp = copy_golden();
    std::fs::create_dir_all(
        tmp.path().join("apps/backend/crates/adapters/inbound/mcp/crates/ports/shared"),
    )
    .expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        errors.iter().any(|e| e.title.contains("unexpected")),
        "expected error about unexpected dir in inner ports/, got: {errors:#?}"
    );
}
