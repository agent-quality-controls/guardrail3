use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::TestFunctionInput;

const ID: &str = "RS-TEST-10";

pub fn check(input: &TestFunctionInput<'_>, results: &mut Vec<CheckResult>) {
    let name = &input.function.name;
    if name.len() >= 10 && !has_numeric_suffix_pattern(name) {
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Warn,
        title: "test name is too weak".to_owned(),
        message: format!("Test function `{name}` is too short or uses a numeric suffix pattern."),
        file: Some(input.file.rel_path.clone()),
        line: Some(input.function.line),
        inventory: false,
    });
}

fn has_numeric_suffix_pattern(name: &str) -> bool {
    let Some((_, suffix)) = name.rsplit_once('_') else {
        return false;
    };
    !suffix.is_empty() && suffix.chars().all(|ch| ch.is_ascii_digit())
}

#[cfg(test)]
#[path = "rs_test_10_test_function_naming_tests.rs"]
mod tests;
