use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::PublishableCrateReleaseInput;

const ID: &str = "RS-PUB-08";

pub fn check(input: &PublishableCrateReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let krate = input.krate;
    if !krate.publishable {
        return;
    }
    if krate.workspace_version && krate.version_valid {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                format!("{}: version inherited from workspace", krate.name),
                "`version.workspace = true` is present.".to_owned(),
                Some(krate.cargo_rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
        return;
    }
    results.push(if krate.version_valid {
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            format!("{}: valid semver", krate.name),
            format!(
                "`version = \"{}\"` parses as semver.",
                krate.version_string.clone().unwrap_or_default()
            ),
            Some(krate.cargo_rel_path.clone()),
            None,
            false,
        )
        .as_inventory()
    } else {
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            format!("{}: invalid semver", krate.name),
            "Publishable crates must set a valid semver version or `version.workspace = true`."
                .to_owned(),
            Some(krate.cargo_rel_path.clone()),
            None,
            false,
        )
    });
}

