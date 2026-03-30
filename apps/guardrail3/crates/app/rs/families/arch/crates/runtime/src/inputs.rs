use guardrail3_app_rs_placement::RustRootClassification;

use super::facts::{
    ArchFacts, ArchInputFailureFacts, ArchRootFacts, GovernedRootFacts, ZoneOverlapFacts,
};

pub struct RootClassificationInput<'a> {
    pub(crate) root: &'a ArchRootFacts,
}

pub struct MisplacedRootInput<'a> {
    pub(crate) root: &'a ArchRootFacts,
    pub(crate) reporting_enabled: bool,
}

pub struct AuxiliaryRootInput<'a> {
    pub(crate) root: &'a ArchRootFacts,
}

pub struct DualOwnershipInput<'a> {
    pub(crate) root: &'a ArchRootFacts,
}

pub struct ZoneOverlapInput<'a> {
    pub(crate) overlap: &'a ZoneOverlapFacts,
}

pub struct ScopedArchConfigInput<'a> {
    pub(crate) failure: &'a ArchInputFailureFacts,
}

pub struct OwnerFamilyCoherenceInput<'a> {
    pub(crate) root: &'a GovernedRootFacts,
}

pub struct RequiredInputFailureInput<'a> {
    pub(crate) failure: &'a ArchInputFailureFacts,
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

impl<'a> AuxiliaryRootInput<'a> {
    pub const fn new(root: &'a ArchRootFacts) -> Self {
        Self { root }
    }

    pub fn from_facts(facts: &'a ArchFacts) -> Vec<Self> {
        facts
            .roots
            .iter()
            .filter(|root| root.classification == RustRootClassification::Auxiliary)
            .map(Self::new)
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

impl<'a> ScopedArchConfigInput<'a> {
    pub fn from_facts(facts: &'a ArchFacts) -> Vec<Self> {
        facts
            .input_failures
            .iter()
            .filter(|failure| {
                matches!(
                    failure.kind,
                    super::facts::ArchInputFailureKind::ScopedArchConfig
                )
            })
            .map(|failure| Self { failure })
            .collect()
    }
}

impl<'a> OwnerFamilyCoherenceInput<'a> {
    pub fn from_facts(facts: &'a ArchFacts) -> Vec<Self> {
        facts
            .governed_roots
            .iter()
            .map(|root| Self { root })
            .collect()
    }
}

impl<'a> RequiredInputFailureInput<'a> {
    pub fn from_facts(facts: &'a ArchFacts) -> Vec<Self> {
        facts
            .input_failures
            .iter()
            .filter(|failure| {
                matches!(
                    failure.kind,
                    super::facts::ArchInputFailureKind::RequiredInput
                )
            })
            .map(|failure| Self { failure })
            .collect()
    }
}
