use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RustCodeFileInput;
use crate::parse::find_facade_body_items;

const ID: &str = "RS-CODE-27";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    if input.profile_name != Some("library") || !is_lib_rs(input.rel_path) {
        return;
    }

    for item in find_facade_body_items(input.ast) {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "lib.rs should stay facade-only".to_owned(),
            format!(
                "lib.rs contains {} `{}`. Keep lib.rs limited to facade declarations and type/const definitions.",
                item.kind, item.name
            ),
            Some(input.rel_path.to_owned()),
            Some(item.line),
            false,
        ));
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
pub(crate) fn check_source(rel_path: &str, content: &str, is_test_root: bool) -> Vec<CheckResult> {
    let ast = crate::parse::parse_rust_file(content)
        .unwrap_or_else(|error| std::panic::panic_any(format!("valid rust: {error}")));
    let input = crate::inputs::RustCodeFileInput {
        rel_path,
        content,
        ast: &ast,
        is_test_root,
        profile_name: Some("library"),
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]

mod rs_code_27_facade_only_lib_tests;
