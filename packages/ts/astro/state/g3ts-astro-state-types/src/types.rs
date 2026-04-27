#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroStateAppRootInput {
    pub app_root_rel_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroStateStrictAppRootInput {
    pub app_root_rel_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroStateLegacyGeneratedPathInput {
    pub app_root_rel_path: String,
    pub rel_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroStateForbiddenPathInput {
    pub app_root_rel_path: String,
    pub rel_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroStatePolicySnapshot {
    pub rel_path: String,
    pub forbidden_state: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsAstroStatePolicySurfaceState {
    Missing {
        rel_path: String,
    },
    Unreadable {
        rel_path: String,
        reason: String,
    },
    ParseError {
        rel_path: String,
        reason: String,
    },
    MissingAstroPolicy {
        rel_path: String,
    },
    Parsed {
        snapshot: G3TsAstroStatePolicySnapshot,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroStateFileTreeChecksInput {
    pub strict_app_roots: Vec<G3TsAstroStateStrictAppRootInput>,
    pub legacy_generated_paths: Vec<G3TsAstroStateLegacyGeneratedPathInput>,
    pub forbidden_state_paths: Vec<G3TsAstroStateForbiddenPathInput>,
}
