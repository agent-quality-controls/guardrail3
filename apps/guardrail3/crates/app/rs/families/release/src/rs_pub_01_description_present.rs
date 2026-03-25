use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::PublishableCrateReleaseInput;

const ID: &str = "RS-PUB-01";

pub fn check(input: &PublishableCrateReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.krate.publishable {
        return;
    }
    results.push(if input.krate.description_present {
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: format!("{}: description present", input.krate.name),
            message: "Cargo.toml sets `[package].description`.".to_owned(),
            file: Some(input.krate.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }
        .as_inventory()
    } else {
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: format!("{}: missing description", input.krate.name),
            message: "Publishable crates must set `[package].description`.".to_owned(),
            file: Some(input.krate.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }
    });
}

#[cfg(test)]
#[path = "rs_pub_01_description_present_tests/mod.rs"]
mod tests;
