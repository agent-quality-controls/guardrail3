use super::facts::{
    PublishableCrateFacts, ReleaseEdgeFacts, ReleaseInputFailureFacts, RepoReleaseFacts,
};

pub struct RepoReleaseInput<'a> {
    pub repo: &'a RepoReleaseFacts,
}

impl<'a> RepoReleaseInput<'a> {
    pub const fn new(repo: &'a RepoReleaseFacts) -> Self {
        Self { repo }
    }
}

pub struct PublishableCrateReleaseInput<'a> {
    pub krate: &'a PublishableCrateFacts,
}

impl<'a> PublishableCrateReleaseInput<'a> {
    pub const fn new(krate: &'a PublishableCrateFacts) -> Self {
        Self { krate }
    }
}

pub struct ReleaseEdgeInput<'a> {
    pub edge: &'a ReleaseEdgeFacts,
}

impl<'a> ReleaseEdgeInput<'a> {
    pub const fn new(edge: &'a ReleaseEdgeFacts) -> Self {
        Self { edge }
    }
}

pub struct ReleaseInputFailureInput<'a> {
    pub failure: &'a ReleaseInputFailureFacts,
}

impl<'a> ReleaseInputFailureInput<'a> {
    pub const fn new(failure: &'a ReleaseInputFailureFacts) -> Self {
        Self { failure }
    }
}
