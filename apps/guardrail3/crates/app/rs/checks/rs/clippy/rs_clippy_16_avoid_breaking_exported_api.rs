use crate::domain::report::{CheckResult, Severity};

use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-16";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.config.parsed.as_ref() else {
        return;
    };

    match parsed.get("avoid-breaking-exported-api").and_then(toml::Value::as_bool) {
        Some(false) => results.push(
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
        Some(true) if input.profile_name() == Some("library") && input.package_publishable() => results.push(
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
        Some(true) => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "avoid-breaking-exported-api enabled".to_owned(),
            message: "`avoid-breaking-exported-api = true` suppresses useful lints. Prefer `false`.".to_owned(),
            file: Some(input.config.rel_path.clone()),
            line: None,
            inventory: false,
        }),
        None => results.push(CheckResult {
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
#[path = "rs_clippy_16_avoid_breaking_exported_api_tests.rs"]
mod tests;
