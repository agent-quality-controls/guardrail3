use guardrail3_domain_config::types::EscapeHatchConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyRootKind {
    WorkspaceRoot,
    StandalonePackageRoot,
}

impl PolicyRootKind {
    pub const fn label(self) -> &'static str {
        match self {
            Self::WorkspaceRoot => "workspace root",
            Self::StandalonePackageRoot => "standalone package root",
        }
    }
}

#[derive(Debug, Clone)]
pub struct PolicyRootCargoFacts {
    pub(crate) kind: PolicyRootKind,
    pub(crate) rel_dir: String,
    pub(crate) cargo_rel_path: String,
    pub(crate) parsed: Option<toml::Value>,
    pub(crate) parse_error: Option<String>,
    pub(crate) guardrail_parse_error: bool,
    pub(crate) members_parse_error: bool,
    pub(crate) edition: Option<String>,
    pub(crate) edition_invalid: bool,
    pub(crate) rust_version: Option<String>,
    pub(crate) rust_version_invalid: bool,
    pub(crate) resolver: Option<String>,
    pub(crate) resolver_invalid: bool,
    pub(crate) profile_name: Option<String>,
    pub(crate) escape_hatches: Vec<EscapeHatchConfig>,
}

#[derive(Debug, Clone)]
pub struct WorkspaceMemberCargoFacts {
    pub(crate) workspace_root_rel: String,
    pub(crate) member_rel: String,
    pub(crate) cargo_rel_path: String,
    pub(crate) parsed: Option<toml::Value>,
    pub(crate) package_name: Option<String>,
    pub(crate) edition: Option<String>,
    pub(crate) edition_invalid: bool,
    pub(crate) lint_workspace_true: bool,
    pub(crate) parse_error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MissingMemberCargoFacts {
    pub(crate) workspace_root_rel: String,
    pub(crate) workspace_cargo_rel_path: String,
    pub(crate) member_rel: String,
}

#[derive(Debug, Clone)]
pub struct InputFailureFacts {
    pub(crate) rel_path: String,
    pub(crate) message: String,
}

#[derive(Debug, Clone, Default)]
pub struct CargoFamilyFacts {
    pub(crate) policy_roots: Vec<PolicyRootCargoFacts>,
    pub(crate) workspace_members: Vec<WorkspaceMemberCargoFacts>,
    pub(crate) missing_members: Vec<MissingMemberCargoFacts>,
    pub(crate) input_failures: Vec<InputFailureFacts>,
}
