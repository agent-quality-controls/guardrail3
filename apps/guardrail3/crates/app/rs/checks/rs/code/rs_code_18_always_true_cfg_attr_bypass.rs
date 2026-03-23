use crate::domain::report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::find_always_true_cfg_attr_allows;

const ID: &str = "RS-CODE-18";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_always_true_cfg_attr_allows(input.ast) {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "always-true cfg_attr bypass".to_owned(),
            message: format!(
                "`#[cfg_attr(..., allow({}))]` is effectively unconditional. Use a direct `#[allow]` with an explicit reason instead.",
                info.lint
            ),
            file: Some(input.rel_path.to_owned()),
            line: Some(info.line),
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_code_18_always_true_cfg_attr_bypass_tests/mod.rs"]
mod tests;
