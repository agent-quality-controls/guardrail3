use super::facts::{
    ClippyConfigFacts, ClippyFacts, CoveredRustUnitFacts, PolicyRootKind, UncoveredRustUnitFacts,
};

pub struct ConfigClippyInput<'a> {
    pub config: &'a ClippyConfigFacts,
    pub profile_name: Option<&'a str>,
}

pub struct CoveredRustUnitInput<'a> {
    pub rel_dir: &'a str,
    pub kind: PolicyRootKind,
    pub covering_config_rel: &'a str,
}

pub struct UncoveredRustUnitInput<'a> {
    pub rel_dir: &'a str,
    pub kind: PolicyRootKind,
}

impl<'a> ConfigClippyInput<'a> {
    pub fn new(config: &'a ClippyConfigFacts, profile_name: Option<&'a str>) -> Self {
        Self {
            config,
            profile_name,
        }
    }

    pub fn from_facts(facts: &'a ClippyFacts) -> Vec<Self> {
        facts
            .allowed_configs
            .iter()
            .map(|config| Self::new(config, facts.profile_name.as_deref()))
            .collect()
    }
}

impl<'a> CoveredRustUnitInput<'a> {
    pub fn new(unit: &'a CoveredRustUnitFacts) -> Self {
        Self {
            rel_dir: &unit.rel_dir,
            kind: unit.kind,
            covering_config_rel: &unit.covering_config_rel,
        }
    }
}

impl<'a> UncoveredRustUnitInput<'a> {
    pub fn new(unit: &'a UncoveredRustUnitFacts) -> Self {
        Self {
            rel_dir: &unit.rel_dir,
            kind: unit.kind,
        }
    }
}
