use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::CodeInputFailureInput;

const ID: &str = "RS-CODE-30";

pub fn check(input: &CodeInputFailureInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "code-family input failure".to_owned(),
        input.message.to_owned(),
        Some(input.rel_path.to_owned()),
        None,
        false,
    ));
}

