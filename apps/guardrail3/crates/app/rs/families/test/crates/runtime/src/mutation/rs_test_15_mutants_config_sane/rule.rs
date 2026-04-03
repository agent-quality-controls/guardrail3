use crate::{CheckResult, Severity};

use crate::inputs::RootTestInput;

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
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                "mutants config excludes everything".to_owned(),
                format!(
                    "`{}` contains an `exclude_re` pattern that matches everything. Remove the overly broad exclude pattern.",
                    input.root.mutants_rel_path
                ),
                Some(input.root.mutants_rel_path.clone()),
                None,
                false,
            ));
        }
    }

    if let Some(timeout_multiplier) = parsed
        .get("timeout_multiplier")
        .and_then(timeout_multiplier)
    {
        if timeout_multiplier < 1.0 {
            pushed = true;
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                "mutants timeout multiplier too low".to_owned(),
                format!(
                    "`{}` sets `timeout_multiplier = {timeout_multiplier}`. Set `timeout_multiplier` to 1.0 or higher.",
                    input.root.mutants_rel_path
                ),
                Some(input.root.mutants_rel_path.clone()),
                None,
                false,
            ));
        }
    }

    if !pushed {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "mutants config looks sane".to_owned(),
                format!(
                    "`{}` avoids the known fake-mutation configurations this family bans.",
                    input.root.mutants_rel_path
                ),
                Some(input.root.mutants_rel_path.clone()),
                None,
                false,
            )
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

