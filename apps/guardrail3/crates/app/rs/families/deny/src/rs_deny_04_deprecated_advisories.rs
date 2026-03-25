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
#[path = "rs_deny_04_deprecated_advisories_tests/mod.rs"]
mod tests;
