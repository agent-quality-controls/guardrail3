#[cfg(test)]
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::{CheckResult, Severity};

use super::clippy_support::{BoolSetting, bool_setting, value_kind};
use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-16";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    if input.policy_context_parse_error().is_some() {
        return;
    }
    let Some(parsed) = input.config.parsed.as_ref() else {
        return;
    };

    match bool_setting(parsed, "avoid-breaking-exported-api") {
        BoolSetting::Value(false) => results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "avoid-breaking-exported-api explicitly false".to_owned(),
                message: "`avoid-breaking-exported-api = false` is set.".to_owned(),
                file: Some(input.config.rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        ),
        BoolSetting::Value(true) if input.published_library_policy() => results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "library keeps avoid-breaking-exported-api enabled".to_owned(),
                message: "Published library profile may legitimately keep `avoid-breaking-exported-api = true`.".to_owned(),
                file: Some(input.config.rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        ),
        BoolSetting::Value(true) => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "avoid-breaking-exported-api enabled".to_owned(),
            message: "`avoid-breaking-exported-api = true` suppresses useful lints. Prefer `false`.".to_owned(),
            file: Some(input.config.rel_path.clone()),
            line: None,
            inventory: false,
        }),
        BoolSetting::WrongType(value) => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "avoid-breaking-exported-api wrong type".to_owned(),
            message: format!(
                "`avoid-breaking-exported-api` must be a bool, found {}.",
                value_kind(value)
            ),
            file: Some(input.config.rel_path.clone()),
            line: None,
            inventory: false,
        }),
        BoolSetting::Missing => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "avoid-breaking-exported-api not set".to_owned(),
            message: "Set `avoid-breaking-exported-api = false` explicitly unless this is a published library.".to_owned(),
            file: Some(input.config.rel_path.clone()),
            line: None,
            inventory: false,
        }),
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
#[path = "rs_clippy_16_avoid_breaking_exported_api_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_clippy_16_avoid_breaking_exported_api_tests;
