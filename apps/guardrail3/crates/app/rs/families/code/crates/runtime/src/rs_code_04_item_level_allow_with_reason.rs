use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::{find_item_allows, same_line_reason};

const ID: &str = "RS-CODE-04";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for (line, lint) in find_item_allows(input.ast) {
        let Some(reason) = same_line_reason(input.content, line) else {
            continue;
        };
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: "item-level allow with reason".to_owned(),
            message: format!("#[allow({lint})] reason: {reason}"),
            file: Some(input.rel_path.to_owned()),
            line: Some(line),
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_code_04_item_level_allow_with_reason_tests/mod.rs"]
mod tests;
