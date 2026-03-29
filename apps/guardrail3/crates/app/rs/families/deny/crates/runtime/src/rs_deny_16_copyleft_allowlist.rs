use guardrail3_domain_report::{CheckResult, Severity};

use super::deny_support::section;
use super::inputs::ConfigDenyInput;

const COPYLEFT_LICENSES: &[&str] = &[
    "GPL-2.0-only",
    "GPL-2.0-or-later",
    "GPL-3.0-only",
    "GPL-3.0-or-later",
    "GPL-2.0",
    "GPL-3.0",
    "AGPL-3.0-only",
    "AGPL-3.0-or-later",
    "AGPL-3.0",
    "LGPL-2.1-only",
    "LGPL-2.1-or-later",
    "LGPL-3.0-only",
    "LGPL-3.0-or-later",
    "LGPL-2.1",
    "LGPL-3.0",
    "SSPL-1.0",
    "EUPL-1.2",
];

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(licenses) = section(config, "licenses") else {
        return;
    };
    let actual: std::collections::BTreeSet<String> = licenses
        .get("allow")
        .and_then(toml::Value::as_array)
        .map(|entries| {
            entries
                .iter()
                .filter_map(toml::Value::as_str)
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default();

    for license in actual {
        if COPYLEFT_LICENSES.contains(&license.as_str()) {
            results.push(CheckResult {
                id: "RS-DENY-16".to_owned(),
                severity: Severity::Warn,
                title: "copyleft license allowed".to_owned(),
                message: format!("`{}` allows copyleft license `{license}`.", config.rel_path),
                file: Some(config.rel_path.clone()),
                line: None,
                inventory: false,
            });
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
    add_allowed_license, build_fixture_deny_toml, copy_fixture, write_file,
};
#[cfg(test)]
#[path = "rs_deny_16_copyleft_allowlist_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_deny_16_copyleft_allowlist_tests;
