use super::facts::{
    CargoConfigOverrideFacts, ClippyConfigFacts, ClippyFacts, CoveredRustUnitFacts, PolicyRootKind,
    UncoveredRustUnitFacts,
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

pub struct PolicyContextFailureInput<'a> {
    pub parse_error: &'a str,
}

pub struct CargoConfigOverrideInput<'a> {
    pub rel_path: &'a str,
    pub parse_error: Option<&'a str>,
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

    pub const fn published_library_policy(&self) -> bool {
        self.config.published_library_policy
    }

    pub fn policy_context_parse_error(&self) -> Option<&'a str> {
        self.config.policy_context_parse_error.as_deref()
    }
}

impl<'a> PolicyContextFailureInput<'a> {
    pub const fn new(parse_error: &'a str) -> Self {
        Self { parse_error }
    }
}

impl<'a> CargoConfigOverrideInput<'a> {
    pub fn new(facts: &'a CargoConfigOverrideFacts) -> Self {
        Self {
            rel_path: &facts.rel_path,
            parse_error: facts.parse_error.as_deref(),
        }
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
