use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ReleaseInputFailureInput;

const ID: &str = "RS-RELEASE-12";

pub fn check(input: &ReleaseInputFailureInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "Release-family input failure".to_owned(),
        input.failure.message.clone(),
        Some(input.failure.rel_path.clone()),
        None,
        false,
    ));
}

#[cfg(test)]
pub(super) fn run_tree(
    tree: &guardrail3_app_rs_family_mapper::RsProjectSurface,
    tc: &dyn guardrail3_outbound_traits::ToolChecker,
    thorough: bool,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::test_fixtures::run_tree(tree, tc, thorough)
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
pub(super) use test_support::{StubToolChecker, dir_entry, project_tree, temp_root, write_file};

#[cfg(test)]
#[path = "rs_release_12_input_failures_tests/mod.rs"]
mod rs_release_12_input_failures_tests;
