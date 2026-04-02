#[cfg(test)]
use guardrail3_app_rs_family_view::FamilyView as ProjectTree;
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
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "avoid-breaking-exported-api explicitly false".to_owned(),
                "`avoid-breaking-exported-api = false` is set.".to_owned(),
                Some(input.config.rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        ),
        BoolSetting::Value(true) if input.published_library_policy() => results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "library keeps avoid-breaking-exported-api enabled".to_owned(),
                "Published library profile may legitimately keep `avoid-breaking-exported-api = true`.".to_owned(),
                Some(input.config.rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        ),
        BoolSetting::Value(true) => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "avoid-breaking-exported-api enabled".to_owned(),
            "`avoid-breaking-exported-api = true` suppresses useful lints. Prefer `false`.".to_owned(),
            Some(input.config.rel_path.clone()),
            None,
            false,
        )),
        BoolSetting::WrongType(value) => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "avoid-breaking-exported-api wrong type".to_owned(),
            format!(
                "`avoid-breaking-exported-api` must be a bool, found {}.",
                value_kind(value)
            ),
            Some(input.config.rel_path.clone()),
            None,
            false,
        )),
        BoolSetting::Missing => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "avoid-breaking-exported-api not set".to_owned(),
            "Set `avoid-breaking-exported-api = false` explicitly unless this is a published library.".to_owned(),
            Some(input.config.rel_path.clone()),
            None,
            false,
        )),
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

// reason: test-only sidecar module wiring
mod tests;
