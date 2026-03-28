#[cfg(test)]
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::{CheckResult, Severity};

use super::clippy_support::parse_ban_entries;
use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-08";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.config.parsed.as_ref() else {
        return;
    };

    for key in [
        "disallowed-methods",
        "disallowed-types",
        "disallowed-macros",
    ] {
        for entry in parse_ban_entries(parsed, key) {
            if entry.is_plain_string || entry.reason.as_deref().is_none() {
                results.push(CheckResult {
                    id: ID.to_owned(),
                    severity: Severity::Warn,
                    title: "ban entry missing reason".to_owned(),
                    message: format!(
                        "`{}` in `{key}` must use table format with a `reason` field.",
                        entry.path
                    ),
                    file: Some(input.config.rel_path.clone()),
                    line: None,
                    inventory: false,
                });
            }
        }
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
#[path = "rs_clippy_08_reason_quality_tests/mod.rs"]
mod rs_clippy_08_reason_quality_tests;
