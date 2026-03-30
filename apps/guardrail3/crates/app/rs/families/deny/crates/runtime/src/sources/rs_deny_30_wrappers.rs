use guardrail3_domain_report::{CheckResult, Severity};

use super::deny_support::{ban_name, expected_bans, join_set, section, string_array};
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    if !config.policy_context_valid {
        return;
    }
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
        let actual_wrappers = string_array(entry.get("wrappers"));
        let Some(expected_ban) = expected.get(&name) else {
            if !actual_wrappers.is_empty() {
                results.push(
                    CheckResult::from_parts(
                        "RS-DENY-30".to_owned(),
                        Severity::Info,
                        "project-specific ban wrappers".to_owned(),
                        format!(
                            "`{}` ban `{name}` adds project-specific wrappers `{}`.",
                            config.rel_path,
                            join_set(&actual_wrappers)
                        ),
                        Some(config.rel_path.clone()),
                        None,
                        false,
                    )
                    .as_inventory(),
                );
            }
            continue;
        };
        if !expected_ban.wrappers.is_empty() && actual_wrappers != expected_ban.wrappers {
            results.push(CheckResult::from_parts(
    "RS-DENY-30".to_owned(),
    Severity::Error,
    "managed ban wrappers changed".to_owned(),
    format!(
                    "`{}` ban `{name}` must keep wrappers `{}`.",
                    config.rel_path,
                    join_set(&expected_ban.wrappers)
                ),
    Some(config.rel_path.clone()),
    None,
    false,
            ));
        } else if expected_ban.wrappers.is_empty() && !actual_wrappers.is_empty() {
            results.push(
                CheckResult::from_parts(
                    "RS-DENY-30".to_owned(),
                    Severity::Info,
                    "project-specific ban wrappers".to_owned(),
                    format!(
                        "`{}` ban `{name}` adds project-specific wrappers `{}`.",
                        config.rel_path,
                        join_set(&actual_wrappers)
                    ),
                    Some(config.rel_path.clone()),
                    None,
                    false,
                )
                .as_inventory(),
            );
        }
    }
}

#[cfg(test)]
pub(crate) fn run_check(deny_toml: &str) -> Vec<CheckResult> {
    crate::run_config_rule_for_test(deny_toml, None, check)
}

#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
pub(crate) use ::test_support::{
    add_deny_ban_entry, build_fixture_deny_toml, copy_fixture, set_deny_ban_wrappers, write_file,
};
#[cfg(test)]
pub(crate) fn expected_ban_wrappers_for_test(
    profile_name: Option<&str>,
) -> std::collections::BTreeMap<String, std::collections::BTreeSet<String>> {
    super::deny_support::expected_bans(profile_name)
        .into_iter()
        .map(|(name, expectation)| (name, expectation.wrappers))
        .collect()
}
#[cfg(test)]
#[path = "rs_deny_30_wrappers_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_deny_30_wrappers_tests;
