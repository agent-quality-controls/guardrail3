use guardrail3_domain_report::{CheckResult, Severity};

use crate::deny_support::{
    expected_tokio_allowed_features, join_set, parse_feature_entries_in_config,
};
use crate::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(parsed) = &config.parsed else {
        return;
    };
    let feature_entries = parse_feature_entries_in_config(parsed);
    let tokio_entries = feature_entries
        .iter()
        .filter(|entry| entry.name == "tokio")
        .collect::<Vec<_>>();
    if tokio_entries.is_empty() {
        results.push(CheckResult::from_parts(
            "RS-DENY-21".to_owned(),
            Severity::Warn,
            "tokio full feature not banned".to_owned(),
            format!(
                "`{}` must ban `tokio` feature `full` under `[[bans.features]]`.",
                config.rel_path
            ),
            Some(config.rel_path.clone()),
            None,
            false,
        ));
        return;
    }

    if tokio_entries
        .iter()
        .any(|entry| !entry.deny.contains("full"))
    {
        results.push(CheckResult::from_parts(
            "RS-DENY-21".to_owned(),
            Severity::Warn,
            "tokio full feature not banned".to_owned(),
            format!(
                "`{}` must ban `tokio` feature `full` under `[[bans.features]]`.",
                config.rel_path
            ),
            Some(config.rel_path.clone()),
            None,
            false,
        ));
    }

    let expected_allow = expected_tokio_allowed_features();
    if tokio_entries
        .iter()
        .any(|entry| entry.allow != expected_allow)
    {
        results.push(CheckResult::from_parts(
            "RS-DENY-21".to_owned(),
            Severity::Warn,
            "tokio allowed features changed".to_owned(),
            format!(
                "`{}` must keep `tokio` allowed features `{}`.",
                config.rel_path,
                join_set(&expected_allow)
            ),
            Some(config.rel_path.clone()),
            None,
            false,
        ));
    }
}
