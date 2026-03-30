#[cfg(test)]
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-25";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    match (&input.config.parsed, &input.config.parse_error) {
        (Some(_), None) => results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "clippy.toml parseable".to_owned(),
                message: format!("`{}` parsed successfully.", input.config.rel_path),
                file: Some(input.config.rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        ),
        (None, Some(parse_error)) => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "clippy.toml parse error".to_owned(),
            message: format!("Failed to parse `{}`: {parse_error}", input.config.rel_path),
            file: Some(input.config.rel_path.clone()),
            line: None,
            inventory: false,
        }),
        (None, None) => {}
        (Some(_), Some(_)) => {}
    }
}

#[cfg(test)]
pub(crate) fn run_for_tests(tree: &ProjectTree, rel_path: &str) -> Vec<CheckResult> {
    let facts = super::facts::collect_for_tests(tree);
    let mut results = Vec::new();
    check(
        &super::facts::config_input_for_tests(&facts, rel_path),
        &mut results,
    );
    results
}

#[cfg(test)]
#[path = "rs_clippy_25_config_parseable_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_clippy_25_config_parseable_tests;
