use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::PublishableCrateReleaseInput;

const ID: &str = "RS-PUB-06";

pub fn check(input: &PublishableCrateReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let krate = input.krate;
    if !krate.publishable {
        return;
    }
    match krate.keywords_count {
        Some(0) | None => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            format!("{}: keywords missing", krate.name),
            "Publishable crates must set 1-5 `[package].keywords`. Add `keywords = [\"...\"]` to `[package]` in Cargo.toml.".to_owned(),
            Some(krate.cargo_rel_path.clone()),
            None,
            false,
        )),
        Some(count) if count > 5 => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            format!("{}: too many keywords", krate.name),
            format!("`[package].keywords` has {count} entries; crates.io allows at most 5."),
            Some(krate.cargo_rel_path.clone()),
            None,
            false,
        )),
        Some(count) => results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                format!("{}: keywords present", krate.name),
                format!("`[package].keywords` has {count} entries."),
                Some(krate.cargo_rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        ),
    }
}

