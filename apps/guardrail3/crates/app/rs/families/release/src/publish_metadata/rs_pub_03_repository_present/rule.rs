use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::PublishableCrateReleaseInput;

const ID: &str = "RS-PUB-03";

pub fn check(input: &PublishableCrateReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.krate.publishable {
        return;
    }
    results.push(if input.krate.repository_present {
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            format!("{}: repository present", input.krate.name),
            "Cargo.toml sets `[package].repository`.".to_owned(),
            Some(input.krate.cargo_rel_path.clone()),
            None,
            false,
        )
        .as_inventory()
    } else {
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            format!("{}: missing repository", input.krate.name),
            "Publishable crates must set `[package].repository`.".to_owned(),
            Some(input.krate.cargo_rel_path.clone()),
            None,
            false,
        )
    });
}

