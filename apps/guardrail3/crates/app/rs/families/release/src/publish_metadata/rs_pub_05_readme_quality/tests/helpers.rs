pub use super::super::check;

pub(super) fn crate_facts(name: &str) -> crate::facts::PublishableCrateFacts {
    crate::test_fixtures::crate_facts(name)
}
pub(super) fn crate_input(
    krate: &crate::facts::PublishableCrateFacts,
) -> crate::inputs::PublishableCrateReleaseInput<'_> {
    crate::test_fixtures::crate_input(krate)
}
