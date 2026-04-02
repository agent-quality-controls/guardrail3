mod rule;
pub use rule::{check};

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

mod tests;
