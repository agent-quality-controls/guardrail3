use crate::domain::report::{CheckResult, Severity};

use super::inputs::TestFileInput;

const ID: &str = "RS-TEST-09";

pub fn check(input: &TestFileInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.file.is_src_file || input.file.is_test_sidecar_file {
        return;
    }

    for module in &input.parsed.cfg_test_modules {
        if module.has_body {
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: "inline cfg(test) module in src file".to_owned(),
                message: "Production `src/` files must not contain inline `#[cfg(test)] mod ... { ... }` bodies.".to_owned(),
                file: Some(input.file.rel_path.clone()),
                line: Some(module.line),
                inventory: false,
            });
        }
    }

    for function in &input.parsed.test_functions {
        if function.inside_cfg_test_module {
            continue;
        }
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "test function in production source file".to_owned(),
            message: "Production `src/` files must not define direct `#[test]` functions.".to_owned(),
            file: Some(input.file.rel_path.clone()),
            line: Some(function.line),
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_test_09_no_inline_tests_in_src_tests.rs"]
mod tests;
