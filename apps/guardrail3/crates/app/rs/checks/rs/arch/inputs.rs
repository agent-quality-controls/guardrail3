use super::super::rust_root_placement::RustRootClassification;
use super::facts::{
    ArchFacts, ArchInputFailureFacts, ArchRootFacts, GovernedRootFacts, ZoneOverlapFacts,
};

pub struct RootClassificationInput<'a> {
    pub root: &'a ArchRootFacts,
}

pub struct MisplacedRootInput<'a> {
    pub root: &'a ArchRootFacts,
    pub reporting_enabled: bool,
}

pub struct DualOwnershipInput<'a> {
    pub root: &'a ArchRootFacts,
}

pub struct ZoneOverlapInput<'a> {
    pub overlap: &'a ZoneOverlapFacts,
}

pub enum EnablementCoherenceInput<'a> {
    GovernedRoot(&'a GovernedRootFacts),
    InputFailure(&'a ArchInputFailureFacts),
}

impl<'a> RootClassificationInput<'a> {
    pub const fn new(root: &'a ArchRootFacts) -> Self {
        Self { root }
    }

    pub fn from_facts(facts: &'a ArchFacts) -> Vec<Self> {
        facts.roots.iter().map(Self::new).collect()
    }
}

impl<'a> MisplacedRootInput<'a> {
    pub const fn new(root: &'a ArchRootFacts, reporting_enabled: bool) -> Self {
        Self {
            root,
            reporting_enabled,
        }
    }

    pub fn from_facts(facts: &'a ArchFacts) -> Vec<Self> {
        facts
            .roots
            .iter()
            .filter(|root| root.classification == RustRootClassification::Other)
            .map(|root| Self::new(root, facts.misplaced_root_reporting_enabled))
            .collect()
    }
}

impl<'a> DualOwnershipInput<'a> {
    pub const fn new(root: &'a ArchRootFacts) -> Self {
        Self { root }
    }

    pub fn from_facts(facts: &'a ArchFacts) -> Vec<Self> {
        facts.roots.iter().map(Self::new).collect()
    }
}

impl<'a> ZoneOverlapInput<'a> {
    pub const fn new(overlap: &'a ZoneOverlapFacts) -> Self {
        Self { overlap }
    }

    pub fn from_facts(facts: &'a ArchFacts) -> Vec<Self> {
        facts.overlaps.iter().map(Self::new).collect()
    }
}

impl<'a> EnablementCoherenceInput<'a> {
    pub fn from_facts(facts: &'a ArchFacts) -> Vec<Self> {
        let mut cases = Vec::new();
        cases.extend(
            facts
                .governed_roots
                .iter()
                .map(EnablementCoherenceInput::GovernedRoot),
        );
        cases.extend(
            facts
                .input_failures
                .iter()
                .map(EnablementCoherenceInput::InputFailure),
        );
        cases
    }
}
