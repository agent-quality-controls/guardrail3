use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_reason_policy::validate_reason_text;

use crate::clippy_support::parse_ban_section;
use crate::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-15";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.config.parsed.as_ref() else {
        return;
    };

    let mut weak_reason_count = 0usize;
    let mut documented_count = 0usize;
    let mut missing_reason_count = 0usize;

    for key in [
        "disallowed-methods",
        "disallowed-types",
        "disallowed-macros",
    ] {
        let section = parse_ban_section(parsed, key);
        for entry in section.entries {
            match entry.reason.as_deref() {
                None => {
                    missing_reason_count += 1;
                }
                Some(reason) => match validate_reason_text(reason) {
                    Ok(()) => {
                        documented_count += 1;
                        results.push(CheckResult::from_parts(
                            ID.to_owned(),
                            Severity::Warn,
                            "ban entry uses documented escape hatch".to_owned(),
                            format!(
                                "`{}` in `{key}` uses a documented ban entry with reason `{reason}`.",
                                entry.path
                            ),
                            Some(input.config.rel_path.clone()),
                            None,
                            false,
                        ));
                    }
                    Err(issue) => {
                        weak_reason_count += 1;
                        results.push(CheckResult::from_parts(
                            ID.to_owned(),
                            Severity::Error,
                            "ban entry reason too weak".to_owned(),
                            format!(
                                "`{}` in `{key}` has a weak `reason`: {}.",
                                entry.path,
                                issue.message()
                            ),
                            Some(input.config.rel_path.clone()),
                            None,
                            false,
                        ));
                    }
                },
            }
        }
    }

    let total = documented_count + missing_reason_count + weak_reason_count;
    if total == 0 {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "no documented ban entries".to_owned(),
                "No managed ban entries are present, so there are no documented clippy escape hatches to review."
                    .to_owned(),
                Some(input.config.rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
        return;
    }

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Warn,
        "ban entry count".to_owned(),
        format!(
            "`{}` has {total} clippy ban entries ({documented_count} documented, {missing_reason_count} missing reasons, {weak_reason_count} weak reasons).",
            input.config.rel_path
        ),
        None,
        None,
        false,
    ));
}

