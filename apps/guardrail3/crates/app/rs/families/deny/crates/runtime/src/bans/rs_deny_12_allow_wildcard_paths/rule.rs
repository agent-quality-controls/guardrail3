use guardrail3_domain_report::{CheckResult, Severity};

use crate::deny_support::{expected_bans_settings, section};
use crate::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(bans) = section(config, "bans") else {
        results.push(CheckResult::from_parts(
            "RS-DENY-CONFIG-08".to_owned(),
            Severity::Error,
            "[bans] section missing".to_owned(),
            format!("`{}` has no `[bans]` section.", config.rel_path),
            Some(config.rel_path.clone()),
            None,
            false,
        ));
        return;
    };
    let (_, expected, _) = expected_bans_settings();
    match bans
        .get("allow-wildcard-paths")
        .and_then(toml::Value::as_bool)
    {
        Some(value) if value == expected => {}
        _ => results.push(CheckResult::from_parts(
            "RS-DENY-CONFIG-08".to_owned(),
            Severity::Error,
            "allow-wildcard-paths must be true".to_owned(),
            format!(
                "`{}` must set `[bans].allow-wildcard-paths = true`.",
                config.rel_path
            ),
            Some(config.rel_path.clone()),
            None,
            false,
        )),
    }
}
