use super::helpers::{arch_01_errors, assert_single_error, copy_golden, run_check, write_file};

// -----------------------------------------------------------------------
// Group 1: Loose file in each structural dir
// -----------------------------------------------------------------------

#[test]
fn loose_file_in_crates_root() {
    let tmp = copy_golden();
    write_file(tmp.path(), "apps/devctl/crates/lib.rs", "// stray");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "loose files in crates/");
}

#[test]
fn loose_file_in_adapters() {
    let tmp = copy_golden();
    write_file(tmp.path(), "apps/devctl/crates/adapters/mod.rs", "pub mod inbound;");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "loose files in crates/adapters/");
}

#[test]
fn loose_file_in_ports() {
    let tmp = copy_golden();
    write_file(tmp.path(), "apps/devctl/crates/ports/mod.rs", "pub mod inbound;");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "loose files in crates/ports/");
}

// -----------------------------------------------------------------------
// Group 2: Loose file in each container dir
// -----------------------------------------------------------------------

#[test]
fn loose_file_in_app() {
    let tmp = copy_golden();
    write_file(tmp.path(), "apps/devctl/crates/app/mod.rs", "pub mod core;");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "loose files in crates/app/");
}

#[test]
fn loose_file_in_domain() {
    let tmp = copy_golden();
    write_file(tmp.path(), "apps/devctl/crates/domain/mod.rs", "pub mod types;");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "loose files in crates/domain/");
}

#[test]
fn loose_file_in_adapters_inbound() {
    let tmp = copy_golden();
    write_file(
        tmp.path(),
        "apps/devctl/crates/adapters/inbound/mod.rs",
        "pub mod cli;",
    );
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(errors.len(), 1, "expected 1 error, got {}: {errors:#?}", errors.len());
    assert!(
        errors[0].title.contains("loose files") && errors[0].title.contains("adapters/inbound"),
        "expected title containing 'loose files' and 'adapters/inbound', got: '{}'",
        errors[0].title
    );
}

#[test]
fn loose_file_in_adapters_outbound() {
    let tmp = copy_golden();
    write_file(
        tmp.path(),
        "apps/devctl/crates/adapters/outbound/mod.rs",
        "pub mod fs;",
    );
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(errors.len(), 1, "expected 1 error, got {}: {errors:#?}", errors.len());
    assert!(
        errors[0].title.contains("loose files") && errors[0].title.contains("adapters/outbound"),
        "expected title containing 'loose files' and 'adapters/outbound', got: '{}'",
        errors[0].title
    );
}

#[test]
fn loose_file_in_ports_inbound() {
    let tmp = copy_golden();
    write_file(
        tmp.path(),
        "apps/devctl/crates/ports/inbound/stray.rs",
        "// stray file",
    );
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(errors.len(), 1, "expected 1 error, got {}: {errors:#?}", errors.len());
    assert!(
        errors[0].title.contains("loose files") && errors[0].title.contains("ports/inbound"),
        "expected title containing 'loose files' and 'ports/inbound', got: '{}'",
        errors[0].title
    );
}

#[test]
fn loose_file_in_ports_outbound() {
    let tmp = copy_golden();
    write_file(
        tmp.path(),
        "apps/devctl/crates/ports/outbound/stray.rs",
        "// stray file",
    );
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(errors.len(), 1, "expected 1 error, got {}: {errors:#?}", errors.len());
    assert!(
        errors[0].title.contains("loose files") && errors[0].title.contains("ports/outbound"),
        "expected title containing 'loose files' and 'ports/outbound', got: '{}'",
        errors[0].title
    );
}

// -----------------------------------------------------------------------
// Group 3: Multiple loose files in the same dir
// -----------------------------------------------------------------------

#[test]
fn multiple_loose_files_same_dir() {
    let tmp = copy_golden();
    write_file(tmp.path(), "apps/devctl/crates/lib.rs", "// stray 1");
    write_file(tmp.path(), "apps/devctl/crates/main.rs", "// stray 2");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    // Both files reported in a single error for crates/
    assert_single_error(&errors, "loose files in crates/");
}

