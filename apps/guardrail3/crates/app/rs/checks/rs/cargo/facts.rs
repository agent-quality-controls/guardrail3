use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct WorkspaceCargoFacts {
    pub rel_path: String,
    pub parsed: Option<toml::Value>,
    pub declared_members: BTreeSet<String>,
    pub workspace_edition: Option<String>,
    pub workspace_rust_version: Option<String>,
    pub resolver: Option<String>,
    pub has_package: bool,
    pub profile_name: Option<String>,
    pub parse_error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MemberCargoFacts {
    pub rel_path: String,
    pub member_rel: String,
    pub parsed: Option<toml::Value>,
    pub package_name: Option<String>,
    pub edition: Option<String>,
    pub lint_workspace_true: bool,
    pub parse_error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CargoFamilyFacts {
    pub workspace: WorkspaceCargoFacts,
    pub members: Vec<MemberCargoFacts>,
    pub discovered_member_rels: BTreeSet<String>,
}
