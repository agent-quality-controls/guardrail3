use cargo_toml_parser::types::CargoToml;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum G3RsTopologyCargoManifestKind {
    Workspace,
    Package,
    Hybrid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum G3RsTopologyWorkspaceFamily {
    Toolchain,
    Fmt,
    Clippy,
    Deny,
    Cargo,
    Deps,
    Garde,
    Release,
    Test,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum G3RsTopologyWorkspaceFamilyFileKind {
    CargoToml,
    Guardrail3RsToml,
    RustfmtToml,
    DotRustfmtToml,
    RustToolchainToml,
    RustToolchainLegacy,
    ClippyToml,
    ClippyDotToml,
    CargoConfigToml,
    CargoConfigLegacy,
    DenyToml,
    DenyDotToml,
    CargoDenyToml,
    ReleasePlzToml,
    CliffToml,
    MutantsToml,
    NextestToml,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum G3RsTopologyWorkspaceFamilyFileAttachment {
    ExactRoot { root_rel: String },
    NestedUnderRoot { root_rel: String, owner_rel: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct G3RsTopologyDescendantCargoRoot {
    pub rel_dir: String,
    pub cargo_rel_path: String,
    pub manifest_kind: Option<G3RsTopologyCargoManifestKind>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct G3RsTopologyWorkspaceFamilyFile {
    pub family: G3RsTopologyWorkspaceFamily,
    pub rel_path: String,
    pub kind: G3RsTopologyWorkspaceFamilyFileKind,
    pub attachment: G3RsTopologyWorkspaceFamilyFileAttachment,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct G3RsTopologyFileTreeInputFailure {
    pub rel_path: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct G3RsTopologyNestedWorkspaceInput {
    pub rel_dir: String,
    pub cargo_rel_path: String,
    pub parent_workspace_rel: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct G3RsTopologyNestedGuardrail3RsTomlInput {
    pub rel_dir: String,
    pub guardrail3_rs_toml_rel_path: String,
    pub outer_adopted_unit_rel: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum G3RsTopologyWorkspaceMemberIssueKind {
    Undeclared {
        workspace_root_rel: String,
    },
    Extra {
        workspace_root_rel: String,
        member_pattern: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct G3RsTopologyWorkspaceMemberIssueInput {
    pub rel_dir: String,
    pub cargo_rel_path: String,
    pub kind: G3RsTopologyWorkspaceMemberIssueKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct G3RsTopologyEscapingWorkspaceMemberPathInput {
    pub cargo_rel_path: String,
    pub workspace_root_rel: String,
    pub member_pattern: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct G3RsTopologyIllegalFamilyFilePlacementInput {
    pub family: G3RsTopologyWorkspaceFamily,
    pub rel_path: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct G3RsTopologyFileTreeChecksInput {
    pub workspace_root_rel_dir: String,
    pub workspace_root_cargo_rel_path: String,
    pub workspace_manifest: CargoToml,
    pub descendant_cargo_roots: Vec<G3RsTopologyDescendantCargoRoot>,
    pub family_files: Vec<G3RsTopologyWorkspaceFamilyFile>,
    pub input_failures: Vec<G3RsTopologyFileTreeInputFailure>,
    pub nested_workspaces: Vec<G3RsTopologyNestedWorkspaceInput>,
    pub nested_guardrail3_rs_tomls: Vec<G3RsTopologyNestedGuardrail3RsTomlInput>,
    pub membership_issues: Vec<G3RsTopologyWorkspaceMemberIssueInput>,
    pub escaping_member_paths: Vec<G3RsTopologyEscapingWorkspaceMemberPathInput>,
    pub illegal_family_files: Vec<G3RsTopologyIllegalFamilyFilePlacementInput>,
}
