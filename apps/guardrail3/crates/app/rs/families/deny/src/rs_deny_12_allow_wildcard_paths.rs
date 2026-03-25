use guardrail3_domain_report::{CheckResult, Severity};

use super::deny_support::{expected_bans_settings, section};
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(bans) = section(config, "bans") else {
        results.push(CheckResult {
            id: "RS-DENY-12".to_owned(),
            severity: Severity::Error,
            title: "[bans] section missing".to_owned(),
            message: format!("`{}` has no `[bans]` section.", config.rel_path),
            file: Some(config.rel_path.clone()),
            line: None,
            inventory: false,
        });
        return;
    };
    let (_, expected, _) = expected_bans_settings();
    match bans
        .get("allow-wildcard-paths")
        .and_then(toml::Value::as_bool)
    {
        Some(value) if value == expected => {}
        _ => results.push(CheckResult {
            id: "RS-DENY-12".to_owned(),
            severity: Severity::Error,
            title: "allow-wildcard-paths must be true".to_owned(),
            message: format!(
                "`{}` must set `[bans].allow-wildcard-paths = true`.",
                config.rel_path
            ),
            file: Some(config.rel_path.clone()),
            line: None,
            inventory: false,
        }),
    }
}

#[cfg(test)]
#[path = "rs_deny_12_allow_wildcard_paths_tests/mod.rs"]
mod tests;
