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
    pub pnpm_override_keys: Vec<String>,
    pub pnpm_only_built_dependencies: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsPackageLocalSnapshot {
    pub rel_path: String,
    pub dependencies: Vec<String>,
    pub dev_dependencies: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
pub struct G3TsPackageChecksInput {
    pub root: G3TsPackageRootState,
    pub locals: Vec<G3TsPackageLocalState>,
}
