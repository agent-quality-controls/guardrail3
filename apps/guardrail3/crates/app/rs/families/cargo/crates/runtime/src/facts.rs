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
    pub kind: PolicyRootKind,
    pub rel_dir: String,
    pub cargo_rel_path: String,
    pub parsed: Option<toml::Value>,
    pub parse_error: Option<String>,
    pub edition: Option<String>,
    pub rust_version: Option<String>,
    pub resolver: Option<String>,
    pub profile_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct WorkspaceMemberCargoFacts {
    pub workspace_root_rel: String,
    pub member_rel: String,
    pub cargo_rel_path: String,
    pub parsed: Option<toml::Value>,
    pub package_name: Option<String>,
    pub edition: Option<String>,
    pub lint_workspace_true: bool,
    pub parse_error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MissingMemberCargoFacts {
    pub workspace_root_rel: String,
    pub workspace_cargo_rel_path: String,
    pub member_rel: String,
}

#[derive(Debug, Clone)]
pub struct InputFailureFacts {
    pub rel_path: String,
    pub message: String,
}

#[derive(Debug, Clone, Default)]
pub struct CargoFamilyFacts {
    pub policy_roots: Vec<PolicyRootCargoFacts>,
    pub workspace_members: Vec<WorkspaceMemberCargoFacts>,
    pub missing_members: Vec<MissingMemberCargoFacts>,
    pub input_failures: Vec<InputFailureFacts>,
}
