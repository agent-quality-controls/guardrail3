use crate::domain::report::{CheckResult, Severity};

use super::inputs::PublishableCrateReleaseInput;

const ID: &str = "RS-RELEASE-11";

pub fn check(input: &PublishableCrateReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let krate = input.krate;
    if !krate.publishable {
        return;
    }
    if krate.description_present || krate.license_present || krate.repository_present {
        return;
    }
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Warn,
        title: format!("{} may be accidentally publishable", krate.name),
        message: format!(
            "Crate `{}` is publishable but missing description, license, and repository metadata. If it is internal, set `publish = false`.",
            krate.name
        ),
        file: Some(krate.cargo_rel_path.clone()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_release_11_accidentally_publishable_internal_crates_tests/mod.rs"]
mod tests;
