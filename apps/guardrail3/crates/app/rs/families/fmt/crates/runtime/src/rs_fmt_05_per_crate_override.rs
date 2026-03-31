use guardrail3_domain_report::{CheckResult, Severity};

use super::facts::RustfmtConfigKind;
use super::inputs::RustfmtExtraConfigInput;

const ID: &str = "RS-FMT-05";

pub fn check(input: &RustfmtExtraConfigInput, results: &mut Vec<CheckResult>) {
    let kind = match input.config_kind {
        RustfmtConfigKind::RustfmtToml => "rustfmt.toml",
        RustfmtConfigKind::DotRustfmtToml => ".rustfmt.toml",
    };

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "Illegal nested rustfmt config".to_owned(),
        format!(
            "{kind} below repository root is forbidden; rustfmt policy is root-only"
        ),
        Some(input.config_rel.clone()),
        None,
        false,
    ));
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
pub(crate) fn run_family_check(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
#[path = "rs_fmt_05_per_crate_override_tests/mod.rs"]
mod rs_fmt_05_per_crate_override_tests;
