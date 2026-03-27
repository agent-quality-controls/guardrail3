use super::facts::{
    AllowlistCoverageFacts, DependencyEntryFacts, InputFailureFacts, LockfileFacts, ToolFacts,
};

pub struct ToolDepsInput<'a> {
    pub tool: &'a ToolFacts,
}

impl<'a> ToolDepsInput<'a> {
    pub fn new(tool: &'a ToolFacts) -> Self {
        Self { tool }
    }
}

pub struct DependencyEntryDepsInput<'a> {
    pub entry: &'a DependencyEntryFacts,
}

impl<'a> DependencyEntryDepsInput<'a> {
    pub fn new(entry: &'a DependencyEntryFacts) -> Self {
        Self { entry }
    }
}

pub struct AllowlistCoverageDepsInput<'a> {
    pub coverage: &'a AllowlistCoverageFacts,
}

impl<'a> AllowlistCoverageDepsInput<'a> {
    pub fn new(coverage: &'a AllowlistCoverageFacts) -> Self {
        Self { coverage }
    }
}

pub struct LockfileDepsInput<'a> {
    pub lockfile: &'a LockfileFacts,
}

impl<'a> LockfileDepsInput<'a> {
    pub fn new(lockfile: &'a LockfileFacts) -> Self {
        Self { lockfile }
    }
}

pub struct InputFailureDepsInput<'a> {
    pub failure: &'a InputFailureFacts,
}

impl<'a> InputFailureDepsInput<'a> {
    pub fn new(failure: &'a InputFailureFacts) -> Self {
        Self { failure }
    }
}
