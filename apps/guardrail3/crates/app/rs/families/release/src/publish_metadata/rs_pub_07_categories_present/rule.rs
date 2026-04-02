use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::PublishableCrateReleaseInput;

const ID: &str = "RS-PUB-07";

pub fn check(input: &PublishableCrateReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let krate = input.krate;
    if !krate.publishable {
        return;
    }
    match krate.categories_count {
        Some(count) if count > 0 => results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                format!("{}: categories present", krate.name),
                format!("`[package].categories` has {count} entries."),
                Some(krate.cargo_rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        ),
        _ => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            format!("{}: categories missing", krate.name),
            "Publishable crates should set non-empty `[package].categories`.".to_owned(),
            Some(krate.cargo_rel_path.clone()),
            None,
            false,
        )),
    }
}

