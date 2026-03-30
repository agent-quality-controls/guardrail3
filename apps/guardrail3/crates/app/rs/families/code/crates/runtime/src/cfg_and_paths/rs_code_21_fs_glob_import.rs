use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::find_std_fs_glob_import_lines;

const ID: &str = "RS-CODE-21";

fn is_filesystem_boundary_module(rel_path: &str) -> bool {
    rel_path.ends_with("src/fs.rs")
        || rel_path.ends_with("src/fs/mod.rs")
        || rel_path.ends_with("fs/src/lib.rs")
}

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    if input.is_test_root || is_filesystem_boundary_module(input.rel_path) {
        return;
    }

    for line in find_std_fs_glob_import_lines(input.ast) {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "std::fs glob import".to_owned(),
            "Direct `use std::fs::*` glob import bypasses clippy method bans.".to_owned(),
            Some(input.rel_path.to_owned()),
            Some(line),
            false,
        ));
    }
}

#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
pub(crate) fn copy_fixture() -> test_support::TempDir {
    crate::copy_test_fixture()
}

#[cfg(test)]
pub(crate) fn check_source(rel_path: &str, content: &str, is_test_root: bool) -> Vec<CheckResult> {
    let ast = super::parse::parse_rust_file(content)
        .unwrap_or_else(|error| std::panic::panic_any(format!("valid rust: {error}")));
    let input = super::inputs::RustCodeFileInput {
        rel_path,
        content,
        ast: &ast,
        is_test_root,
        profile_name: None,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
#[path = "rs_code_21_fs_glob_import_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_code_21_fs_glob_import_tests;
