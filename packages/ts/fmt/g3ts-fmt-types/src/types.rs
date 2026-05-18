#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsFmtPackageSurfaceSnapshot {
    pub rel_path: String,
    pub name: Option<String>,
    pub dependencies: Vec<String>,
    pub dev_dependencies: Vec<String>,
    pub dependency_declarations: Vec<G3TsFmtDependencyDeclarationSnapshot>,
    pub script_names: Vec<String>,
    pub script_tool_invocations: Vec<G3TsFmtPackageScriptToolInvocation>,
    pub script_parse_blockers: Vec<G3TsFmtPackageScriptParseBlocker>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsFmtDependencyDeclarationSnapshot {
    pub name: String,
    pub lane: String,
    pub specifier_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsFmtPackageScriptToolInvocation {
    pub script_name: String,
    pub invocation: String,
    pub executable: String,
    pub args: Vec<String>,
    pub preceded_by: Option<G3TsFmtPackageScriptCommandSeparator>,
    pub followed_by: Option<G3TsFmtPackageScriptCommandSeparator>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3TsFmtPackageScriptCommandSeparator {
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsFmtPackageScriptParseBlocker {
    pub script_name: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsFmtPackageSurfaceState {
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
        snapshot: G3TsFmtPackageSurfaceSnapshot,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsFmtConfigSurfaceState {
    Missing { rel_path: String },
    Unreadable { rel_path: String, reason: String },
    ParseError { rel_path: String, reason: String },
    Parsed { rel_path: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsFmtSyncpackSnapshot {
    pub rel_path: String,
    pub source: Vec<String>,
    pub version_groups: Vec<syncpack_config_parser::types::SyncpackVersionGroup>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsFmtSyncpackSurfaceState {
    Missing { rel_path: String },
    Unreadable { rel_path: String, reason: String },
    ParseError { rel_path: String, reason: String },
    Parsed { snapshot: G3TsFmtSyncpackSnapshot },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsFmtContractInput {
    pub app_root_rel_path: String,
    pub package: G3TsFmtPackageSurfaceState,
    pub prettier_config: G3TsFmtConfigSurfaceState,
    pub syncpack_config: G3TsFmtSyncpackSurfaceState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsFmtConfigChecksInput {
    pub contracts: Vec<G3TsFmtContractInput>,
}
