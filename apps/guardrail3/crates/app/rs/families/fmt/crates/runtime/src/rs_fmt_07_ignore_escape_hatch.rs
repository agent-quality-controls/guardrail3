use guardrail3_domain_report::{CheckResult, Severity};

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
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "rustfmt ignore escape hatch".to_owned(),
            format!("`ignore` excludes paths from formatting: {ignore}"),
            Some(rel.to_owned()),
            None,
            false,
        ));
    }
}

#[cfg(test)]
pub(crate) fn run_check(parsed: toml::Value) -> Vec<CheckResult> {
    let input = RustfmtRootInput {
        config_rel: Some("rustfmt.toml".to_owned()),
        parsed: Some(parsed),
        cargo_edition: super::facts::CargoEditionState::Present("2024".to_owned()),
        toolchain_channel: super::facts::ToolchainChannelState::Present("stable".to_owned()),
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
#[path = "rs_fmt_07_ignore_escape_hatch_tests/mod.rs"]
mod rs_fmt_07_ignore_escape_hatch_tests;
