use std::collections::BTreeSet;

use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::{find_inline_std_fs_call_lines, find_std_fs_import_lines, line_text};

const ID: &str = "RS-CODE-15";

fn is_filesystem_boundary_module(rel_path: &str) -> bool {
    rel_path.ends_with("src/fs.rs")
        || rel_path.ends_with("src/fs/mod.rs")
        || rel_path.ends_with("fs/src/lib.rs")
}

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    if input.is_test || is_filesystem_boundary_module(input.rel_path) {
        return;
    }

    let mut import_lines: BTreeSet<usize> =
        find_std_fs_import_lines(input.ast).into_iter().collect();
    let call_lines: BTreeSet<usize> = find_inline_std_fs_call_lines(input.ast)
        .into_iter()
        .collect();

    for line in import_lines.iter().copied() {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "direct std::fs import".to_owned(),
            message: format!(
                "Direct `use std::fs` import found: `{}`.",
                line_text(input.content, line)
            ),
            file: Some(input.rel_path.to_owned()),
            line: Some(line),
            inventory: false,
        });
    }

    for line in call_lines {
        if import_lines.remove(&line) {
            continue;
        }
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "direct std::fs call".to_owned(),
            message: format!(
                "Direct `std::fs::*` call found: `{}`.",
                line_text(input.content, line)
            ),
            file: Some(input.rel_path.to_owned()),
            line: Some(line),
            inventory: false,
        });
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
pub(crate) fn check_source(rel_path: &str, content: &str, is_test: bool) -> Vec<CheckResult> {
    let ast = super::parse::parse_rust_file(content)
        .unwrap_or_else(|error| std::panic::panic_any(format!("valid rust: {error}")));
    let input = super::inputs::RustCodeFileInput {
        rel_path,
        content,
        ast: &ast,
        is_test,
        profile_name: None,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
#[path = "rs_code_15_direct_fs_usage_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_code_15_direct_fs_usage_tests;
