mod rule;
pub use rule::{check};

#[cfg(test)]
pub(crate) fn repo_facts() -> crate::facts::RepoReleaseFacts {
    crate::test_fixtures::repo_facts()
}
#[cfg(test)]
pub(crate) fn repo_input(
    repo: &crate::facts::RepoReleaseFacts,
) -> crate::inputs::RepoReleaseInput<'_> {
    crate::test_fixtures::repo_input(repo)
}
#[cfg(test)]

mod tests;
