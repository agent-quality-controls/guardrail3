use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::GuardrailConfigValidationInput;

const ID: &str = "RS-GARDE-14";

pub fn check(input: &GuardrailConfigValidationInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "`GuardrailConfig` parse without garde validation".to_owned(),
        format!(
            "This {} call constructs `GuardrailConfig` but does not call `.validate()` on it. Call `.validate()` on the constructed `GuardrailConfig` before using it.",
            input.site.parse_kind.label()
        ),
        Some(input.site.rel_path.clone()),
        Some(input.site.line),
        false,
    ));
}

