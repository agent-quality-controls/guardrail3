use crate::domain::report::{CheckResult, Severity};

use super::inputs::PublishableCrateReleaseInput;

const ID: &str = "RS-PUB-03";

pub fn check(input: &PublishableCrateReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.krate.publishable {
        return;
    }
    results.push(if input.krate.repository_present {
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: format!("{}: repository present", input.krate.name),
            message: "Cargo.toml sets `[package].repository`.".to_owned(),
            file: Some(input.krate.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }
        .as_inventory()
    } else {
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: format!("{}: missing repository", input.krate.name),
            message: "Publishable crates must set `[package].repository`.".to_owned(),
            file: Some(input.krate.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }
    });
}

#[cfg(test)]
#[path = "rs_pub_03_repository_present_tests.rs"]
mod tests;
