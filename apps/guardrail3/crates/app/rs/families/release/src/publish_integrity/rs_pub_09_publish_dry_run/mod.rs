mod rule;
pub use rule::{check};

#[cfg(test)]
pub(crate) fn run_family(
    root: &std::path::Path,
    thorough: bool,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::test_fixtures::run_family(root, thorough)
}
#[cfg(test)]
pub(crate) fn copy_fixture() -> tempfile::TempDir {
    crate::test_fixtures::copy_fixture()
}
#[cfg(test)]
pub(crate) fn crate_facts(name: &str) -> crate::facts::PublishableCrateFacts {
    crate::test_fixtures::crate_facts(name)
}
#[cfg(test)]
pub(crate) fn crate_input(
    krate: &crate::facts::PublishableCrateFacts,
) -> crate::inputs::PublishableCrateReleaseInput<'_> {
    crate::test_fixtures::crate_input(krate)
}
#[cfg(test)]
pub(super) use test_support::write_file;
#[cfg(test)]

mod tests;
