#[cfg(test)]
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::{CheckResult, Severity};

use super::clippy_support::{is_placeholder_reason, parse_ban_section};
use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-15";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.config.parsed.as_ref() else {
        return;
    };

    let mut issue_count = 0usize;

    for key in [
        "disallowed-methods",
        "disallowed-types",
        "disallowed-macros",
    ] {
        let section = parse_ban_section(parsed, key);
        for malformed in &section.malformed_messages {
            issue_count += 1;
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Warn,
                "ban section malformed".to_owned(),
                malformed.clone(),
                Some(input.config.rel_path.clone()),
                None,
                false,
            ));
        }
        for entry in section.entries {
            if let Some(reason) = entry.reason.as_deref()
                && is_placeholder_reason(reason)
            {
                issue_count += 1;
                results.push(CheckResult::from_parts(
                    ID.to_owned(),
                    Severity::Warn,
                    "ban entry has placeholder reason".to_owned(),
                    format!(
                        "`{}` in `{key}` has a trivial or placeholder `reason`.",
                        entry.path
                    ),
                    Some(input.config.rel_path.clone()),
                    None,
                    false,
                ));
            }
        }
    }

    if issue_count == 0 {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "ban reasons are substantive".to_owned(),
                "All managed ban entries use substantive non-placeholder `reason` text.".to_owned(),
                Some(input.config.rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
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
#[path = "rs_clippy_15_trivial_reason_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_clippy_15_trivial_reason_tests;
