use crate::domain::report::{CheckResult, Severity};

use super::deny_support::{expected_bans_settings, section};
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(bans) = section(config, "bans") else {
        return;
    };
    let (_, _, expected_highlight) = expected_bans_settings();
    let actual = bans.get("highlight").and_then(toml::Value::as_str);
    if actual.map(str::to_owned) != expected_highlight {
        results.push(
            CheckResult {
                id: "RS-DENY-11".to_owned(),
                severity: Severity::Info,
                title: "highlight differs from baseline".to_owned(),
                message: format!(
                    "`{}` sets `[bans].highlight = {}`.",
                    config.rel_path,
                    actual.unwrap_or("<missing>")
                ),
                file: Some(config.rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    }
}

#[cfg(test)]
#[path = "rs_deny_11_highlight_inventory_tests.rs"]
mod tests;
