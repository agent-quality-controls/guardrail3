use crate::domain::report::{CheckResult, Severity};

use super::inputs::PublishableCrateReleaseInput;

const ID: &str = "RS-PUB-08";

pub fn check(input: &PublishableCrateReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let krate = input.krate;
    if !krate.publishable {
        return;
    }
    if krate.workspace_version {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: format!("{}: version inherited from workspace", krate.name),
                message: "`version.workspace = true` is present.".to_owned(),
                file: Some(krate.cargo_rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
        return;
    }
    results.push(if krate.version_valid {
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: format!("{}: valid semver", krate.name),
            message: format!(
                "`version = \"{}\"` parses as semver.",
                krate.version_string.clone().unwrap_or_default()
            ),
            file: Some(krate.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }
        .as_inventory()
    } else {
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: format!("{}: invalid semver", krate.name),
            message:
                "Publishable crates must set a valid semver version or `version.workspace = true`."
                    .to_owned(),
            file: Some(krate.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }
    });
}

#[cfg(test)]
#[path = "rs_pub_08_valid_semver_tests/mod.rs"]
mod tests;
