use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::PublishableCrateReleaseInput;

const ID: &str = "RS-BIN-03";

pub fn check(input: &PublishableCrateReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let krate = input.krate;
    if !krate.publishable || !krate.is_binary {
        return;
    }
    results.push(if krate.has_binstall_metadata {
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            format!("{}: binstall metadata present", krate.name),
            "`[package.metadata.binstall]` is present.".to_owned(),
            Some(krate.cargo_rel_path.clone()),
            None,
            false,
        )
        .as_inventory()
    } else {
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            format!("{}: missing binstall metadata", krate.name),
            "Publishable binary crates should set `[package.metadata.binstall]`.".to_owned(),
            Some(krate.cargo_rel_path.clone()),
            None,
            false,
        )
    });
}

