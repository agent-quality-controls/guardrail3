use std::collections::BTreeSet;

use guardrail3_domain_report::{CheckResult, Severity};

use crate::clippy_support::{expected_required_type_bans, parse_ban_section};
use crate::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-05";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    if input.policy_context_parse_error().is_some() {
        return;
    }
    let Some(parsed) = input.config.parsed.as_ref() else {
        return;
    };

    let section = parse_ban_section(parsed, "disallowed-types");
    for malformed in &section.malformed_messages {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "disallowed-types section malformed".to_owned(),
            malformed.clone(),
            Some(input.config.rel_path.clone()),
            None,
            false,
        ));
    }

    let found: BTreeSet<_> = section
        .entries
        .into_iter()
        .map(|entry| entry.path)
        .collect();
    for expected in expected_required_type_bans(input.garde_enabled()) {
        if found.contains(expected) {
            results.push(
                CheckResult::from_parts(
                    ID.to_owned(),
                    Severity::Info,
                    "type ban present".to_owned(),
                    format!("`{expected}` is banned."),
                    Some(input.config.rel_path.clone()),
                    None,
                    false,
                )
                .as_inventory(),
            );
        } else {
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                "missing type ban".to_owned(),
                format!("`{expected}` is not present in `disallowed-types`."),
                Some(input.config.rel_path.clone()),
                None,
                false,
            ));
        }
    }
}

