use std::collections::BTreeMap;

use guardrail3_domain_report::{CheckResult, Severity};

use super::deny_support::{ban_name, parse_feature_entries_in_config, section};
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(parsed) = &config.parsed else {
        return;
    };

    if let Some(bans) = section(config, "bans") {
        let mut deny_counts = BTreeMap::<String, usize>::new();
        if let Some(entries) = bans.get("deny").and_then(toml::Value::as_array) {
            for entry in entries {
                if let Some(name) = ban_name(entry) {
                    *deny_counts.entry(name).or_default() += 1;
                }
            }
        }
        for (name, count) in deny_counts {
            if count > 1 {
                results.push(CheckResult::from_parts(
    "RS-DENY-27".to_owned(),
    Severity::Warn,
    "duplicate deny entry".to_owned(),
    format!("`{}` has duplicate deny entry `{name}`.", config.rel_path),
    Some(config.rel_path.clone()),
    None,
    false,
                ));
            }
        }

        let mut skip_counts = BTreeMap::<String, usize>::new();
        if let Some(entries) = bans.get("skip").and_then(toml::Value::as_array) {
            for entry in entries {
                let name =
                    if let Some(crate_field) = entry.get("crate").and_then(toml::Value::as_str) {
                        crate_field
                            .split('@')
                            .next()
                            .unwrap_or(crate_field)
                            .to_owned()
                    } else if let Some(name) = entry.as_str() {
                        name.to_owned()
                    } else if let Some(name) = entry.get("name").and_then(toml::Value::as_str) {
                        name.to_owned()
                    } else {
                        "unknown".to_owned()
                    };
                *skip_counts.entry(name).or_default() += 1;
            }
        }
        for (name, count) in skip_counts {
            if count > 1 {
                results.push(CheckResult::from_parts(
                    "RS-DENY-27".to_owned(),
                    Severity::Warn,
                    "duplicate skip entry".to_owned(),
                    format!("`{}` has duplicate skip entry `{name}`.", config.rel_path),
                    Some(config.rel_path.clone()),
                    None,
                    false,
                ));
            }
        }
    }

    let mut ignore_counts = BTreeMap::<String, usize>::new();
    if let Some(advisories) = section(config, "advisories") {
        if let Some(entries) = advisories.get("ignore").and_then(toml::Value::as_array) {
            for entry in entries {
                let id = if let Some(id) = entry.as_str() {
                    id.to_owned()
                } else if let Some(id) = entry.get("id").and_then(toml::Value::as_str) {
                    id.to_owned()
                } else {
                    "unknown".to_owned()
                };
                *ignore_counts.entry(id).or_default() += 1;
            }
        }
    }
    for (id, count) in ignore_counts {
        if count > 1 {
            results.push(CheckResult::from_parts(
                "RS-DENY-27".to_owned(),
                Severity::Warn,
                "duplicate advisory ignore entry".to_owned(),
                format!(
                    "`{}` has duplicate advisory ignore `{id}`.",
                    config.rel_path
                ),
                Some(config.rel_path.clone()),
                None,
                false,
            ));
        }
    }

    let mut feature_counts = BTreeMap::<String, usize>::new();
    for entry in parse_feature_entries_in_config(parsed) {
        *feature_counts.entry(entry.name).or_default() += 1;
    }
    for (name, count) in feature_counts {
        if count > 1 {
            results.push(CheckResult::from_parts(
                "RS-DENY-27".to_owned(),
                Severity::Warn,
                "duplicate feature-ban entry".to_owned(),
                format!(
                    "`{}` has duplicate `[[bans.features]]` for `{name}`.",
                    config.rel_path
                ),
                Some(config.rel_path.clone()),
                None,
                false,
            ));
        }
    }
}

#[cfg(test)]
pub(crate) fn run_check(deny_toml: &str) -> Vec<CheckResult> {
    crate::run_config_rule_for_test(deny_toml, None, check)
}

#[cfg(test)]
pub(crate) use ::test_support::{
    add_deny_ban_entry, add_skip_entry, build_fixture_deny_toml, set_advisory_ignores,
    set_feature_entries,
};
#[cfg(test)]
#[path = "rs_deny_27_duplicate_entries_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_deny_27_duplicate_entries_tests;
