use crate::domain::report::{CheckResult, Severity};

use super::inputs::PublishableCrateReleaseInput;

const ID: &str = "RS-PUB-09";

pub fn check(input: &PublishableCrateReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let krate = input.krate;
    if !krate.publishable {
        return;
    }
    let Some(run) = &krate.dry_run else {
        return;
    };
    results.push(if run.success {
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: format!("{}: publish dry-run passed", krate.name),
            message: "`cargo publish --dry-run` succeeded.".to_owned(),
            file: Some(krate.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }
        .as_inventory()
    } else {
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: format!("{}: publish dry-run failed", krate.name),
            message: format!(
                "`cargo publish --dry-run` failed: {}",
                run.stderr.lines().take(3).collect::<Vec<_>>().join("; ")
            ),
            file: Some(krate.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }
    });
}

#[cfg(test)]
#[path = "rs_pub_09_publish_dry_run_tests.rs"]
mod tests;
