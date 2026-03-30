use super::facts::{
    AllowlistCoverageFacts, DependencyEntryFacts, DirectDependencyCapFacts, InputFailureFacts,
    LockfileFacts, ToolFacts,
};

pub struct ToolDepsInput<'a> {
    pub(crate) tool: &'a ToolFacts,
}

impl<'a> ToolDepsInput<'a> {
    pub fn new(tool: &'a ToolFacts) -> Self {
        Self { tool }
    }
}

pub struct DependencyEntryDepsInput<'a> {
    pub(crate) entry: &'a DependencyEntryFacts,
}

impl<'a> DependencyEntryDepsInput<'a> {
    pub fn new(entry: &'a DependencyEntryFacts) -> Self {
        Self { entry }
    }
}

pub struct DirectDependencyCapDepsInput<'a> {
    pub(crate) cap: &'a DirectDependencyCapFacts,
}

impl<'a> DirectDependencyCapDepsInput<'a> {
    pub fn new(cap: &'a DirectDependencyCapFacts) -> Self {
        Self { cap }
    }
}

pub struct AllowlistCoverageDepsInput<'a> {
    pub(crate) coverage: &'a AllowlistCoverageFacts,
}

impl<'a> AllowlistCoverageDepsInput<'a> {
    pub fn new(coverage: &'a AllowlistCoverageFacts) -> Self {
        Self { coverage }
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
