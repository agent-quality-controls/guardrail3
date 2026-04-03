use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::PublishableCrateReleaseInput;

const ID: &str = "RS-PUB-02";

pub fn check(input: &PublishableCrateReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.krate.publishable {
        return;
    }
    results.push(if input.krate.license_present {
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            format!("{}: license present", input.krate.name),
            "Cargo.toml sets `license` or `license-file`.".to_owned(),
            Some(input.krate.cargo_rel_path.clone()),
            None,
            false,
        )
        .as_inventory()
    } else {
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            format!("{}: missing license", input.krate.name),
            "Publishable crates must set `license` or `license-file`. Add `license = \"MIT\"` (or appropriate license) to `[package]` in Cargo.toml.".to_owned(),
            Some(input.krate.cargo_rel_path.clone()),
            None,
            false,
        )
    });
}

