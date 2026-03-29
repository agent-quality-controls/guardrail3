use guardrail3_domain_report::{CheckResult, Severity};

use super::deny_support::parsed_table;
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(table) = parsed_table(config) else {
        return;
    };

    for deprecated in ["vulnerability", "notice", "unsound"] {
        if table
            .get("advisories")
            .and_then(|value| value.get(deprecated))
            .is_some()
        {
            results.push(CheckResult {
                id: "RS-DENY-04".to_owned(),
                severity: Severity::Warn,
                title: format!("deprecated advisory field `{deprecated}`"),
                message: format!(
                    "`{}` uses deprecated `[advisories].{deprecated}`.",
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
pub(crate) fn run_check(deny_toml: &str) -> Vec<CheckResult> {
    crate::run_config_rule_for_test(deny_toml, None, check)
}

#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
pub(crate) use ::test_support::{
    build_fixture_deny_toml, copy_fixture, set_section_string, write_file,
};
#[cfg(test)]
#[path = "rs_deny_04_deprecated_advisories_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_deny_04_deprecated_advisories_tests;