// -----------------------------------------------------------------------
// Group 4: Non-.rs loose files
// -----------------------------------------------------------------------

#[test]
fn loose_readme_in_domain() {
    let tmp = copy_golden();
    write_file(tmp.path(), "apps/devctl/crates/domain/README.md", "# Domain");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "loose files in crates/domain/");
}

#[test]
fn loose_env_in_adapters() {
    let tmp = copy_golden();
    write_file(tmp.path(), "apps/devctl/crates/adapters/.env", "SECRET=123");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "loose files in crates/adapters/");
}

// -----------------------------------------------------------------------
// Group 5: .gitkeep is allowed (not flagged as loose)
// -----------------------------------------------------------------------

#[test]
fn gitkeep_not_flagged() {
    let tmp = copy_golden();
    // Write .gitkeep into adapters/ — should NOT trigger an error
    write_file(tmp.path(), "apps/devctl/crates/adapters/.gitkeep", "");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        errors.is_empty(),
        "expected 0 errors for golden with .gitkeep, got: {errors:#?}"
    );
}

// -----------------------------------------------------------------------
// Group 6: .gitkeep alongside loose files
// -----------------------------------------------------------------------

#[test]
fn gitkeep_alongside_loose_files() {
    let tmp = copy_golden();
    write_file(tmp.path(), "apps/devctl/crates/adapters/.gitkeep", "");
    write_file(tmp.path(), "apps/devctl/crates/adapters/mod.rs", "pub mod inbound;");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    // .gitkeep is allowed, but mod.rs is still a violation
    assert_single_error(&errors, "loose files in crates/adapters/");
}

// -----------------------------------------------------------------------
// Group 7: Inner hex-in-hex loose files
// -----------------------------------------------------------------------

#[test]
fn inner_hex_loose_file_in_structural() {
    let tmp = copy_golden();
    write_file(
        tmp.path(),
        "apps/backend/crates/adapters/inbound/mcp/crates/adapters/mod.rs",
        "pub mod inbound;",
    );
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(errors.len(), 1, "expected 1 error, got {}: {errors:#?}", errors.len());
    assert!(
        errors[0].title.contains("loose files") && errors[0].title.contains("adapters"),
        "expected error about loose files in inner hex adapters/, got: '{}'",
        errors[0].title
    );
}

#[test]
fn inner_hex_loose_file_in_container() {
    let tmp = copy_golden();
    write_file(
        tmp.path(),
        "apps/backend/crates/adapters/inbound/mcp/crates/domain/mod.rs",
        "pub mod protocol;",
    );
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(errors.len(), 1, "expected 1 error, got {}: {errors:#?}", errors.len());
    assert!(
        errors[0].title.contains("loose files") && errors[0].title.contains("domain"),
        "expected error about loose files in inner hex domain/, got: '{}'",
        errors[0].title
    );
}

// -----------------------------------------------------------------------
// Group 8: Loose files in multiple dirs (same app)
// -----------------------------------------------------------------------

#[test]
fn loose_files_in_multiple_dirs() {
    let tmp = copy_golden();
    write_file(tmp.path(), "apps/devctl/crates/adapters/mod.rs", "pub mod inbound;");
    write_file(tmp.path(), "apps/devctl/crates/ports/mod.rs", "pub mod inbound;");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(errors.len(), 2, "expected 2 errors (one per dir), got {}: {errors:#?}", errors.len());
}

// -----------------------------------------------------------------------
// Group 9: Cross-app loose files
// -----------------------------------------------------------------------

#[test]
fn loose_files_across_apps() {
    let tmp = copy_golden();
    write_file(tmp.path(), "apps/devctl/crates/domain/stray.rs", "// stray");
    write_file(tmp.path(), "apps/worker/crates/domain/stray.rs", "// stray");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(errors.len(), 2, "expected 2 errors (one per app), got {}: {errors:#?}", errors.len());
}
