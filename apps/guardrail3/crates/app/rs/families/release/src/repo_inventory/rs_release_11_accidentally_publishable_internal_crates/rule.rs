use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::PublishableCrateReleaseInput;

const ID: &str = "RS-RELEASE-11";

pub fn check(input: &PublishableCrateReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let krate = input.krate;
    if !krate.publishable {
        return;
    }
    if krate.description_present || krate.license_present || krate.repository_present {
        return;
    }
    results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Error,
    format!("{} may be accidentally publishable", krate.name),
    format!(
            "Crate `{}` is publishable but missing description, license, and repository metadata. If it is internal, set `publish = false`.",
            krate.name
        ),
    Some(krate.cargo_rel_path.clone()),
    None,
    false,
    ));
}

