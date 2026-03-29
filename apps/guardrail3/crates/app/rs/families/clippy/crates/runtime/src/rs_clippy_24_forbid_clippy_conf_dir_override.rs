#[cfg(test)]
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::CargoConfigOverrideInput;

const ID: &str = "RS-CLIPPY-24";

pub fn check_clean(results: &mut Vec<CheckResult>) {
    results.push(
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: "no clippy config dir overrides found".to_owned(),
            message: "No applicable cargo config surfaces set `CLIPPY_CONF_DIR`.".to_owned(),
            file: None,
            line: None,
            inventory: false,
        }
        .as_inventory(),
    );
}

pub fn check(input: &CargoConfigOverrideInput<'_>, results: &mut Vec<CheckResult>) {
    let (title, message) = match input.parse_error {
        Some(parse_error) => (
            "cargo config override surface is not parseable".to_owned(),
            format!(
                "Failed to parse `{}` while checking for forbidden `CLIPPY_CONF_DIR` overrides: {parse_error}",
                input.rel_path
            ),
        ),
        None => (
            "clippy config dir override is forbidden".to_owned(),
            format!(
                "`{}` sets `CLIPPY_CONF_DIR`, which bypasses the routed clippy policy-root model.",
                input.rel_path
            ),
        ),
    };

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title,
        message,
        file: Some(input.rel_path.to_owned()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
pub(crate) fn run_for_tests(tree: &ProjectTree) -> Vec<CheckResult> {
    let facts = super::facts::collect_for_tests(tree);
    let mut results = Vec::new();
    if facts.cargo_config_overrides.is_empty() {
        check_clean(&mut results);
    } else {
        for override_facts in &facts.cargo_config_overrides {
            check(&CargoConfigOverrideInput::new(override_facts), &mut results);
        }
    }
    results
}

#[cfg(test)]
#[path = "rs_clippy_24_forbid_clippy_conf_dir_override_tests/mod.rs"]
mod rs_clippy_24_forbid_clippy_conf_dir_override_tests;
