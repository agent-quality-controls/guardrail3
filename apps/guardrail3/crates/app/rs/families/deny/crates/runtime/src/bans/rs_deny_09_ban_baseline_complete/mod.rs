use guardrail3_domain_report::{CheckResult, Severity};

use crate::deny_support::{ban_name, expected_bans, string_array};
use crate::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    if !config.policy_context_valid {
        return;
    }
    let Some(bans) = config.parsed.as_ref().and_then(|parsed| parsed.get("bans")) else {
        results.push(CheckResult::from_parts(
            "RS-DENY-09".to_owned(),
            Severity::Error,
            "[bans] section missing".to_owned(),
            format!("`{}` has no `[bans]` section.", config.rel_path),
            Some(config.rel_path.clone()),
            None,
            false,
        ));
        return;
    };

    let Some(deny_entries) = bans.get("deny").and_then(toml::Value::as_array) else {
        results.push(CheckResult::from_parts(
            "RS-DENY-09".to_owned(),
            Severity::Error,
            "[bans].deny missing".to_owned(),
            format!("`{}` must contain `[bans].deny`.", config.rel_path),
            Some(config.rel_path.clone()),
            None,
            false,
        ));
        return;
    };

    let expected = expected_bans(config.profile_name.as_deref());
    let actual_names: std::collections::BTreeSet<String> =
        deny_entries.iter().filter_map(ban_name).collect();

    for name in expected.keys() {
        if !actual_names.contains(name) {
            results.push(CheckResult::from_parts(
                "RS-DENY-09".to_owned(),
                Severity::Error,
                "missing canonical ban".to_owned(),
                format!("`{}` is missing deny ban `{name}`.", config.rel_path),
                Some(config.rel_path.clone()),
                None,
                false,
            ));
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
            results.push(CheckResult::from_parts(
                "RS-DENY-09".to_owned(),
                Severity::Error,
                "managed ban wrappers changed".to_owned(),
                format!(
                    "`{}` ban `{name}` no longer matches the canonical managed entry.",
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
pub(crate) fn run_check_with_profile(deny_toml: &str, profile_name: &str) -> Vec<CheckResult> {
    crate::run_config_rule_for_test(deny_toml, Some(profile_name), check)
}

#[cfg(test)]
pub(crate) use crate::config_facts_with_profile;
#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
pub(crate) use ::test_support::{
    build_fixture_deny_toml, copy_fixture, remove_deny_ban, set_deny_ban_wrappers, write_file,
};
#[cfg(test)]
pub(crate) fn expected_ban_names_for_test(
    profile_name: Option<&str>,
) -> std::collections::BTreeSet<String> {
    crate::deny_support::expected_bans(profile_name)
        .into_keys()
        .collect()
}
#[cfg(test)]

mod tests;
