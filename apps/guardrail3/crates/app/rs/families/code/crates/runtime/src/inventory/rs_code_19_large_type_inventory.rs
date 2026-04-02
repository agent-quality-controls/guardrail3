use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RustCodeFileInput;
use crate::parse::{LargeTypeItem as LargeTypeFact, find_large_type_items};

const ID: &str = "RS-CODE-19";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for item in find_large_type_items(input.ast) {
        let (line, kind, count, threshold) = match item {
            LargeTypeFact::Struct {
                line,
                name,
                field_count,
            } => {
                push_struct_result(input, results, line, &name, field_count);
                continue;
            }
            LargeTypeFact::Enum {
                line,
                name,
                variant_count,
            } => (line, format!("enum `{name}`"), variant_count, 20),
        };

        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "large type inventory".to_owned(),
                format!("{kind} has {count} items (inventory threshold {threshold})."),
                Some(input.rel_path.to_owned()),
                Some(line),
                false,
            )
            .as_inventory(),
        );
    }
}

fn push_struct_result(
    input: &RustCodeFileInput<'_>,
    results: &mut Vec<CheckResult>,
    line: usize,
    name: &str,
    field_count: usize,
) {
    results.push(
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            "large type inventory".to_owned(),
            format!("struct `{name}` has {field_count} fields (inventory threshold 15)."),
            Some(input.rel_path.to_owned()),
            Some(line),
            false,
        )
        .as_inventory(),
    );
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
#[path = "rs_code_19_large_type_inventory_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_code_19_large_type_inventory_tests;
