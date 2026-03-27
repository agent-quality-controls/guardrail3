use guardrail3_domain_report::{CheckResult, Severity};
#[cfg(test)]
use guardrail3_domain_project_tree::ProjectTree;

use super::facts::{ForbiddenConfigFacts, ForbiddenConfigReason};

const ID: &str = "RS-CLIPPY-12";

pub fn check(forbidden: &ForbiddenConfigFacts, results: &mut Vec<CheckResult>) {
    let (title, message) = match &forbidden.reason {
        ForbiddenConfigReason::NotAllowedRoot => (
            "clippy.toml in forbidden location".to_owned(),
            format!(
                "`{}` is not an allowed clippy policy root. clippy.toml is allowed only at the validation root, workspace roots, and standalone package roots that are not workspace members.",
                forbidden.config.rel_path
            ),
        ),
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

#[cfg(test)]
pub(crate) fn run_for_tests(tree: &ProjectTree) -> Vec<CheckResult> {
    let facts = super::facts::collect_for_tests(tree);
    let mut results = Vec::new();
    for forbidden in &facts.forbidden_configs {
        check(forbidden, &mut results);
    }
    results
}

#[cfg(test)]
#[path = "rs_clippy_12_allowed_placement_tests/mod.rs"]
mod rs_clippy_12_allowed_placement_tests;
