use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::{ConfigDenyInput, CoveredRustUnitInput, UncoveredRustUnitInput};

pub fn check_covered(input: &CoveredRustUnitInput<'_>, results: &mut Vec<CheckResult>) {
    let covered = input.covered;
    if covered.quiet_if_self_hosted {
        return;
    }
    results.push(
        CheckResult::from_parts(
            "RS-DENY-01".to_owned(),
            Severity::Info,
            "Rust root covered by deny config".to_owned(),
            format!(
                "{} `{}` is covered by `{}`.",
                covered.kind.label(),
                rel_label(&covered.rel_dir),
                covered.covering_config_rel
            ),
            Some(covered.covering_config_rel.clone()),
            None,
            false,
        )
        .as_inventory(),
    );
}

pub fn check_uncovered(input: &UncoveredRustUnitInput<'_>, results: &mut Vec<CheckResult>) {
    let uncovered = input.uncovered;
    results.push(CheckResult::from_parts(
        "RS-DENY-01".to_owned(),
        Severity::Error,
        "Rust root uncovered by deny config".to_owned(),
        format!(
            "{} `{}` is not covered by any allowed deny config.",
            uncovered.kind.label(),
            rel_label(&uncovered.rel_dir)
        ),
        None,
        None,
        false,
    ));
}

pub fn check_parse_error(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    if let Some(parse_error) = &config.parse_error {
        results.push(CheckResult::from_parts(
            "RS-DENY-01".to_owned(),
            Severity::Error,
            "deny config parse error".to_owned(),
            format!("`{}` could not be parsed: {parse_error}", config.rel_path),
            Some(config.rel_path.clone()),
            None,
            false,
        ));
    }
}

pub fn check_policy_context_error(parse_error: &str, results: &mut Vec<CheckResult>) {
    results.push(CheckResult::from_parts(
        "RS-DENY-01".to_owned(),
        Severity::Error,
        "deny policy context is not parseable".to_owned(),
        format!("Failed to parse active `guardrail3.toml` used for deny profile selection: {parse_error}"),
        Some("guardrail3.toml".to_owned()),
        None,
        false,
    ));
}

fn rel_label(rel: &str) -> String {
    if rel.is_empty() {
        ".".to_owned()
    } else {
        rel.to_owned()
    }
}

#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
pub(crate) use ::test_support::{build_fixture_deny_toml, copy_fixture, write_file};
#[cfg(test)]

mod tests;
