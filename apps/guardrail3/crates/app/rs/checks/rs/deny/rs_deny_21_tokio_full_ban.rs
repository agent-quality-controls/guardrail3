use crate::domain::report::{CheckResult, Severity};

use super::deny_support::parse_feature_entries_in_config;
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(parsed) = &config.parsed else {
        return;
    };
    let feature_entries = parse_feature_entries_in_config(parsed);
    let tokio_full_banned = feature_entries
        .iter()
        .any(|entry| entry.name == "tokio" && entry.deny.contains("full"));
    if !tokio_full_banned {
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
}

#[cfg(test)]
#[path = "rs_deny_21_tokio_full_ban_tests.rs"]
mod tests;
