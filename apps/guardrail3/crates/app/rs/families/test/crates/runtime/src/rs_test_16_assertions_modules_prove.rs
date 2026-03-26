use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::AssertionsModuleInput;

const ID: &str = "RS-TEST-16";

pub fn check(input: &AssertionsModuleInput<'_>, results: &mut Vec<CheckResult>) {
    let first_exported_function = input
        .parsed
        .functions
        .iter()
        .find(|function| function.is_public && !function.is_test);
    let Some(first_exported_function) = first_exported_function else {
        return;
    };
    if !input.proof_bearing_exported_functions.is_empty() {
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: "assertions module lacks proof-bearing export".to_owned(),
        message: "Assertions modules that expose helper functions must contain at least one public function with a real assertion or a call into another proof-bearing owned assertions function.".to_owned(),
        file: Some(input.file.rel_path.clone()),
        line: Some(first_exported_function.line),
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_test_16_assertions_modules_prove_tests/mod.rs"]
mod rs_test_16_assertions_modules_prove_tests;
