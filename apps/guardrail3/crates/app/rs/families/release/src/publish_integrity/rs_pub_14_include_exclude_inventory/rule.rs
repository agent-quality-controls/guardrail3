use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::PublishableCrateReleaseInput;

const ID: &str = "RS-PUB-14";

pub fn check(input: &PublishableCrateReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let krate = input.krate;
    if !krate.publishable {
        return;
    }
    results.push(if krate.include_exclude_present {
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            format!("{}: include/exclude configured", krate.name),
            "Cargo.toml sets `include` or `exclude`.".to_owned(),
            Some(krate.cargo_rel_path.clone()),
            None,
            false,
        )
        .as_inventory()
    } else {
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            format!("{}: include/exclude missing", krate.name),
            "Publishable crates should set `include` or `exclude` patterns to control what gets published. Add `include = [\"src/**\", \"Cargo.toml\", \"README.md\", \"LICENSE\"]` to `[package]`.".to_owned(),
            Some(krate.cargo_rel_path.clone()),
            None,
            false,
        )
    });
}

