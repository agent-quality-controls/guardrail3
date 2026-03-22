use crate::domain::report::{CheckResult, Severity};

use super::deny_support::{
    expected_tokio_allowed_features, join_set, parse_feature_entries_in_config,
};
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(parsed) = &config.parsed else {
        return;
    };
    let feature_entries = parse_feature_entries_in_config(parsed);
    let Some(tokio_entry) = feature_entries.iter().find(|entry| entry.name == "tokio") else {
        results.push(CheckResult {
            id: "RS-DENY-21".to_owned(),
            severity: Severity::Warn,
            title: "tokio full feature not banned".to_owned(),
            message: format!(
                "`{}` must ban `tokio` feature `full` under `[[bans.features]]`.",
                config.rel_path
            ),
            file: Some(config.rel_path.clone()),
            line: None,
            inventory: false,
        });
        return;
    };

    if !tokio_entry.deny.contains("full") {
        results.push(CheckResult {
            id: "RS-DENY-21".to_owned(),
            severity: Severity::Warn,
            title: "tokio full feature not banned".to_owned(),
            message: format!(
                "`{}` must ban `tokio` feature `full` under `[[bans.features]]`.",
                config.rel_path
            ),
            file: Some(config.rel_path.clone()),
            line: None,
            inventory: false,
        });
    }

    let expected_allow = expected_tokio_allowed_features();
    if tokio_entry.allow != expected_allow {
        results.push(CheckResult {
            id: "RS-DENY-21".to_owned(),
            severity: Severity::Warn,
            title: "tokio allowed features changed".to_owned(),
            message: format!(
                "`{}` must keep `tokio` allowed features `{}`.",
                config.rel_path,
                join_set(&expected_allow)
            ),
            file: Some(config.rel_path.clone()),
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_deny_21_tokio_full_ban_tests.rs"]
mod tests;
