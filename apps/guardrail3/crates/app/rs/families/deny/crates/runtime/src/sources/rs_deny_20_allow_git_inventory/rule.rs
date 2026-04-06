use guardrail3_domain_report::{CheckResult, Severity};

use crate::deny_support::section;
use crate::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(sources) = section(config, "sources") else {
        return;
    };
    if let Some(allow_git) = sources.get("allow-git").and_then(toml::Value::as_array) {
        if !allow_git.is_empty() {
            results.push(CheckResult::from_parts(
                "RS-DENY-CONFIG-15".to_owned(),
                Severity::Warn,
                "allow-git is non-empty".to_owned(),
                format!("`{}` has non-empty `[sources].allow-git`.", config.rel_path),
                Some(config.rel_path.clone()),
                None,
                false,
            ));
        }
        for entry in allow_git.iter().filter_map(toml::Value::as_str) {
            if entry.trim().is_empty() {
                results.push(CheckResult::from_parts(
                    "RS-DENY-CONFIG-15".to_owned(),
                    Severity::Error,
                    "allow-git entry must be non-empty".to_owned(),
                    format!(
                        "`{}` has blank `[sources].allow-git` entry.",
                        config.rel_path
                    ),
                    Some(config.rel_path.clone()),
                    None,
                    false,
                ));
                continue;
            }
            results.push(
                CheckResult::from_parts(
                    "RS-DENY-CONFIG-15".to_owned(),
                    Severity::Info,
                    "allow-git entry".to_owned(),
                    format!("`{}` allows git source `{entry}`.", config.rel_path),
                    Some(config.rel_path.clone()),
                    None,
                    false,
                )
                .as_inventory(),
            );
        }
    }
}
