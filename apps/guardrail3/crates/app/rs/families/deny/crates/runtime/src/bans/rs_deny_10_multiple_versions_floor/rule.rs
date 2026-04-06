use guardrail3_domain_report::{CheckResult, Severity};

use crate::deny_support::section;
use crate::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(bans) = section(config, "bans") else {
        results.push(CheckResult::from_parts(
            "RS-DENY-CONFIG-06".to_owned(),
            Severity::Warn,
            "[bans] section missing".to_owned(),
            format!("`{}` has no `[bans]` section.", config.rel_path),
            Some(config.rel_path.clone()),
            None,
            false,
        ));
        return;
    };

    match bans.get("multiple-versions").and_then(toml::Value::as_str) {
        Some("deny") => {}
        Some(other) => results.push(CheckResult::from_parts(
            "RS-DENY-CONFIG-06".to_owned(),
            Severity::Warn,
            "multiple-versions weaker than baseline".to_owned(),
            format!(
                "`{}` sets `[bans].multiple-versions = \"{other}\"`.",
                config.rel_path
            ),
            Some(config.rel_path.clone()),
            None,
            false,
        )),
        None => results.push(CheckResult::from_parts(
            "RS-DENY-CONFIG-06".to_owned(),
            Severity::Warn,
            "multiple-versions missing".to_owned(),
            format!(
                "`{}` does not set `[bans].multiple-versions`.",
                config.rel_path
            ),
            Some(config.rel_path.clone()),
            None,
            false,
        )),
    }
}
