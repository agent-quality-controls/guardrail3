use cargo_toml_parser::types::CargoTomlDocument;
use g3_guardrail_toml_types::WaiverConfig;
use g3rs_toml_parser::types::RustProfile;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum G3RsCargoRustPolicyState {
    Missing,
    Unreadable {
        rel_path: String,
        reason: String,
    },
    ParseError {
        rel_path: String,
        reason: String,
    },
    Parsed {
        rel_path: String,
        profile: Option<RustProfile>,
        waivers: Vec<WaiverConfig>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum G3RsCargoConfigTomlState {
    Missing,
    Unreadable {
        rel_path: String,
        reason: String,
    },
    ParseError {
        rel_path: String,
        reason: String,
    },
    Parsed {
        rel_path: String,
        incompatible_rust_versions: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize)]
pub struct G3RsCargoPolicyRoot {
    pub kind: G3RsCargoPolicyRootKind,
    pub rel_dir: String,
    pub cargo_rel_path: String,
    pub cargo: CargoTomlDocument,
    pub rust_policy: G3RsCargoRustPolicyState,
    pub cargo_config: G3RsCargoConfigTomlState,
}

#[derive(Debug, Clone, Serialize)]
pub struct G3RsCargoWorkspaceMember {
    pub workspace_root_rel: String,
    pub member_rel: String,
    pub cargo_rel_path: String,
    pub cargo: CargoTomlDocument,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct G3RsCargoMissingMember {
    pub workspace_root_rel: String,
    pub workspace_cargo_rel_path: String,
    pub member_rel: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct G3RsCargoInputFailure {
    pub rel_path: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct G3RsCargoConfigChecksInput {
    pub root: G3RsCargoPolicyRoot,
    pub workspace_members: Vec<G3RsCargoWorkspaceMember>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct G3RsCargoSourceChecksInput;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct G3RsCargoFileTreeRoot {
    pub kind: Option<G3RsCargoPolicyRootKind>,
    pub rel_dir: String,
    pub cargo_rel_path: String,
    pub rust_policy_rel_path: Option<String>,
    pub members_parse_error: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct G3RsCargoFileTreeChecksInput {
    pub root: G3RsCargoFileTreeRoot,
    pub missing_members: Vec<G3RsCargoMissingMember>,
    pub input_failures: Vec<G3RsCargoInputFailure>,
}
