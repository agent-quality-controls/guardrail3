use guardrail3_domain_report::{CheckResult, Severity};

use crate::deny_support::parse_feature_entries_in_config;
use crate::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(parsed) = &config.parsed else {
        return;
    };
    for entry in parse_feature_entries_in_config(parsed) {
        if entry.name != "tokio" {
            results.push(
                CheckResult::from_parts(
                    "RS-DENY-CONFIG-17".to_owned(),
                    Severity::Info,
                    "extra feature ban".to_owned(),
                    format!(
                        "`{}` has extra feature-ban entry for `{}`.",
                        config.rel_path, entry.name
                    ),
                    Some(config.rel_path.clone()),
                    None,
                    false,
                )
                .as_inventory(),
            );
        }
    }
}
