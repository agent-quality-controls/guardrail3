use cargo_toml_parser::CargoToml;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3RsTopologyCargoManifestKind {
    Workspace,
    Package,
    Hybrid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3RsTopologyWorkspaceFamilyFileKind {
    CargoToml,
    GuardrailToml,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3RsTopologyWorkspaceFamilyFileAttachment {
    ExactRoot { root_rel: String },
    NestedUnderRoot { root_rel: String, owner_rel: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsTopologyDescendantCargoRoot {
    pub rel_dir: String,
    pub cargo_rel_path: String,
    pub manifest_kind: Option<G3RsTopologyCargoManifestKind>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsTopologyWorkspaceFamilyFile {
    pub family: G3RsTopologyWorkspaceFamily,
    pub rel_path: String,
    pub kind: G3RsTopologyWorkspaceFamilyFileKind,
    pub attachment: G3RsTopologyWorkspaceFamilyFileAttachment,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsTopologyFileTreeInputFailure {
    pub rel_path: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct G3RsTopologyFileTreeChecksInput {
    pub workspace_root_rel_dir: String,
    pub workspace_root_cargo_rel_path: String,
    pub workspace_manifest: CargoToml,
    pub descendant_cargo_roots: Vec<G3RsTopologyDescendantCargoRoot>,
    pub family_files: Vec<G3RsTopologyWorkspaceFamilyFile>,
    pub input_failures: Vec<G3RsTopologyFileTreeInputFailure>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct G3RsTopologyConfigChecksInput;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct G3RsTopologySourceChecksInput;
