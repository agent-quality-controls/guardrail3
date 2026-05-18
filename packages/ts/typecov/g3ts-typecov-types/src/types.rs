#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsTypecovPackageSurfaceSnapshot {
    pub rel_path: String,
    pub name: Option<String>,
    pub dependencies: Vec<String>,
    pub dev_dependencies: Vec<String>,
    pub dependency_declarations: Vec<G3TsTypecovDependencyDeclarationSnapshot>,
    pub script_names: Vec<String>,
    pub script_tool_invocations: Vec<G3TsTypecovPackageScriptToolInvocation>,
    pub script_parse_blockers: Vec<G3TsTypecovPackageScriptParseBlocker>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsTypecovDependencyDeclarationSnapshot {
    pub name: String,
    pub lane: String,
    pub specifier_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsTypecovPackageScriptToolInvocation {
    pub script_name: String,
    pub invocation: String,
    pub executable: String,
    pub args: Vec<String>,
    pub preceded_by: Option<G3TsTypecovPackageScriptCommandSeparator>,
    pub followed_by: Option<G3TsTypecovPackageScriptCommandSeparator>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3TsTypecovPackageScriptCommandSeparator {
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsTypecovPackageScriptParseBlocker {
    pub script_name: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsTypecovPackageSurfaceState {
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
        snapshot: G3TsTypecovPackageSurfaceSnapshot,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsTypecovPolicySnapshot {
    pub rel_path: String,
    pub minimum: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsTypecovPolicySurfaceState {
    Missing { rel_path: String },
    Unreadable { rel_path: String, reason: String },
    ParseError { rel_path: String, reason: String },
    MissingTypecovPolicy { rel_path: String },
    Parsed { snapshot: G3TsTypecovPolicySnapshot },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsTypecovSyncpackSnapshot {
    pub rel_path: String,
    pub source: Vec<String>,
    pub version_groups: Vec<syncpack_config_parser::types::SyncpackVersionGroup>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsTypecovSyncpackSurfaceState {
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
        snapshot: G3TsTypecovSyncpackSnapshot,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsTypecovContractInput {
    pub app_root_rel_path: String,
    pub package: G3TsTypecovPackageSurfaceState,
    pub typecov_policy: G3TsTypecovPolicySurfaceState,
    pub syncpack_config: G3TsTypecovSyncpackSurfaceState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsTypecovConfigChecksInput {
    pub contracts: Vec<G3TsTypecovContractInput>,
}
