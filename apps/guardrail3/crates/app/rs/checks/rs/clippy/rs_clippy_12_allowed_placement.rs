use crate::domain::report::{CheckResult, Severity};

use super::facts::ClippyConfigFacts;

const ID: &str = "RS-CLIPPY-12";

pub fn check(config: &ClippyConfigFacts, results: &mut Vec<CheckResult>) {
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: "clippy.toml in forbidden location".to_owned(),
        message: format!(
            "`{}` is not an allowed clippy policy root. clippy.toml is allowed only at the validation root, workspace roots, and standalone package roots that are not workspace members.",
            config.rel_path
        ),
        file: Some(config.rel_path.clone()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_clippy_12_allowed_placement_tests.rs"]
mod tests;
