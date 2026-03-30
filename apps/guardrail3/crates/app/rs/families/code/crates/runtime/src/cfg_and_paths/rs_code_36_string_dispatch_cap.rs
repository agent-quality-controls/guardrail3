use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::find_string_dispatch_sites;

const ID: &str = "RS-CODE-36";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_string_dispatch_sites(input.ast, input.is_test_root) {
        results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Error,
    "string dispatch is too large".to_owned(),
    format!(
                "{} site has {} string-literal branches (cap 10). Replace string dispatch with typed models.",
                info.site_kind, info.string_literal_branch_count
            ),
    Some(input.rel_path.to_owned()),
    Some(info.line),
    false,
        ));
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
#[path = "rs_code_36_string_dispatch_cap_tests/mod.rs"]
mod rs_code_36_string_dispatch_cap_tests;
