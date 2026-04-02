use guardrail3_domain_report::{CheckResult, Severity};

use crate::deny_support::section;
use crate::inputs::ConfigDenyInput;

const ADVISORY_IGNORE_THRESHOLD: usize = 5;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(advisories) = section(config, "advisories") else {
        return;
    };
    let Some(ignore_entries) = advisories.get("ignore").and_then(toml::Value::as_array) else {
        return;
    };
    if ignore_entries.len() > ADVISORY_IGNORE_THRESHOLD {
        results.push(CheckResult::from_parts(
            "RS-DENY-29".to_owned(),
            Severity::Warn,
            "advisory ignore list is large".to_owned(),
            format!(
                "`{}` has {} `[advisories].ignore` entries (threshold: {}).",
                config.rel_path,
                ignore_entries.len(),
                ADVISORY_IGNORE_THRESHOLD
            ),
            Some(config.rel_path.clone()),
            None,
            false,
        ));
    }
}

#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
pub(crate) use crate::config_facts;
#[cfg(test)]
pub(crate) use ::test_support::{build_fixture_deny_toml, set_advisory_ignores};
#[cfg(test)]

mod tests;
