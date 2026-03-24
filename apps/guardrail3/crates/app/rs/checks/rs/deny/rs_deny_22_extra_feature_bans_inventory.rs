use crate::domain::report::{CheckResult, Severity};

use super::deny_support::parse_feature_entries_in_config;
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(parsed) = &config.parsed else {
        return;
    };
    for entry in parse_feature_entries_in_config(parsed) {
        if entry.name != "tokio" {
            results.push(
                CheckResult {
                    id: "RS-DENY-22".to_owned(),
                    severity: Severity::Info,
                    title: "extra feature ban".to_owned(),
                    message: format!(
                        "`{}` has extra feature-ban entry for `{}`.",
                        config.rel_path, entry.name
                    ),
                    file: Some(config.rel_path.clone()),
                    line: None,
                    inventory: false,
                }
                .as_inventory(),
            );
        }
    }
}

#[cfg(test)]
#[path = "rs_deny_22_extra_feature_bans_inventory_tests/mod.rs"]
mod tests;
