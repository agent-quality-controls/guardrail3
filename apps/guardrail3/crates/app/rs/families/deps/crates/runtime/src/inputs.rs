use super::facts::{InputFailureFacts, LockfileFacts, ToolFacts};

pub struct ToolDepsInput<'a> {
    pub(crate) tool: &'a ToolFacts,
}

impl<'a> ToolDepsInput<'a> {
    pub fn new(tool: &'a ToolFacts) -> Self {
        Self { tool }
    }
}

pub struct LockfileDepsInput<'a> {
    pub(crate) lockfile: &'a LockfileFacts,
}

impl<'a> LockfileDepsInput<'a> {
    pub fn new(lockfile: &'a LockfileFacts) -> Self {
        Self { lockfile }
    }
}

pub struct InputFailureDepsInput<'a> {
    pub(crate) failure: &'a InputFailureFacts,
}

impl<'a> InputFailureDepsInput<'a> {
    pub fn new(failure: &'a InputFailureFacts) -> Self {
        Self { failure }
    }
}
