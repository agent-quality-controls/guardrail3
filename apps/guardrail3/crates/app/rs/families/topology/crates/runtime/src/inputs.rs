use guardrail3_app_rs_placement::RustRootClassification;

use super::facts::{
    GovernedRootFacts, IllegalFamilyFileFacts, TopologyFacts, TopologyInputFailureFacts,
    TopologyRootFacts,
    TopologyIssueFacts, ZoneOverlapFacts,
};

pub struct RootClassificationInput<'a> {
    pub(crate) root: &'a TopologyRootFacts,
}

pub struct MisplacedRootInput<'a> {
    pub(crate) root: &'a TopologyRootFacts,
    pub(crate) reporting_enabled: bool,
}

pub struct AuxiliaryRootInput<'a> {
    pub(crate) root: &'a TopologyRootFacts,
}

pub struct DualOwnershipInput<'a> {
    pub(crate) root: &'a TopologyRootFacts,
}

pub struct ZoneOverlapInput<'a> {
    pub(crate) overlap: &'a ZoneOverlapFacts,
}

pub struct ScopedTopologyConfigInput<'a> {
    pub(crate) failure: &'a TopologyInputFailureFacts,
}

pub struct OwnerFamilyCoherenceInput<'a> {
    pub(crate) root: &'a GovernedRootFacts,
}

pub struct RequiredInputFailureInput<'a> {
    pub(crate) failure: &'a TopologyInputFailureFacts,
}

pub struct IllegalFamilyFilePlacementInput<'a> {
    pub(crate) file: &'a IllegalFamilyFileFacts,
}

pub struct TopologyIssueInput<'a> {
    pub(crate) issue: &'a TopologyIssueFacts,
}

impl<'a> RootClassificationInput<'a> {
    pub const fn new(root: &'a TopologyRootFacts) -> Self {
        Self { root }
    }

    pub fn from_facts(facts: &'a TopologyFacts) -> Vec<Self> {
        facts.roots.iter().map(Self::new).collect()
    }
}

impl<'a> MisplacedRootInput<'a> {
    pub const fn new(root: &'a TopologyRootFacts, reporting_enabled: bool) -> Self {
        Self {
            root,
            reporting_enabled,
        }
    }

    pub fn from_facts(facts: &'a TopologyFacts) -> Vec<Self> {
        facts
            .roots
            .iter()
            .filter(|root| root.classification == RustRootClassification::Other)
            .map(|root| Self::new(root, facts.misplaced_root_reporting_enabled))
            .collect()
    }
}

impl<'a> AuxiliaryRootInput<'a> {
    pub const fn new(root: &'a TopologyRootFacts) -> Self {
        Self { root }
    }

    pub fn from_facts(facts: &'a TopologyFacts) -> Vec<Self> {
        facts
            .roots
            .iter()
            .filter(|root| root.classification == RustRootClassification::Auxiliary)
            .map(Self::new)
            .collect()
    }
}

impl<'a> DualOwnershipInput<'a> {
    pub const fn new(root: &'a TopologyRootFacts) -> Self {
        Self { root }
    }

    pub fn from_facts(facts: &'a TopologyFacts) -> Vec<Self> {
        facts.roots.iter().map(Self::new).collect()
    }
}

impl<'a> ZoneOverlapInput<'a> {
    pub const fn new(overlap: &'a ZoneOverlapFacts) -> Self {
        Self { overlap }
    }

    pub fn from_facts(facts: &'a TopologyFacts) -> Vec<Self> {
        facts.overlaps.iter().map(Self::new).collect()
    }
}

impl<'a> ScopedTopologyConfigInput<'a> {
    pub fn from_facts(facts: &'a TopologyFacts) -> Vec<Self> {
        facts
            .input_failures
            .iter()
            .filter(|failure| {
                matches!(
                    failure.kind,
                    super::facts::TopologyInputFailureKind::ScopedTopologyConfig
                )
            })
            .map(|failure| Self { failure })
            .collect()
    }
}

impl<'a> OwnerFamilyCoherenceInput<'a> {
    pub fn from_facts(facts: &'a TopologyFacts) -> Vec<Self> {
        facts
            .governed_roots
            .iter()
            .map(|root| Self { root })
            .collect()
    }
}

impl<'a> RequiredInputFailureInput<'a> {
    pub fn from_facts(facts: &'a TopologyFacts) -> Vec<Self> {
        facts
            .input_failures
            .iter()
            .filter(|failure| {
                matches!(
                    failure.kind,
                    super::facts::TopologyInputFailureKind::RequiredInput
                )
            })
            .map(|failure| Self { failure })
            .collect()
    }
}

impl<'a> IllegalFamilyFilePlacementInput<'a> {
    pub fn from_facts(facts: &'a TopologyFacts) -> Vec<Self> {
        facts
            .illegal_family_files
            .iter()
            .map(|file| Self { file })
            .collect()
    }
}

impl<'a> TopologyIssueInput<'a> {
    pub fn from_facts(facts: &'a TopologyFacts) -> Vec<Self> {
        facts
            .topology_issues
            .iter()
            .map(|issue| Self { issue })
            .collect()
    }
}
