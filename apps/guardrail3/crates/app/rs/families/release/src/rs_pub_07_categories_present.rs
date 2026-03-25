use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::PublishableCrateReleaseInput;

const ID: &str = "RS-PUB-07";

pub fn check(input: &PublishableCrateReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let krate = input.krate;
    if !krate.publishable {
        return;
    }
    match krate.categories_count {
        Some(count) if count > 0 => results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: format!("{}: categories present", krate.name),
                message: format!("`[package].categories` has {count} entries."),
                file: Some(krate.cargo_rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        ),
        _ => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: format!("{}: categories missing", krate.name),
            message: "Publishable crates should set non-empty `[package].categories`.".to_owned(),
            file: Some(krate.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }),
    }
}

#[cfg(test)]
#[path = "rs_pub_07_categories_present_tests/mod.rs"]
mod tests;
