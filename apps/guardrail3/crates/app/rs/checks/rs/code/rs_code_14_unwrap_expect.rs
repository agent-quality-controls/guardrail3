use crate::domain::report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::{find_unwrap_expect, line_text};

const ID: &str = "RS-CODE-14";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for (line, method) in find_unwrap_expect(input.ast) {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: format!(".{method}() usage"),
            message: format!("`.{method}()` found: {}.", line_text(input.content, line)),
            file: Some(input.rel_path.to_owned()),
            line: Some(line),
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_code_14_unwrap_expect_tests/mod.rs"]
mod tests;
