use crate::domain::report::{CheckResult, Severity};

use super::deny_support::{expected_bans_settings, section};
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(bans) = section(config, "bans") else {
        return;
    };
    let (expected, _, _) = expected_bans_settings();
    let actual = bans.get("wildcards").and_then(toml::Value::as_str);
    if actual.map(str::to_owned) != expected {
        results.push(CheckResult {
            id: "RS-DENY-13".to_owned(),
            severity: Severity::Warn,
            title: "wildcards differs from baseline".to_owned(),
            message: format!(
                "`{}` sets `[bans].wildcards = {}`.",
                config.rel_path,
                actual.unwrap_or("<missing>")
            ),
            file: Some(config.rel_path.clone()),
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_deny_13_wildcards_inventory_tests/mod.rs"]
mod tests;
