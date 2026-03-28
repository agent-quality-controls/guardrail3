use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::find_facade_body_items;

const ID: &str = "RS-CODE-27";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    if input.profile_name != Some("library") || !is_lib_rs(input.rel_path) {
        return;
    }

    for item in find_facade_body_items(input.ast) {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "lib.rs should stay facade-only".to_owned(),
            message: format!(
                "lib.rs contains {} `{}`. Keep lib.rs limited to facade declarations and type/const definitions.",
                item.kind, item.name
            ),
            file: Some(input.rel_path.to_owned()),
            line: Some(item.line),
            inventory: false,
        });
    }
}

fn is_lib_rs(rel_path: &str) -> bool {
    rel_path
        .rsplit('/')
        .next()
        .is_some_and(|name| name == "lib.rs")
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
        profile_name: Some("library"),
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
#[path = "rs_code_27_facade_only_lib_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_code_27_facade_only_lib_tests;
