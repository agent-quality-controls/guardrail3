use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_reason_policy::validate_reason_text;

use super::inputs::RustfmtRootInput;

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
                format!("`{rel}` uses `ignore = {ignore}` without a matching escape-hatch reason."),
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
                        "`{rel}` uses `ignore = {ignore}` with a weak reason: {}.",
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
            None,
            None,
            false,
        ));
    }
}

#[cfg(test)]
pub(crate) fn run_check(parsed: toml::Value) -> Vec<CheckResult> {
    run_check_with_escape_hatches(parsed, Vec::new())
}

#[cfg(test)]
pub(crate) fn run_check_with_escape_hatches(
    parsed: toml::Value,
    escape_hatches: Vec<guardrail3_domain_config::types::EscapeHatchConfig>,
) -> Vec<CheckResult> {
    let input = RustfmtRootInput {
        config_rel: Some("rustfmt.toml".to_owned()),
        parsed: Some(parsed),
        escape_hatches,
        cargo_edition: super::facts::CargoEditionState::Present("2024".to_owned()),
        toolchain_channel: super::facts::ToolchainChannelState::Present("stable".to_owned()),
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]

mod rs_fmt_07_ignore_escape_hatch_tests;
