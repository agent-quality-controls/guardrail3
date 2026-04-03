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
