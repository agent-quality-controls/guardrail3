use super::facts::{
    CoveredRustUnitFacts, DenyConfigFacts, DenyFacts, SameRootConflictFacts, UncoveredRustUnitFacts,
};

pub struct ConfigDenyInput<'a> {
    pub(crate) config: &'a DenyConfigFacts,
}

pub struct CoveredRustUnitInput<'a> {
    pub(crate) covered: &'a CoveredRustUnitFacts,
}

pub struct SameRootConflictInput<'a> {
    pub(crate) conflict: &'a SameRootConflictFacts,
}

pub struct UncoveredRustUnitInput<'a> {
    pub(crate) uncovered: &'a UncoveredRustUnitFacts,
}

impl<'a> ConfigDenyInput<'a> {
    pub fn from_facts(facts: &'a DenyFacts) -> Vec<Self> {
        facts
            .linted_configs
            .iter()
            .map(|config| Self { config })
            .collect()
    }
}

impl<'a> CoveredRustUnitInput<'a> {
    pub const fn new(covered: &'a CoveredRustUnitFacts) -> Self {
        Self { covered }
    }
}

impl<'a> SameRootConflictInput<'a> {
    pub const fn new(conflict: &'a SameRootConflictFacts) -> Self {
        Self { conflict }
    }
}

impl<'a> UncoveredRustUnitInput<'a> {
    pub const fn new(uncovered: &'a UncoveredRustUnitFacts) -> Self {
        Self { uncovered }
    }
}
