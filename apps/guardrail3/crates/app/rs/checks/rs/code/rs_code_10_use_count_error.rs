use crate::domain::report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::count_top_level_use_statements;

const ID: &str = "RS-CODE-10";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    if input.is_test {
        return;
    }

    let use_count = count_top_level_use_statements(input.ast);
    if use_count <= 20 {
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: "too many use statements".to_owned(),
        message: format!("{use_count} top-level use statements (max 20)."),
        file: Some(input.rel_path.to_owned()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_code_10_use_count_error_tests/mod.rs"]
mod tests;
