use std::path::Path;

use guardrail3::adapters::outbound::fs::RealFileSystem;
use guardrail3::app::rs::validate::arch::rs_arch_01::check_hex_arch_structure;
use guardrail3::domain::report::CheckResult;

// Re-export shared utilities so rule files import from super::helpers
pub use crate::test_support::{
    assert_file_field, assert_single_error, copy_golden, errors_by_id, remove_dir, remove_file,
    write_file,
};

const GOLDEN: &str = "tests/fixtures/r_arch_01/golden";

pub const RUST_APPS: &[&str] = &["devctl", "backend", "worker"];
pub const INNER_HEX: &str = "apps/backend/crates/adapters/inbound/mcp/crates";

pub fn copy_fixture() -> tempfile::TempDir {
    copy_golden(GOLDEN)
}

pub fn run_check(root: &Path) -> Vec<CheckResult> {
    let fs = RealFileSystem;
    let mut results = Vec::new();
    check_hex_arch_structure(&fs, root, &mut results);
    results
}

pub fn arch_errors(results: &[CheckResult]) -> Vec<&CheckResult> {
    errors_by_id(results, "R-ARCH-01")
}

pub fn assert_per_app(errors: &[&CheckResult]) {
    for app in RUST_APPS {
        assert!(
            errors.iter().any(|e| e.title.contains(app)),
            "expected error for app `{app}`, got: {errors:#?}"
        );
    }
}

pub fn assert_inner_hex(errors: &[&CheckResult]) {
    assert!(
        errors
            .iter()
            .any(|e| e.file.as_deref().unwrap_or("").contains("mcp/crates")),
        "expected at least one error from inner hex (mcp/crates), got: {errors:#?}"
    );
}

pub fn assert_no_ts_apps(errors: &[&CheckResult]) {
    assert!(
        !errors
            .iter()
            .any(|e| e.title.contains("admin") || e.title.contains("landing") || e.title.contains("portal")),
        "TS apps should not be flagged, got: {errors:#?}"
    );
}

pub fn assert_no_packages(errors: &[&CheckResult]) {
    assert!(
        !errors.iter().any(|e| {
            let t = &e.title;
            t.contains("shared-types") || t.contains("ui-kit")
        }),
        "packages should not be flagged, got: {errors:#?}"
    );
}
