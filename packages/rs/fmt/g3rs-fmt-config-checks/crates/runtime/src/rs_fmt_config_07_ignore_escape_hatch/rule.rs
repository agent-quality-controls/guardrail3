use g3rs_fmt_config_checks_types::{G3RsFmtConfigChecksInput, G3RsFmtRustfmtConfigState};
use guardrail3_check_types::{G3CheckResult, G3Severity};
use guardrail3_reason_policy::validate_reason_text;

const ID: &str = "RS-FMT-CONFIG-07";

pub(crate) fn check(input: &G3RsFmtConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let G3RsFmtRustfmtConfigState::Parsed(rustfmt) = &input.rustfmt_state else {
        return;
    };
    if rustfmt.ignore.is_empty() {
        return;
    }

    let ignore = format!("{:?}", rustfmt.ignore);
    let reason = input
        .escape_hatches
        .iter()
        .find(|entry| {
            entry.family == "fmt"
                && entry.file == input.rustfmt_rel_path
                && entry.kind == "ignore"
                && entry.selector == "ignore"
        })
        .map(|entry| entry.reason.as_str());

    match reason {
        None => results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "rustfmt ignore missing reason".to_owned(),
            format!(
                "`{}` uses `ignore = {ignore}` without a matching escape-hatch reason. Add an escape-hatch entry in guardrail3.toml with family = \"fmt\", file = \"{}\", kind = \"ignore\", and a reason explaining why these paths are excluded.",
                input.rustfmt_rel_path, input.rustfmt_rel_path
            ),
            Some(input.rustfmt_rel_path.clone()),
            None,
        )),
        Some(reason) => match validate_reason_text(reason) {
            Ok(()) => results.push(
                G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Warn,
                    "rustfmt ignore escape hatch".to_owned(),
                    format!(
                        "`{}` excludes paths from formatting with documented reason `{reason}`: {ignore}",
                        input.rustfmt_rel_path
                    ),
                    Some(input.rustfmt_rel_path.clone()),
                    None,
                ),
            ),
            Err(issue) => results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "rustfmt ignore reason too weak".to_owned(),
                format!(
                    "`{}` uses `ignore = {ignore}` with a weak reason: {}. Provide a more specific reason explaining why these paths cannot be formatted.",
                    input.rustfmt_rel_path,
                    issue.message()
                ),
                Some(input.rustfmt_rel_path.clone()),
                None,
            )),
        },
    }

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Warn,
        "rustfmt ignore count".to_owned(),
        format!(
            "`{}` has 1 rustfmt ignore escape hatch.",
            input.rustfmt_rel_path
        ),
        Some(input.rustfmt_rel_path.clone()),
        None,
    ));
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
