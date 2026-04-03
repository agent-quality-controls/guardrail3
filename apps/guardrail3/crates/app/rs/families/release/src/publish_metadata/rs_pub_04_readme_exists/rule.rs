use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::PublishableCrateReleaseInput;

const ID: &str = "RS-PUB-04";

pub fn check(input: &PublishableCrateReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let krate = input.krate;
    if !krate.publishable || krate.readme_declared_false {
        return;
    }
    results.push(if krate.readme_exists {
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            format!("{}: README present", krate.name),
            format!("README exists at `{}`.", krate.readme_rel_path),
            Some(krate.readme_rel_path.clone()),
            None,
            false,
        )
        .as_inventory()
    } else {
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            format!("{}: README missing", krate.name),
            format!(
                "Publishable crate `{}` is missing README content at `{}`. Create a README.md for this crate.",
                krate.name, krate.readme_rel_path
            ),
            Some(krate.cargo_rel_path.clone()),
            None,
            false,
        )
    });
}

