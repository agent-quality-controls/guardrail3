use guardrail3_domain_report::{CheckResult, Severity};

use crate::clippy_support::parse_ban_section;
use crate::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-08";

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
                Severity::Error,
                "ban section malformed".to_owned(),
                malformed.clone(),
                Some(input.config.rel_path.clone()),
                None,
                false,
            ));
        }
        for entry in section.entries {
            if entry.is_plain_string || entry.reason.as_deref().is_none() {
                issue_count += 1;
                results.push(CheckResult::from_parts(
                    ID.to_owned(),
                    Severity::Error,
                    "ban entry missing reason".to_owned(),
                    format!(
                        "`{}` in `{key}` must use table format with a `reason` field.",
                        entry.path
                    ),
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
                "ban entries use reasoned table format".to_owned(),
                "All managed ban entries use table format with a `reason` field.".to_owned(),
                Some(input.config.rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
    }
}

