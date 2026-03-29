use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::PublishableCrateReleaseInput;

const ID: &str = "RS-PUB-09";

pub fn check(input: &PublishableCrateReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let krate = input.krate;
    if !krate.publishable {
        return;
    }
    let Some(run) = &krate.dry_run else {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: format!("{}: publish dry-run missing", krate.name),
            message: "Expected `cargo publish --dry-run` result in thorough mode, but no result was collected.".to_owned(),
            file: Some(krate.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        });
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
pub(super) fn run_family(
    root: &std::path::Path,
    thorough: bool,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::test_fixtures::run_family(root, thorough)
}

#[cfg(test)]
pub(super) fn copy_fixture() -> tempfile::TempDir {
    crate::test_fixtures::copy_fixture()
}
#[cfg(test)]
pub(super) fn crate_facts(name: &str) -> crate::facts::PublishableCrateFacts {
    crate::test_fixtures::crate_facts(name)
}

#[cfg(test)]
pub(super) fn crate_input(
    krate: &crate::facts::PublishableCrateFacts,
) -> crate::inputs::PublishableCrateReleaseInput<'_> {
    crate::test_fixtures::crate_input(krate)
}
#[cfg(test)]
pub(super) use test_support::write_file;

#[cfg(test)]
#[path = "rs_pub_09_publish_dry_run_tests/mod.rs"]
mod rs_pub_09_publish_dry_run_tests;
