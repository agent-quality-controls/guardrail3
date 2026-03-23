use crate::domain::report::{CheckResult, Severity};

use super::inputs::PublishableCrateReleaseInput;

const ID: &str = "RS-PUB-13";

pub fn check(input: &PublishableCrateReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let krate = input.krate;
    if !krate.publishable || !krate.is_library {
        return;
    }
    results.push(if krate.docs_rs_present {
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: format!("{}: docs.rs metadata present", krate.name),
            message: "`[package.metadata.docs.rs]` is present.".to_owned(),
            file: Some(krate.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }
        .as_inventory()
    } else {
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: format!("{}: docs.rs metadata missing", krate.name),
            message: "Library crates should set `[package.metadata.docs.rs]` for reproducible docs.rs builds.".to_owned(),
            file: Some(krate.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }
    });
}

#[cfg(test)]
#[path = "rs_pub_13_docs_rs_metadata_tests.rs"]
mod tests;
