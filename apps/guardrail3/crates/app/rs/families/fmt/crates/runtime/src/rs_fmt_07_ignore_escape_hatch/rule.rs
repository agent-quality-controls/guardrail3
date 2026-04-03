use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_reason_policy::validate_reason_text;

use crate::inputs::RustfmtRootInput;

const ID: &str = "RS-FMT-07";

pub fn check(input: &RustfmtRootInput, results: &mut Vec<CheckResult>) {
    let Some(rel) = input.config_rel.as_deref() else {
        return;
    };
    let Some(parsed) = input.parsed.as_ref() else {
        return;
    };

    if let Some(ignore) = parsed.get("ignore") {
        let reason = input
            .escape_hatches
            .iter()
            .find(|entry| {
                entry.family() == "fmt"
                    && entry.file() == rel
                    && entry.kind() == "ignore"
                    && entry.selector() == "ignore"
            })
            .map(|entry| entry.reason());

        match reason {
            None => results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                "rustfmt ignore missing reason".to_owned(),
                format!("`{rel}` uses `ignore = {ignore}` without a matching escape-hatch reason. Add an escape-hatch entry in guardrail3.toml with family = \"fmt\", file = \"{rel}\", kind = \"ignore\", and a reason explaining why these paths are excluded."),
                Some(rel.to_owned()),
                None,
                false,
            )),
            Some(reason) => match validate_reason_text(reason) {
                Ok(()) => {
                    results.push(CheckResult::from_parts(
                        ID.to_owned(),
                        Severity::Warn,
                        "rustfmt ignore escape hatch".to_owned(),
                        format!(
                            "`{rel}` excludes paths from formatting with documented reason `{reason}`: {ignore}"
                        ),
                        Some(rel.to_owned()),
                        None,
                        false,
                    ));
                }
                Err(issue) => results.push(CheckResult::from_parts(
                    ID.to_owned(),
                    Severity::Error,
                    "rustfmt ignore reason too weak".to_owned(),
                    format!(
                        "`{rel}` uses `ignore = {ignore}` with a weak reason: {}. Provide a more specific reason explaining why these paths cannot be formatted.",
                        issue.message()
                    ),
                    Some(rel.to_owned()),
                    None,
                    false,
                )),
            },
        }

        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "rustfmt ignore count".to_owned(),
            format!("`{rel}` has 1 rustfmt ignore escape hatch."),
            Some(rel.to_owned()),
            None,
            false,
        ));
    }
}

