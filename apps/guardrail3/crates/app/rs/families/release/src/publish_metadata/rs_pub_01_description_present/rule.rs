use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::PublishableCrateReleaseInput;

const ID: &str = "RS-PUB-01";

pub fn check(input: &PublishableCrateReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.krate.publishable {
        return;
    }
    results.push(if input.krate.description_present {
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            format!("{}: description present", input.krate.name),
            "Cargo.toml sets `[package].description`.".to_owned(),
            Some(input.krate.cargo_rel_path.clone()),
            None,
            false,
        )
        .as_inventory()
    } else {
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            format!("{}: missing description", input.krate.name),
            "Publishable crates must set `[package].description`. Add `description = \"...\"` to `[package]` in Cargo.toml.".to_owned(),
            Some(input.krate.cargo_rel_path.clone()),
            None,
            false,
        )
    });
}

