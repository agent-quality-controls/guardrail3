use super::facts::{
    ClippyConfigFacts, ClippyFacts, CoveredRustUnitFacts, PolicyRootKind, UncoveredRustUnitFacts,
};

pub struct ConfigClippyInput<'a> {
    pub config: &'a ClippyConfigFacts,
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
    pub const fn new(config: &'a ClippyConfigFacts) -> Self {
        Self { config }
    }

    pub fn from_facts(facts: &'a ClippyFacts) -> Vec<Self> {
        facts.allowed_configs.iter().map(Self::new).collect()
    }

    pub fn profile_name(&self) -> Option<&'a str> {
        self.config.profile_name.as_deref()
    }

    pub const fn garde_enabled(&self) -> bool {
        self.config.garde_enabled
    }

    pub const fn package_publishable(&self) -> bool {
        self.config.package_publishable
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
