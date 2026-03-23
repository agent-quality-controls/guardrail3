use crate::domain::report::{CheckResult, Severity};

use super::inputs::PublishableCrateReleaseInput;

const ID: &str = "RS-PUB-04";

pub fn check(input: &PublishableCrateReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let krate = input.krate;
    if !krate.publishable || krate.readme_declared_false {
        return;
    }
    results.push(if krate.readme_exists {
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: format!("{}: README present", krate.name),
            message: format!("README exists at `{}`.", krate.readme_rel_path),
            file: Some(krate.readme_rel_path.clone()),
            line: None,
            inventory: false,
        }
        .as_inventory()
    } else {
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: format!("{}: README missing", krate.name),
            message: format!(
                "Publishable crate `{}` is missing README content at `{}`.",
                krate.name, krate.readme_rel_path
            ),
            file: Some(krate.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }
    });
}

#[cfg(test)]
#[path = "rs_pub_04_readme_exists_tests/mod.rs"]
mod tests;
