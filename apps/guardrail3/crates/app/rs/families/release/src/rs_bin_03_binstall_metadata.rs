use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::PublishableCrateReleaseInput;

const ID: &str = "RS-BIN-03";

pub fn check(input: &PublishableCrateReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let krate = input.krate;
    if !krate.publishable || !krate.is_binary {
        return;
    }
    results.push(if krate.has_binstall_metadata {
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: format!("{}: binstall metadata present", krate.name),
            message: "`[package.metadata.binstall]` is present.".to_owned(),
            file: Some(krate.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }
        .as_inventory()
    } else {
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: format!("{}: missing binstall metadata", krate.name),
            message: "Publishable binary crates should set `[package.metadata.binstall]`."
                .to_owned(),
            file: Some(krate.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }
    });
}

#[cfg(test)]
#[path = "rs_bin_03_binstall_metadata_tests/mod.rs"]
mod tests;
