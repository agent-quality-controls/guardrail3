use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::PublishableCrateReleaseInput;

const ID: &str = "RS-PUB-13";

pub fn check(input: &PublishableCrateReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let krate = input.krate;
    if !krate.publishable || !krate.is_library {
        return;
    }
    results.push(if krate.docs_rs_present {
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            format!("{}: docs.rs metadata present", krate.name),
            "`[package.metadata.docs.rs]` is present.".to_owned(),
            Some(krate.cargo_rel_path.clone()),
            None,
            false,
        )
        .as_inventory()
    } else {
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            format!("{}: docs.rs metadata missing", krate.name),
            "Library crates should set `[package.metadata.docs.rs]` for reproducible docs.rs builds.".to_owned(),
            Some(krate.cargo_rel_path.clone()),
            None,
            false,
        )
    });
}

