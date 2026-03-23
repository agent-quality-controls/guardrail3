use crate::domain::report::{CheckResult, Severity};

use super::inputs::PublishableCrateReleaseInput;

const ID: &str = "RS-PUB-06";

pub fn check(input: &PublishableCrateReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let krate = input.krate;
    if !krate.publishable {
        return;
    }
    match krate.keywords_count {
        Some(0) | None => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: format!("{}: keywords missing", krate.name),
            message: "Publishable crates should set 1-5 `[package].keywords`.".to_owned(),
            file: Some(krate.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }),
        Some(count) if count > 5 => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: format!("{}: too many keywords", krate.name),
            message: format!(
                "`[package].keywords` has {count} entries; crates.io allows at most 5."
            ),
            file: Some(krate.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }),
        Some(count) => results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: format!("{}: keywords present", krate.name),
                message: format!("`[package].keywords` has {count} entries."),
                file: Some(krate.cargo_rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        ),
    }
}

#[cfg(test)]
#[path = "rs_pub_06_keywords_present_tests/mod.rs"]
mod tests;
