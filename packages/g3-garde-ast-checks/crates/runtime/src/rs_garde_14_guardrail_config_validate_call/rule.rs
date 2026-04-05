use guardrail3_check_types::G3CheckResult;

use crate::support::{GuardrailConfigValidationSite, error};

const ID: &str = "RS-GARDE-14";

pub(crate) fn check(site: &GuardrailConfigValidationSite, results: &mut Vec<G3CheckResult>) {
    results.push(error(
        ID,
        "`GuardrailConfig` parse without garde validation",
        format!(
            "This {} call constructs `GuardrailConfig` but does not call `.validate()` on it. Call `.validate()` on the constructed `GuardrailConfig` before using it.",
            site.parse_kind.label()
        ),
        &site.rel_path,
        Some(site.line),
    ));
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
