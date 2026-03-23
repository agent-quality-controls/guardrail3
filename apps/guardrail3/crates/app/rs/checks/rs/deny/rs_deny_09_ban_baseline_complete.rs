use crate::domain::report::{CheckResult, Severity};

use super::deny_support::{ban_name, expected_bans, string_array};
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(bans) = config.parsed.as_ref().and_then(|parsed| parsed.get("bans")) else {
        results.push(CheckResult {
            id: "RS-DENY-09".to_owned(),
            severity: Severity::Error,
            title: "[bans] section missing".to_owned(),
            message: format!("`{}` has no `[bans]` section.", config.rel_path),
            file: Some(config.rel_path.clone()),
            line: None,
            inventory: false,
        });
        return;
    };

    let Some(deny_entries) = bans.get("deny").and_then(toml::Value::as_array) else {
        results.push(CheckResult {
            id: "RS-DENY-09".to_owned(),
            severity: Severity::Error,
            title: "[bans].deny missing".to_owned(),
            message: format!("`{}` must contain `[bans].deny`.", config.rel_path),
            file: Some(config.rel_path.clone()),
            line: None,
            inventory: false,
        });
        return;
    };

    let expected = expected_bans(config.profile_name.as_deref());
    let actual_names: std::collections::BTreeSet<String> =
        deny_entries.iter().filter_map(ban_name).collect();

    for name in expected.keys() {
        if !actual_names.contains(name) {
            results.push(CheckResult {
                id: "RS-DENY-09".to_owned(),
                severity: Severity::Error,
                title: "missing canonical ban".to_owned(),
                message: format!("`{}` is missing deny ban `{name}`.", config.rel_path),
                file: Some(config.rel_path.clone()),
                line: None,
                inventory: false,
            });
        }
    }

    for entry in deny_entries {
        let Some(name) = ban_name(entry) else {
            continue;
        };
        let Some(expected_ban) = expected.get(&name) else {
            continue;
        };
        let wrappers = string_array(entry.get("wrappers"));
        if !expected_ban.wrappers.is_empty() && wrappers != expected_ban.wrappers {
            results.push(CheckResult {
                id: "RS-DENY-09".to_owned(),
                severity: Severity::Error,
                title: "managed ban wrappers changed".to_owned(),
                message: format!(
                    "`{}` ban `{name}` no longer matches the canonical managed entry.",
                    config.rel_path
                ),
                file: Some(config.rel_path.clone()),
                line: None,
                inventory: false,
            });
        }
    }
}

#[cfg(test)]
#[path = "rs_deny_09_ban_baseline_complete_tests/mod.rs"]
mod tests;
