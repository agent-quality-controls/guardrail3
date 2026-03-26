use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ScopedArchConfigInput;

const ID: &str = "RS-ARCH-05";

pub fn check(input: &ScopedArchConfigInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: "Scoped `arch` config is forbidden".to_owned(),
        message: input.failure.message.clone(),
        file: Some(input.failure.rel_path.clone()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_arch_05_scoped_arch_config_forbidden_tests/mod.rs"]
mod tests;
