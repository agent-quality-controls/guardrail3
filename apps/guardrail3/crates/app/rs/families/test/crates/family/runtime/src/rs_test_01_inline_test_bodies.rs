use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::CfgTestModuleInput;

const ID: &str = "RS-TEST-01";

pub fn check(input: &CfgTestModuleInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.module.has_body {
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: "inline cfg(test) body in src".to_owned(),
        message:
            "Production `src/` files must not contain inline `#[cfg(test)] mod ... { ... }` bodies."
                .to_owned(),
        file: Some(input.file.rel_path.clone()),
        line: Some(input.module.line),
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_test_01_inline_test_bodies_tests/mod.rs"]
mod rs_test_01_inline_test_bodies_tests;
