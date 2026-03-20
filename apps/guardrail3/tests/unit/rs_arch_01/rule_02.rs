use super::helpers::{arch_01_errors, assert_single_error, copy_golden, remove_dir, run_check, write_file};

// ---------------------------------------------------------------------------
// Group 1: missing individual required dirs
// ---------------------------------------------------------------------------

#[test]
fn missing_domain() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/devctl/crates/domain");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "missing crates/domain/");
}

#[test]
fn missing_app() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/devctl/crates/app");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "missing crates/app/");
}

#[test]
fn missing_ports() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/devctl/crates/ports");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "missing crates/ports/");
}

#[test]
fn missing_adapters() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/devctl/crates/adapters");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "missing crates/adapters/");
}

// ---------------------------------------------------------------------------
// Group 2: missing multiple required dirs
// ---------------------------------------------------------------------------

#[test]
fn missing_two_dirs() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/devctl/crates/domain");
    remove_dir(tmp.path(), "apps/devctl/crates/ports");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(errors.len(), 2, "expected 2 errors, got: {errors:#?}");
}

#[test]
fn missing_all_four() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/devctl/crates/adapters");
    remove_dir(tmp.path(), "apps/devctl/crates/app");
    remove_dir(tmp.path(), "apps/devctl/crates/domain");
    remove_dir(tmp.path(), "apps/devctl/crates/ports");
    // crates/ dir still exists, just empty
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(errors.len(), 4, "expected 4 errors (one per missing dir), got: {errors:#?}");
}

// ---------------------------------------------------------------------------
// Group 3: unexpected directories in crates/
// ---------------------------------------------------------------------------

#[test]
fn unexpected_dir_utils() {
    let tmp = copy_golden();
    write_file(tmp.path(), "apps/devctl/crates/utils/Cargo.toml", "[package]\nname = \"utils\"");
    write_file(tmp.path(), "apps/devctl/crates/utils/src/lib.rs", "");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "unexpected directory crates/utils/");
}

#[test]
fn unexpected_dir_lib() {
    let tmp = copy_golden();
    std::fs::create_dir_all(tmp.path().join("apps/devctl/crates/lib")).expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "unexpected directory crates/lib/");
}

#[test]
fn multiple_unexpected_dirs() {
    let tmp = copy_golden();
    std::fs::create_dir_all(tmp.path().join("apps/devctl/crates/utils")).expect("mkdir");
    std::fs::create_dir_all(tmp.path().join("apps/devctl/crates/shared")).expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(errors.len(), 2, "expected 2 errors (one per unexpected dir), got: {errors:#?}");
}

// ---------------------------------------------------------------------------
// Group 4: loose files in crates/ root
// ---------------------------------------------------------------------------

#[test]
fn loose_file_lib_rs() {
    let tmp = copy_golden();
    write_file(tmp.path(), "apps/devctl/crates/lib.rs", "// stray");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "loose files in crates/");
}

#[test]
fn loose_file_main_rs() {
    let tmp = copy_golden();
    write_file(tmp.path(), "apps/devctl/crates/main.rs", "fn main() {}");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "loose files in crates/");
}

#[test]
fn multiple_loose_files() {
    let tmp = copy_golden();
    write_file(tmp.path(), "apps/devctl/crates/lib.rs", "// stray");
    write_file(tmp.path(), "apps/devctl/crates/main.rs", "fn main() {}");
    write_file(tmp.path(), "apps/devctl/crates/fs.rs", "// stray");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    // All loose files reported in a single error
    assert_single_error(&errors, "loose files in crates/");
}

#[test]
fn gitkeep_allowed_in_crates_root() {
    let tmp = copy_golden();
    write_file(tmp.path(), "apps/devctl/crates/.gitkeep", "");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(errors.len(), 0, "expected 0 errors (golden + .gitkeep), got: {errors:#?}");
}

// ---------------------------------------------------------------------------
// Group 5: combinations
// ---------------------------------------------------------------------------

#[test]
fn missing_dir_and_unexpected_dir() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/devctl/crates/domain");
    std::fs::create_dir_all(tmp.path().join("apps/devctl/crates/utils")).expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(errors.len(), 2, "expected 2 errors (missing + unexpected), got: {errors:#?}");
}

#[test]
fn missing_dir_and_loose_file() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/devctl/crates/domain");
    write_file(tmp.path(), "apps/devctl/crates/lib.rs", "// stray");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(errors.len(), 2, "expected 2 errors (missing + loose), got: {errors:#?}");
}

// ---------------------------------------------------------------------------
// Group 6: cross-app
// ---------------------------------------------------------------------------

#[test]
fn missing_dir_in_one_unexpected_in_another() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/devctl/crates/domain");
    std::fs::create_dir_all(tmp.path().join("apps/worker/crates/utils")).expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(errors.len(), 2, "expected 2 errors (one per app), got: {errors:#?}");
}

// ---------------------------------------------------------------------------
// Group 7: inner hex-in-hex (backend/crates/adapters/inbound/mcp/crates/)
// ---------------------------------------------------------------------------

#[test]
fn inner_hex_missing_domain() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/backend/crates/adapters/inbound/mcp/crates/domain");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "domain");
}

#[test]
fn inner_hex_unexpected_dir() {
    let tmp = copy_golden();
    std::fs::create_dir_all(
        tmp.path().join("apps/backend/crates/adapters/inbound/mcp/crates/utils"),
    )
    .expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "unexpected");
}

#[test]
fn inner_hex_loose_file() {
    let tmp = copy_golden();
    write_file(
        tmp.path(),
        "apps/backend/crates/adapters/inbound/mcp/crates/mod.rs",
        "// stray",
    );
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "loose files");
}
