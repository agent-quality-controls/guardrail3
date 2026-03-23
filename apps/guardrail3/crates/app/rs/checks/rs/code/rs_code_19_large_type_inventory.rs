use crate::domain::report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::find_large_type_items;

const ID: &str = "RS-CODE-19";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for item in find_large_type_items(input.ast) {
        let (line, kind, count, threshold) = match item {
            super::parse::LargeTypeItem::Struct {
                line,
                name,
                field_count,
            } => {
                push_struct_result(input, results, line, &name, field_count);
                continue;
            }
            super::parse::LargeTypeItem::Enum {
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
#[path = "rs_code_19_large_type_inventory_tests/mod.rs"]
mod tests;
