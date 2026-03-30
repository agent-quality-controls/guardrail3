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
pub struct ToolchainPolicyRootFacts {
    pub(crate) kind: PolicyRootKind,
    pub(crate) rel_dir: String,
    pub(crate) cargo_rel_path: String,
    pub(crate) toolchain_toml_rel: Option<String>,
    pub(crate) legacy_toolchain_rel: Option<String>,
    pub(crate) parsed: Option<toml::Value>,
    pub(crate) parse_error: Option<String>,
    pub(crate) cargo_rust_version: Option<String>,
    pub(crate) cargo_rust_version_invalid: bool,
    pub(crate) cargo_parse_error: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ToolchainFamilyFacts {
    pub(crate) policy_roots: Vec<ToolchainPolicyRootFacts>,
}
