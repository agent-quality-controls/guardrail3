use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::UnsafeCodeLintInput;

const ID: &str = "RS-CODE-12";

pub fn check(input: &UnsafeCodeLintInput<'_>, results: &mut Vec<CheckResult>) {
    match input.lint_level {
        Some("forbid") => results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "unsafe_code = forbid".to_owned(),
                "unsafe_code is set to forbid in workspace lints.".to_owned(),
                Some(input.cargo_rel_path.to_owned()),
                None,
                false,
            )
            .as_inventory(),
        ),
        Some("deny") => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "unsafe_code should be forbid".to_owned(),
            "unsafe_code = deny can be overridden; use forbid in workspace lints.".to_owned(),
            Some(input.cargo_rel_path.to_owned()),
            None,
            false,
        )),
        _ => {}
    }
}

