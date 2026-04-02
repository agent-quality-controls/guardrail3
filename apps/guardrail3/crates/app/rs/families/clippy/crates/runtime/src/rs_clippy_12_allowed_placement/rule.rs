use guardrail3_domain_report::{CheckResult, Severity};

use crate::facts::{ClippyConfigFacts, ForbiddenConfigFacts, ForbiddenConfigReason};

const ID: &str = "RS-CLIPPY-12";

pub fn check_allowed(config: &ClippyConfigFacts, results: &mut Vec<CheckResult>) {
    results.push(
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            "clippy.toml placement allowed".to_owned(),
            format!(
                "`{}` is placed at an allowed clippy policy root.",
                config.rel_path
            ),
            Some(config.rel_path.clone()),
            None,
            false,
        )
        .as_inventory(),
    );
}

pub fn check(forbidden: &ForbiddenConfigFacts, results: &mut Vec<CheckResult>) {
    let (title, message) = match &forbidden.reason {
        ForbiddenConfigReason::ShadowedSameRoot { preferred_rel_path } => (
            "same-root clippy config conflict".to_owned(),
            format!(
                "`{}` conflicts with `{preferred_rel_path}` at the same policy root. Keep only the highest-precedence clippy config file.",
                forbidden.config.rel_path
            ),
        ),
    };

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title,
        message,
        file: Some(forbidden.config.rel_path.clone()),
        line: None,
        inventory: false,
    });
}

