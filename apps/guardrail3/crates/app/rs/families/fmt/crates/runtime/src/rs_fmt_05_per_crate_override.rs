use guardrail3_domain_report::{CheckResult, Severity};

use super::facts::RustfmtConfigKind;
use super::inputs::RustfmtExtraConfigInput;

const ID: &str = "RS-FMT-05";

pub fn check(input: &RustfmtExtraConfigInput, results: &mut Vec<CheckResult>) {
    let kind = match input.config_kind {
        RustfmtConfigKind::RustfmtToml => "rustfmt.toml",
        RustfmtConfigKind::DotRustfmtToml => ".rustfmt.toml",
    };

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Warn,
        title: "Per-crate rustfmt override".to_owned(),
        message: format!("{kind} below workspace root overrides root formatting policy"),
        file: Some(input.config_rel.clone()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
pub(crate) fn run_check(config_rel: &str, config_kind: RustfmtConfigKind) -> Vec<CheckResult> {
    let input = RustfmtExtraConfigInput {
        config_rel: config_rel.to_owned(),
        config_kind,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
#[path = "rs_fmt_05_per_crate_override_tests/mod.rs"]
mod rs_fmt_05_per_crate_override_tests;
