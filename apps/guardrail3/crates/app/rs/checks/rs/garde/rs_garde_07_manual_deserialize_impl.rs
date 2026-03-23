use crate::domain::report::{CheckResult, Severity};

use super::inputs::ManualDeserializeImplInput;

const ID: &str = "RS-GARDE-07";

pub fn check(input: &ManualDeserializeImplInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.target.needs_validate || input.target.has_validate {
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: format!(
            "manual Deserialize impl for `{}` without Validate",
            input.target.type_name
        ),
        message: format!(
            "Manual `Deserialize` impl for `{}` bypasses derive-based garde checks and the type does not also implement `Validate`.",
            input.target.type_name
        ),
        file: Some(input.target.rel_path.clone()),
        line: Some(input.target.line),
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_garde_07_manual_deserialize_impl_tests.rs"]
mod tests;
