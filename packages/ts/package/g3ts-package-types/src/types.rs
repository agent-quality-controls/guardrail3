#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsPackageRootSnapshot {
    pub rel_path: String,
    pub private_field: Option<bool>,
    pub package_manager: Option<String>,
    pub engines_node: Option<String>,
    pub engines_pnpm: Option<String>,
    pub preinstall_script: Option<String>,
    pub prepare_script: Option<String>,
    pub lint_script: Option<String>,
    pub typecheck_script: Option<String>,
    pub validate_script: Option<String>,
    pub dependencies: Vec<String>,
    pub dev_dependencies: Vec<String>,
    pub pnpm_override_keys: Vec<String>,
    pub pnpm_only_built_dependencies: Vec<String>,
    pub script_commands: Vec<G3TsPackageScriptCommand>,
    pub script_tool_invocations: Vec<G3TsPackageScriptToolInvocation>,
    pub script_parse_blockers: Vec<G3TsPackageScriptParseBlocker>,
    pub safely_runs_only_allow_pnpm: bool,
    pub safely_runs_syncpack_lint: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsPackageScriptCommand {
    pub script_name: String,
    pub invocation: String,
    pub executable: String,
    pub args: Vec<String>,
    pub preceded_by: Option<G3TsPackageScriptCommandSeparator>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsPackageScriptToolInvocation {
    pub script_name: String,
    pub command_index: usize,
    pub invocation: String,
    pub executable: String,
    pub args: Vec<String>,
    pub preceded_by: Option<G3TsPackageScriptCommandSeparator>,
    pub followed_by: Option<G3TsPackageScriptCommandSeparator>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3TsPackageScriptCommandSeparator {
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsPackageScriptParseBlocker {
    pub script_name: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsPackageLocalSnapshot {
    pub rel_path: String,
    pub dependencies: Vec<String>,
    pub dev_dependencies: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[expect(
    clippy::large_enum_variant,
    reason = "Parsed snapshot is the dominant runtime variant; boxing would force \
              construction-site changes across consumer crates outside this workspace and \
              the snapshot is short-lived at the call site"
)]
pub enum G3TsPackageRootState {
    NotPackageManagerRoot,
    Missing,
    Unreadable { rel_path: String, reason: String },
    ParseError { rel_path: String, reason: String },
    Parsed { snapshot: G3TsPackageRootSnapshot },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsPackageLocalState {
    Unreadable { rel_path: String, reason: String },
    ParseError { rel_path: String, reason: String },
    Parsed { snapshot: G3TsPackageLocalSnapshot },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsPackageSyncpackConfigSnapshot {
    pub rel_path: String,
    pub missing_source_entries: Vec<String>,
    pub missing_forbidden_bans: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsPackageSyncpackConfigState {
    NotRequired,
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
        snapshot: G3TsPackageSyncpackConfigSnapshot,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsPackageChecksInput {
    pub root: G3TsPackageRootState,
    pub locals: Vec<G3TsPackageLocalState>,
    pub syncpack_config: G3TsPackageSyncpackConfigState,
    pub forbidden_syncpack_deps: Vec<String>,
}
