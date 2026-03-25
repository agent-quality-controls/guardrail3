use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::TestModuleInput;

const ID: &str = "RS-TEST-11";

pub fn check(input: &TestModuleInput<'_>, results: &mut Vec<CheckResult>) {
    if input.module.name == "tests" {
        return;
    }
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Warn,
        title: "cfg(test) module should be named tests".to_owned(),
        message: format!(
            "`#[cfg(test)] mod {}` should be named `tests`.",
            input.module.name
        ),
        file: Some(input.file.rel_path.clone()),
        line: Some(input.module.line),
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_test_11_cfg_test_module_naming_tests.rs"]
mod tests;
