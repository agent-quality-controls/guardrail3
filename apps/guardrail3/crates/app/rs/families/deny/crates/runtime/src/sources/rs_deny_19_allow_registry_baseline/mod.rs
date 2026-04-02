use guardrail3_domain_report::{CheckResult, Severity};

use crate::deny_support::section;
use crate::inputs::ConfigDenyInput;

const CANONICAL_CRATES_IO_REGISTRY: &str = "sparse+https://index.crates.io/";

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(sources) = section(config, "sources") else {
        results.push(CheckResult::from_parts(
            "RS-DENY-19".to_owned(),
            Severity::Error,
            "[sources] allow-registry missing".to_owned(),
            format!(
                "`{}` has no valid crates.io registry allow-list.",
                config.rel_path
            ),
            Some(config.rel_path.clone()),
            None,
            false,
        ));
        return;
    };
    let Some(allow_registry_value) = sources.get("allow-registry") else {
        results.push(CheckResult::from_parts(
            "RS-DENY-19".to_owned(),
            Severity::Error,
            "[sources] allow-registry missing".to_owned(),
            format!(
                "`{}` has no valid crates.io registry allow-list.",
                config.rel_path
            ),
            Some(config.rel_path.clone()),
            None,
            false,
        ));
        return;
    };
    let Some(allow_registry_entries) = allow_registry_value.as_array() else {
        results.push(CheckResult::from_parts(
            "RS-DENY-19".to_owned(),
            Severity::Error,
            "malformed allow-registry container".to_owned(),
            format!(
                "`{}` must use an array for `[sources].allow-registry` entries.",
                config.rel_path
            ),
            Some(config.rel_path.clone()),
            None,
            false,
        ));
        return;
    };
    let mut allow_registry = Vec::new();
    for (index, entry) in allow_registry_entries.iter().enumerate() {
        if let Some(registry) = entry.as_str() {
            allow_registry.push(registry);
        } else {
            results.push(CheckResult::from_parts(
                "RS-DENY-19".to_owned(),
                Severity::Error,
                "registry allow entry must be a string".to_owned(),
                format!(
                    "`{}` has non-string `[sources].allow-registry` entry at index {index}.",
                    config.rel_path
                ),
                Some(config.rel_path.clone()),
                None,
                false,
            ));
        }
    }
    if allow_registry.len() != 1 {
        results.push(CheckResult::from_parts(
            "RS-DENY-19".to_owned(),
            Severity::Error,
            "allow-registry must contain exactly one entry".to_owned(),
            format!(
                "`{}` must contain exactly one `[sources].allow-registry` entry: `{CANONICAL_CRATES_IO_REGISTRY}`.",
                config.rel_path
            ),
            Some(config.rel_path.clone()),
            None,
            false,
        ));
    } else if allow_registry[0] != CANONICAL_CRATES_IO_REGISTRY {
        results.push(CheckResult::from_parts(
            "RS-DENY-19".to_owned(),
            Severity::Error,
            "canonical crates.io registry not allowed".to_owned(),
            format!(
                "`{}` must allow only `{CANONICAL_CRATES_IO_REGISTRY}` in `[sources].allow-registry`.",
                config.rel_path
            ),
            Some(config.rel_path.clone()),
            None,
            false,
        ));
    }
    let unexpected_registries: Vec<_> = allow_registry
        .iter()
        .filter(|registry| **registry != CANONICAL_CRATES_IO_REGISTRY)
        .copied()
        .collect();
    if !unexpected_registries.is_empty() {
        results.push(CheckResult::from_parts(
            "RS-DENY-19".to_owned(),
            Severity::Error,
            "unexpected registry allowed".to_owned(),
            format!(
                "`{}` allows unexpected registries: {}.",
                config.rel_path,
                unexpected_registries.join(", ")
            ),
            Some(config.rel_path.clone()),
            None,
            false,
        ));
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
pub(crate) use crate::config_facts;
#[cfg(test)]
pub(crate) use ::test_support::{build_fixture_deny_toml, set_allow_registries};
#[cfg(test)]
pub(crate) fn expected_sources_for_test() -> (std::collections::BTreeSet<String>, String, String) {
    crate::deny_support::expected_sources()
}
#[cfg(test)]

// reason: test-only sidecar module wiring
mod rs_deny_19_allow_registry_baseline_tests;
