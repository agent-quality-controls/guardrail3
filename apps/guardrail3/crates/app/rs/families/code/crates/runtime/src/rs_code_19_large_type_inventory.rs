use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::{find_large_type_items, LargeTypeItem as LargeTypeFact};

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
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "large type inventory".to_owned(),
                message: format!("{kind} has {count} items (inventory threshold {threshold})."),
                file: Some(input.rel_path.to_owned()),
                line: Some(line),
                inventory: false,
            }
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
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: "large type inventory".to_owned(),
            message: format!("struct `{name}` has {field_count} fields (inventory threshold 15)."),
            file: Some(input.rel_path.to_owned()),
            line: Some(line),
            inventory: false,
        }
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
pub(crate) fn check_source(rel_path: &str, content: &str, is_test: bool) -> Vec<CheckResult> {
    let ast = super::parse::parse_rust_file(content).unwrap_or_else(|error| std::panic::panic_any(format!("valid rust: {error}")));
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
#[path = "rs_code_19_large_type_inventory_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_code_19_large_type_inventory_tests;
