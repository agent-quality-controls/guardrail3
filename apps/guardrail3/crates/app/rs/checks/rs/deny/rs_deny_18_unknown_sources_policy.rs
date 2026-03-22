use crate::domain::report::{CheckResult, Severity};

use super::deny_support::{expected_sources, section};
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(sources) = section(config, "sources") else {
        results.push(CheckResult {
            id: "RS-DENY-18".to_owned(),
            severity: Severity::Error,
            title: "[sources] section missing".to_owned(),
            message: format!("`{}` has no `[sources]` section.", config.rel_path),
            file: Some(config.rel_path.clone()),
            line: None,
            inventory: false,
        });
        return;
    };
    let (_, expected_unknown_registry, expected_unknown_git) = expected_sources();
    for (key, expected) in [
        ("unknown-registry", expected_unknown_registry),
        ("unknown-git", expected_unknown_git),
    ] {
        match sources.get(key).and_then(toml::Value::as_str) {
            Some(value) if value == expected => {}
            _ => results.push(CheckResult {
                id: "RS-DENY-18".to_owned(),
                severity: Severity::Error,
                title: format!("sources `{key}` has wrong value"),
                message: format!("`{}` must set `[sources].{key} = \"{expected}\"`.", config.rel_path),
                file: Some(config.rel_path.clone()),
                line: None,
                inventory: false,
            }),
        }
    }
}

#[cfg(test)]
#[path = "rs_deny_18_unknown_sources_policy_tests.rs"]
mod tests;
