use guardrail3_domain_report::{CheckResult, Severity};

use crate::deny_support::{ban_name, expected_ban_names, section};
use crate::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(bans) = section(config, "bans") else {
        return;
    };
    let Some(deny_entries) = bans.get("deny").and_then(toml::Value::as_array) else {
        return;
    };
    let expected_names = expected_ban_names(config.profile_name.as_deref());
    let mut extra_count = 0usize;
    for entry in deny_entries {
        let Some(name) = ban_name(entry) else {
            continue;
        };
        if expected_names.contains(&name) {
            continue;
        }
        extra_count += 1;
        results.push(
            CheckResult::from_parts(
                "RS-DENY-26".to_owned(),
                Severity::Info,
                "extra deny ban".to_owned(),
                format!("`{}` adds deny ban `{name}` beyond the managed baseline.", config.rel_path),
                Some(config.rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
    }

    results.push(
        CheckResult::from_parts(
            "RS-DENY-26".to_owned(),
            Severity::Info,
            if extra_count == 0 {
                "no extra deny bans".to_owned()
            } else {
                "extra deny ban count".to_owned()
            },
            format!(
                "`{}` has {extra_count} deny bans beyond the managed baseline.",
                config.rel_path,
            ),
            Some(config.rel_path.clone()),
            None,
            false,
        )
        .as_inventory(),
    );
}
