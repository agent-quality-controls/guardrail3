use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::GuardrailConfigValidationInput;

const ID: &str = "RS-GARDE-14";

pub fn check(input: &GuardrailConfigValidationInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "`GuardrailConfig` parse without garde validation".to_owned(),
        format!(
            "This {} site constructs `GuardrailConfig`, but the same function does not prove garde validation with `.validate()` before the config is used.",
            input.site.parse_kind.label()
        ),
        Some(input.site.rel_path.clone()),
        Some(input.site.line),
        false,
    ));
}

