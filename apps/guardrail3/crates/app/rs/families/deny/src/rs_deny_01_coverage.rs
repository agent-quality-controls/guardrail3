use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::{ConfigDenyInput, CoveredRustUnitInput, UncoveredRustUnitInput};

pub fn check_covered(input: &CoveredRustUnitInput<'_>, results: &mut Vec<CheckResult>) {
    let covered = input.covered;
    results.push(
        CheckResult {
            id: "RS-DENY-01".to_owned(),
            severity: Severity::Info,
            title: "Rust root covered by deny config".to_owned(),
            message: format!(
                "{} `{}` is covered by `{}`.",
                covered.kind.label(),
                rel_label(&covered.rel_dir),
                covered.covering_config_rel
            ),
            file: Some(covered.covering_config_rel.clone()),
            line: None,
            inventory: false,
        }
        .as_inventory(),
    );
}

pub fn check_uncovered(input: &UncoveredRustUnitInput<'_>, results: &mut Vec<CheckResult>) {
    let uncovered = input.uncovered;
    results.push(CheckResult {
        id: "RS-DENY-01".to_owned(),
        severity: Severity::Error,
        title: "Rust root uncovered by deny config".to_owned(),
        message: format!(
            "{} `{}` is not covered by any allowed deny config.",
            uncovered.kind.label(),
            rel_label(&uncovered.rel_dir)
        ),
        file: None,
        line: None,
        inventory: false,
    });
}

pub fn check_parse_error(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    if let Some(parse_error) = &config.parse_error {
        results.push(CheckResult {
            id: "RS-DENY-01".to_owned(),
            severity: Severity::Error,
            title: "deny config parse error".to_owned(),
            message: format!("`{}` could not be parsed: {parse_error}", config.rel_path),
            file: Some(config.rel_path.clone()),
            line: None,
            inventory: false,
        });
    }
}

fn rel_label(rel: &str) -> String {
    if rel.is_empty() {
        ".".to_owned()
    } else {
        rel.to_owned()
    }
}

#[cfg(test)]
#[path = "rs_deny_01_coverage_tests/mod.rs"]
mod tests;
