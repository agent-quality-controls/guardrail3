use crate::domain::report::{CheckResult, Severity};

use super::inputs::CodeInputFailureInput;

const ID: &str = "RS-CODE-30";

pub fn check(input: &CodeInputFailureInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: "code-family input failure".to_owned(),
        message: input.message.to_owned(),
        file: Some(input.rel_path.to_owned()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_code_30_input_failures_tests/mod.rs"]
mod tests;
