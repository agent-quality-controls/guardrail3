#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsSpellingPackageSurfaceSnapshot {
    pub rel_path: String,
    pub dependencies: Vec<String>,
    pub dev_dependencies: Vec<String>,
    pub script_names: Vec<String>,
    pub script_tool_invocations: Vec<G3TsSpellingPackageScriptToolInvocation>,
    pub script_parse_blockers: Vec<G3TsSpellingPackageScriptParseBlocker>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsSpellingPackageScriptToolInvocation {
    pub script_name: String,
    pub executable: String,
    pub args: Vec<String>,
    pub preceded_by: Option<G3TsSpellingPackageScriptCommandSeparator>,
    pub followed_by: Option<G3TsSpellingPackageScriptCommandSeparator>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3TsSpellingPackageScriptCommandSeparator {
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsSpellingPackageScriptParseBlocker {
    pub script_name: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsSpellingPackageSurfaceState {
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
    Parsed {
        snapshot: G3TsSpellingPackageSurfaceSnapshot,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsSpellingConfigSurfaceState {
    Missing { rel_path: String },
    Unreadable { rel_path: String, reason: String },
    ParseError { rel_path: String, reason: String },
    Parsed { rel_path: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsSpellingSyncpackSnapshot {
    pub rel_path: String,
    pub source: Vec<String>,
    pub version_groups: Vec<G3TsSpellingSyncpackVersionGroupSnapshot>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsSpellingSyncpackVersionGroupSnapshot {
    pub dependencies: Vec<String>,
    pub dependency_types: Vec<String>,
    pub packages: Option<Vec<String>>,
    pub specifier_types: Option<Vec<String>>,
    pub is_ignored: Option<bool>,
    pub is_banned: Option<bool>,
    pub pin_version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsSpellingSyncpackSurfaceState {
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
    Parsed {
        snapshot: G3TsSpellingSyncpackSnapshot,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsSpellingContractInput {
    pub app_root_rel_path: String,
    pub package: G3TsSpellingPackageSurfaceState,
    pub cspell_config: G3TsSpellingConfigSurfaceState,
    pub syncpack_config: G3TsSpellingSyncpackSurfaceState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsSpellingConfigChecksInput {
    pub contracts: Vec<G3TsSpellingContractInput>,
}
