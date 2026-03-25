use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RootTestInput;

const ID: &str = "RS-TEST-18";

pub fn check(input: &RootTestInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.root.mutants_parsed.as_ref() else {
        return;
    };

    if let Some(excludes) = parsed.get("exclude_re").and_then(toml::Value::as_array) {
        if excludes
            .iter()
            .filter_map(toml::Value::as_str)
            .any(is_exclude_all_pattern)
        {
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
        .and_then(toml::Value::as_float)
    {
        if timeout_multiplier < 1.0 {
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
}

fn is_exclude_all_pattern(pattern: &str) -> bool {
    matches!(pattern.trim(), ".*" | "^.*$" | ".+" | "^.+$")
}

#[cfg(test)]
#[path = "rs_test_18_mutants_config_content_tests.rs"]
mod tests;
