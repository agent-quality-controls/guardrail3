use guardrail3_domain_report::{CheckResult, Severity};

use crate::deny_support::{expected_bans_settings, section};
use crate::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(bans) = section(config, "bans") else {
        return;
    };
    let (_, _, expected_highlight) = expected_bans_settings();
    let actual = bans.get("highlight").and_then(toml::Value::as_str);
    if actual.map(str::to_owned) != expected_highlight {
        results.push(
            CheckResult::from_parts(
                "RS-DENY-11".to_owned(),
                Severity::Info,
                "highlight differs from baseline".to_owned(),
                format!(
                    "`{}` sets `[bans].highlight = {}`.",
                    config.rel_path,
                    actual.unwrap_or("<missing>")
                ),
                Some(config.rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
    }
}
