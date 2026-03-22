use crate::domain::report::{CheckResult, Severity};

use super::inputs::{CoveredRustUnitInput, UncoveredRustUnitInput};

const ID: &str = "RS-CLIPPY-01";

pub fn check_covered(input: &CoveredRustUnitInput<'_>, results: &mut Vec<CheckResult>) {
    let scope = if input.rel_dir.is_empty() {
        input.kind.label().to_owned()
    } else {
        format!("{} `{}`", input.kind.label(), input.rel_dir)
    };
    results.push(
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: "Rust unit covered by clippy.toml".to_owned(),
            message: format!("{scope} is covered by `{}`.", input.covering_config_rel),
            file: Some(input.covering_config_rel.to_owned()),
            line: None,
            inventory: false,
        }
        .as_inventory(),
    );
}

pub fn check_uncovered(input: &UncoveredRustUnitInput<'_>, results: &mut Vec<CheckResult>) {
    let scope = if input.rel_dir.is_empty() {
        input.kind.label().to_owned()
    } else {
        format!("{} `{}`", input.kind.label(), input.rel_dir)
    };
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: "Rust unit uncovered by clippy.toml".to_owned(),
        message: format!(
            "{scope} is not covered by any allowed clippy.toml at the validation root, a workspace root, or a standalone package root."
        ),
        file: Some(input.rel_dir.to_owned()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_clippy_01_coverage_tests.rs"]
mod tests;
