use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RustCodeFileInput;
use crate::parse::find_foreign_mod_allows;

const ID: &str = "RS-CODE-20";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_foreign_mod_allows(input.ast) {
        let lint = info.lint;
        let message = if info.via_cfg_attr {
            format!(
                "`#[cfg_attr(..., {}({lint}))]` on an `extern` block hides FFI risk behind a broad suppression.",
                info.kind.attr_name()
            )
        } else {
            format!(
                "`#[{}({lint})]` on an `extern` block hides FFI risk behind a broad suppression.",
                info.kind.attr_name()
            )
        };
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: if info.kind.attr_name() == "allow" {
                "allow on extern block".to_owned()
            } else {
                "expect on extern block".to_owned()
            },
            message,
            file: Some(input.rel_path.to_owned()),
            line: Some(info.line),
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
pub(crate) fn check_source(rel_path: &str, content: &str, is_test_root: bool) -> Vec<CheckResult> {
    let ast = crate::parse::parse_rust_file(content)
        .unwrap_or_else(|error| std::panic::panic_any(format!("valid rust: {error}")));
    let input = crate::inputs::RustCodeFileInput {
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

mod rs_code_20_extern_allow_tests;
