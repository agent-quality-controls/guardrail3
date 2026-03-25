use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustfmtRootInput;

const ID: &str = "RS-FMT-07";

pub fn check(input: &RustfmtRootInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(rel) = input.config_rel else {
        return;
    };
    let Some(parsed) = input.parsed else {
        return;
    };

    if let Some(ignore) = parsed.get("ignore") {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "rustfmt ignore escape hatch".to_owned(),
            message: format!("`ignore` excludes paths from formatting: {ignore}"),
            file: Some(rel.to_owned()),
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_fmt_07_ignore_escape_hatch_tests.rs"]
mod tests;
