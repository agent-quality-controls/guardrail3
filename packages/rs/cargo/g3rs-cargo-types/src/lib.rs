use cargo_toml_parser::CargoToml;
use toml::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum G3RsCargoPolicyRootKind {
    WorkspaceRoot,
    StandalonePackageRoot,
    Other,
}

impl G3RsCargoPolicyRootKind {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::WorkspaceRoot => "workspace root",
            Self::StandalonePackageRoot => "standalone package root",
            Self::Other => "Cargo manifest",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsCargoEscapeHatch {
    pub family: String,
    pub file: String,
    pub kind: String,
    pub selector: String,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct G3RsCargoPolicyRoot {
    pub kind: G3RsCargoPolicyRootKind,
    pub rel_dir: String,
    pub cargo_rel_path: String,
    pub cargo: CargoToml,
    pub raw_cargo: Value,
    pub guardrail_rel_path: Option<String>,
    pub profile_name: Option<String>,
    pub escape_hatches: Vec<G3RsCargoEscapeHatch>,
    pub guardrail_parse_error: bool,
    pub edition: Option<String>,
    pub edition_invalid: bool,
    pub rust_version: Option<String>,
    pub rust_version_invalid: bool,
}

#[derive(Debug, Clone)]
pub struct G3RsCargoWorkspaceMember {
    pub workspace_root_rel: String,
    pub member_rel: String,
    pub cargo_rel_path: String,
    pub raw_cargo: Value,
    pub package_name: Option<String>,
    pub edition: Option<String>,
    pub edition_invalid: bool,
    pub lint_workspace_invalid: bool,
    pub lint_workspace_true: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsCargoMissingMember {
    pub workspace_root_rel: String,
    pub workspace_cargo_rel_path: String,
    pub member_rel: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsCargoInputFailure {
    pub rel_path: String,
    pub message: String,
}

/// Input contract for extracted Cargo config checks.
///
/// Ingestion owns crawl selection, root/member discovery, policy-file parsing,
/// and normalization into one root plus zero or more workspace members.
#[derive(Debug, Clone)]
pub struct G3RsCargoConfigChecksInput {
    pub root: G3RsCargoPolicyRoot,
    pub workspace_members: Vec<G3RsCargoWorkspaceMember>,
}

/// Placeholder input contract for future Cargo source checks.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct G3RsCargoSourceChecksInput;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsCargoFileTreeRoot {
    pub kind: Option<G3RsCargoPolicyRootKind>,
    pub rel_dir: String,
    pub cargo_rel_path: String,
    pub guardrail_rel_path: Option<String>,
    pub members_parse_error: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsCargoFileTreeChecksInput {
    pub root: G3RsCargoFileTreeRoot,
    pub missing_members: Vec<G3RsCargoMissingMember>,
    pub input_failures: Vec<G3RsCargoInputFailure>,
}
