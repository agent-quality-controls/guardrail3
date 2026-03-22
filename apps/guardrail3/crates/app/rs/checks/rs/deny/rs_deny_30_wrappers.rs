use crate::domain::report::{CheckResult, Severity};

use super::deny_support::{ban_name, expected_bans, join_set, section, string_array};
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(bans) = section(config, "bans") else {
        return;
    };
    let Some(deny_entries) = bans.get("deny").and_then(toml::Value::as_array) else {
        return;
    };
    let expected = expected_bans(config.profile_name.as_deref());
    for entry in deny_entries {
        let Some(name) = ban_name(entry) else {
            continue;
        };
        let Some(expected_ban) = expected.get(&name) else {
            continue;
        };
        let actual_wrappers = string_array(entry.get("wrappers"));
        if !expected_ban.wrappers.is_empty() && actual_wrappers != expected_ban.wrappers {
            results.push(CheckResult {
                id: "RS-DENY-30".to_owned(),
                severity: Severity::Error,
                title: "managed ban wrappers changed".to_owned(),
                message: format!(
                    "`{}` ban `{name}` must keep wrappers `{}`.",
                    config.rel_path,
                    join_set(&expected_ban.wrappers)
                ),
                file: Some(config.rel_path.clone()),
                line: None,
                inventory: false,
            });
        } else if expected_ban.wrappers.is_empty() && !actual_wrappers.is_empty() {
            results.push(CheckResult {
                id: "RS-DENY-30".to_owned(),
                severity: Severity::Warn,
                title: "ban wrappers weaken canonical ban".to_owned(),
                message: format!(
                    "`{}` ban `{name}` adds wrappers `{}`.",
                    config.rel_path,
                    join_set(&actual_wrappers)
                ),
                file: Some(config.rel_path.clone()),
                line: None,
                inventory: false,
            });
        }
    }
}

#[cfg(test)]
#[path = "rs_deny_30_wrappers_tests.rs"]
mod tests;
