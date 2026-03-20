use std::path::Path;

use guardrail3::adapters::outbound::fs::RealFileSystem;
use guardrail3::app::ts::validate::ts_arch_checks;
use guardrail3::domain::report::CheckResult;

// Re-export shared utilities so rule files import from super::helpers
pub use crate::test_support::{
    copy_golden, errors_by_id, remove_dir,
    write_file,
};

const GOLDEN: &str = "tests/fixtures/r_arch_01/golden";

pub fn copy_fixture() -> tempfile::TempDir {
    copy_golden(GOLDEN)
}

pub fn run_check(root: &Path) -> Vec<CheckResult> {
    let fs = RealFileSystem;
    ts_arch_checks::check_hex_arch_structure(&fs, root)
}

pub fn run_import_check(root: &Path) -> Vec<CheckResult> {
    let fs = RealFileSystem;
    ts_arch_checks::check_import_boundaries(&fs, root)
}

pub fn arch_errors(results: &[CheckResult]) -> Vec<&CheckResult> {
    errors_by_id(results, "T-ARCH-01")
}

pub fn import_errors(results: &[CheckResult]) -> Vec<&CheckResult> {
    errors_by_id(results, "T-ARCH-02")
}

/// Assert no errors mention Rust apps (devctl, backend, worker).
pub fn assert_no_rust_apps(errors: &[&CheckResult]) {
    assert!(
        !errors.iter().any(|e| {
            let t = &e.title;
            t.contains("devctl") || t.contains("backend") || t.contains("worker")
        }),
        "Rust apps should not be flagged by TS checks, got: {errors:#?}"
    );
}

/// Assert no errors mention packages (shared-types, ui-kit).
pub fn assert_no_packages(errors: &[&CheckResult]) {
    assert!(
        !errors.iter().any(|e| {
            let t = &e.title;
            t.contains("shared-types") || t.contains("ui-kit")
        }),
        "packages should not be flagged, got: {errors:#?}"
    );
}

/// Assert no errors mention the landing app (content site, should be skipped).
pub fn assert_no_landing(errors: &[&CheckResult]) {
    assert!(
        !errors.iter().any(|e| e.title.contains("landing")),
        "landing (content site) should not be flagged, got: {errors:#?}"
    );
}
