use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::ManualDeserializeImplInput;

const ID: &str = "RS-GARDE-07";

pub fn check(input: &ManualDeserializeImplInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.target.needs_validate || input.target.has_validate {
        return;
    }

    results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Error,
    format!(
            "manual Deserialize impl for `{}` without Validate",
            input.target.type_name
        ),
    format!(
            "Manual `Deserialize` impl for `{}` bypasses derive-based garde checks and the type does not also implement `Validate`.",
            input.target.type_name
        ),
    Some(input.target.rel_path.clone()),
    Some(input.target.line),
    false,
    ));
}

