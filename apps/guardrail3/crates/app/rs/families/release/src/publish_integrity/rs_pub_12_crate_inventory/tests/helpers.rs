pub use super::super::check;

pub(super) fn repo_facts() -> crate::facts::RepoReleaseFacts {
    crate::test_fixtures::repo_facts()
}
pub(super) fn repo_input(
    repo: &crate::facts::RepoReleaseFacts,
) -> crate::inputs::RepoReleaseInput<'_> {
    crate::test_fixtures::repo_input(repo)
}
