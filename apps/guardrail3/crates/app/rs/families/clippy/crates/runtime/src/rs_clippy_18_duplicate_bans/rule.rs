use std::collections::BTreeMap;

use guardrail3_domain_report::{CheckResult, Severity};

use crate::clippy_support::parse_ban_section;
use crate::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-18";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.config.parsed.as_ref() else {
        return;
    };

    let mut issue_count = 0usize;

    for key in [
        "disallowed-methods",
        "disallowed-types",
        "disallowed-macros",
    ] {
        let section = parse_ban_section(parsed, key);
        for malformed in &section.malformed_messages {
            issue_count += 1;
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Warn,
                "ban section malformed".to_owned(),
                malformed.clone(),
                Some(input.config.rel_path.clone()),
                None,
                false,
            ));
        }
        let mut counts = BTreeMap::new();
        for entry in section.entries {
            *counts.entry(entry.path).or_insert(0usize) += 1;
        }
        for (path, count) in counts {
            if count > 1 {
                issue_count += 1;
                results.push(CheckResult::from_parts(
                    ID.to_owned(),
                    Severity::Warn,
                    "duplicate ban entry".to_owned(),
                    format!("`{path}` appears {count} times in `{key}`. Remove the duplicate entries."),
                    Some(input.config.rel_path.clone()),
                    None,
                    false,
                ));
            }
        }
    }

    if issue_count == 0 {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "ban entries are duplicate-free".to_owned(),
                "Managed ban sections contain no duplicate paths.".to_owned(),
                Some(input.config.rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
    }
}

