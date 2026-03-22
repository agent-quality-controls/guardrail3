use crate::domain::report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::{find_item_allows, same_line_reason};

const ID: &str = "RS-CODE-03";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for (line, lint) in find_item_allows(input.ast) {
        if same_line_reason(input.content, line).is_some() {
            continue;
        }
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "item-level allow without reason".to_owned(),
            message: format!("`#[allow({lint})]` requires `// reason:` on the same line."),
            file: Some(input.rel_path.to_owned()),
            line: Some(line),
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_code_03_item_level_allow_without_reason_tests.rs"]
mod tests;
