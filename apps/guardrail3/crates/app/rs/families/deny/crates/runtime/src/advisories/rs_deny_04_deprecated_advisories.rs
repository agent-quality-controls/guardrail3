use guardrail3_domain_report::{CheckResult, Severity};

use crate::deny_support::parsed_table;
use crate::inputs::ConfigDenyInput;

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
            results.push(CheckResult::from_parts(
                "RS-DENY-04".to_owned(),
                Severity::Warn,
                format!("deprecated advisory field `{deprecated}`"),
                format!(
                    "`{}` uses deprecated `[advisories].{deprecated}`.",
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
#[allow(dead_code)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
pub(crate) use ::test_support::{build_fixture_deny_toml, set_section_string};
#[cfg(test)]
#[path = "rs_deny_04_deprecated_advisories_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_deny_04_deprecated_advisories_tests;
