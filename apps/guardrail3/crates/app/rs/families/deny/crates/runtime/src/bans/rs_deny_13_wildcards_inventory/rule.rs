use guardrail3_domain_report::{CheckResult, Severity};

use crate::deny_support::{expected_bans_settings, section};
use crate::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(bans) = section(config, "bans") else {
        return;
    };
    let (expected, _, _) = expected_bans_settings();
    let actual = bans.get("wildcards").and_then(toml::Value::as_str);
    if actual.map(str::to_owned) != expected {
        results.push(CheckResult::from_parts(
            "RS-DENY-CONFIG-09".to_owned(),
            Severity::Warn,
            "wildcards differs from baseline".to_owned(),
            format!(
                "`{}` sets `[bans].wildcards = {}`.",
                config.rel_path,
                actual.unwrap_or("<missing>")
            ),
            Some(config.rel_path.clone()),
            None,
            false,
        ));
    }
}
