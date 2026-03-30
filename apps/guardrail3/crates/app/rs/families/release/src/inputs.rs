use super::facts::{
    PublishableCrateFacts, ReleaseEdgeFacts, ReleaseInputFailureFacts, RepoReleaseFacts,
};

#[derive(Debug)]
pub struct RepoReleaseInput<'a> {
    pub(crate) repo: &'a RepoReleaseFacts,
}

impl<'a> RepoReleaseInput<'a> {
    pub const fn new(repo: &'a RepoReleaseFacts) -> Self {
        Self { repo }
    }
}

#[derive(Debug)]
pub struct PublishableCrateReleaseInput<'a> {
    pub(crate) krate: &'a PublishableCrateFacts,
}

impl<'a> PublishableCrateReleaseInput<'a> {
    pub const fn new(krate: &'a PublishableCrateFacts) -> Self {
        Self { krate }
    }
}

#[derive(Debug)]
pub struct ReleaseEdgeInput<'a> {
    pub(crate) edge: &'a ReleaseEdgeFacts,
}

impl<'a> ReleaseEdgeInput<'a> {
    pub const fn new(edge: &'a ReleaseEdgeFacts) -> Self {
        Self { edge }
    }
}

#[derive(Debug)]
pub struct ReleaseInputFailureInput<'a> {
    pub(crate) failure: &'a ReleaseInputFailureFacts,
}

impl<'a> ReleaseInputFailureInput<'a> {
    pub const fn new(failure: &'a ReleaseInputFailureFacts) -> Self {
        Self { failure }
    }
}
