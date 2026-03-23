use crate::domain::report::{CheckResult, Severity};

use super::inputs::QueryAsMacroInput;

const ID: &str = "RS-GARDE-09";

pub fn check(input: &QueryAsMacroInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: "sqlx query_as requires validation review".to_owned(),
            message: format!(
                "`{}` bypasses derive-based garde boundary checks. Review the target type and ensure validated input handling is explicit.",
                input.macro_use.macro_name
            ),
            file: Some(input.macro_use.rel_path.clone()),
            line: Some(input.macro_use.line),
            inventory: false,
        }
        .as_inventory(),
    );
}

#[cfg(test)]
#[path = "rs_garde_09_query_as_inventory_tests.rs"]
mod tests;
