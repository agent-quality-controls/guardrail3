use guardrail3_domain_report::{CheckResult, Severity};

use super::deny_support::section;
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(sources) = section(config, "sources") else {
        results.push(CheckResult {
            id: "RS-DENY-19".to_owned(),
            severity: Severity::Error,
            title: "[sources] allow-registry missing".to_owned(),
            message: format!(
                "`{}` has no valid crates.io registry allow-list.",
                config.rel_path
            ),
            file: Some(config.rel_path.clone()),
            line: None,
            inventory: false,
        });
        return;
    };
    let allow_registry = sources
        .get("allow-registry")
        .and_then(toml::Value::as_array)
        .map(|entries| {
            entries
                .iter()
                .filter_map(toml::Value::as_str)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let has_crates_io = allow_registry.iter().any(|registry| {
        *registry == "https://github.com/rust-lang/crates.io-index"
            || *registry == "sparse+https://index.crates.io/"
    });
    if !has_crates_io {
        results.push(CheckResult {
            id: "RS-DENY-19".to_owned(),
            severity: Severity::Error,
            title: "crates.io registry not allowed".to_owned(),
            message: format!(
                "`{}` must include crates.io in `[sources].allow-registry`.",
                config.rel_path
            ),
            file: Some(config.rel_path.clone()),
            line: None,
            inventory: false,
        });
    }
    let unexpected_registries: Vec<_> = allow_registry
        .iter()
        .filter(|registry| {
            **registry != "https://github.com/rust-lang/crates.io-index"
                && **registry != "sparse+https://index.crates.io/"
        })
        .copied()
        .collect();
    if !unexpected_registries.is_empty() {
        results.push(CheckResult {
            id: "RS-DENY-19".to_owned(),
            severity: Severity::Error,
            title: "unexpected registry allowed".to_owned(),
            message: format!(
                "`{}` allows unexpected registries: {}.",
                config.rel_path,
                unexpected_registries.join(", ")
            ),
            file: Some(config.rel_path.clone()),
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_deny_19_allow_registry_baseline_tests/mod.rs"]
mod tests;
