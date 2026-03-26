use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RootTestInput;

const ID: &str = "RS-TEST-15";

pub fn check(input: &RootTestInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.root.mutants_parsed.as_ref() else {
        return;
    };

    let mut pushed = false;

    if let Some(excludes) = parsed.get("exclude_re").and_then(toml::Value::as_array) {
        if excludes
            .iter()
            .filter_map(toml::Value::as_str)
            .any(is_exclude_all_pattern)
        {
            pushed = true;
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Warn,
                title: "mutants config excludes everything".to_owned(),
                message: format!(
                    "`{}` contains an `exclude_re` pattern that matches everything.",
                    input.root.mutants_rel_path
                ),
                file: Some(input.root.mutants_rel_path.clone()),
                line: None,
                inventory: false,
            });
        }
    }

    if let Some(timeout_multiplier) = parsed
        .get("timeout_multiplier")
        .and_then(timeout_multiplier)
    {
        if timeout_multiplier < 1.0 {
            pushed = true;
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Warn,
                title: "mutants timeout multiplier too low".to_owned(),
                message: format!(
                    "`{}` sets `timeout_multiplier = {timeout_multiplier}`.",
                    input.root.mutants_rel_path
                ),
                file: Some(input.root.mutants_rel_path.clone()),
                line: None,
                inventory: false,
            });
        }
    }

    if !pushed {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "mutants config looks sane".to_owned(),
                message: format!(
                    "`{}` avoids the known fake-mutation configurations this family bans.",
                    input.root.mutants_rel_path
                ),
                file: Some(input.root.mutants_rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    }
}

fn is_exclude_all_pattern(pattern: &str) -> bool {
    matches!(pattern.trim(), ".*" | "^.*$" | ".+" | "^.+$")
}

fn timeout_multiplier(value: &toml::Value) -> Option<f64> {
    value
        .as_float()
        .or_else(|| value.as_integer().map(|integer| integer as f64))
}

#[cfg(test)]
#[path = "rs_test_15_mutants_config_sane_tests/mod.rs"]
mod rs_test_15_mutants_config_sane_tests;
