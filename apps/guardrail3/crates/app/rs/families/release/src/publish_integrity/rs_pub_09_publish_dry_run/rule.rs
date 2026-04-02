use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::PublishableCrateReleaseInput;

const ID: &str = "RS-PUB-09";

pub fn check(input: &PublishableCrateReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let krate = input.krate;
    if !krate.publishable {
        return;
    }
    let Some(run) = &krate.dry_run else {
        results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Error,
    format!("{}: publish dry-run missing", krate.name),
    "Expected `cargo publish --dry-run` result in thorough mode, but no result was collected.".to_owned(),
    Some(krate.cargo_rel_path.clone()),
    None,
    false,
        ));
        return;
    };
    results.push(if run.success() {
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            format!("{}: publish dry-run passed", krate.name),
            "`cargo publish --dry-run` succeeded.".to_owned(),
            Some(krate.cargo_rel_path.clone()),
            None,
            false,
        )
        .as_inventory()
    } else {
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            format!("{}: publish dry-run failed", krate.name),
            format!(
                "`cargo publish --dry-run` failed: {}",
                run.stderr().lines().take(3).collect::<Vec<_>>().join("; ")
            ),
            Some(krate.cargo_rel_path.clone()),
            None,
            false,
        )
    });
}

