use guardrail3_validation_model::RustValidateFamily;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RustFamilyFileKind {
    CargoToml,
    GuardrailToml,
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
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RustFamilyFileAttachment {
    ExactRoot {
        root_rel: String,
    },
    NestedUnderRoot {
        root_rel: String,
        owner_rel: String,
    },
    AncestorOfRoots {
        root_rels: Vec<String>,
        owner_rel: String,
    },
    OutsideRoots {
        owner_rel: String,
    },
}

impl RustFamilyFileAttachment {
    #[must_use]
    pub fn root_rel(&self) -> Option<&str> {
        match self {
            Self::ExactRoot { root_rel } | Self::NestedUnderRoot { root_rel, .. } => {
                Some(root_rel.as_str())
            }
            Self::AncestorOfRoots { .. } | Self::OutsideRoots { .. } => None,
        }
    }

    #[must_use]
    pub fn owner_rel(&self) -> &str {
        match self {
            Self::ExactRoot { root_rel } => root_rel,
            Self::NestedUnderRoot { owner_rel, .. } | Self::OutsideRoots { owner_rel } => owner_rel,
            Self::AncestorOfRoots { owner_rel, .. } => owner_rel,
        }
    }

    #[must_use]
    pub fn ancestor_root_rels(&self) -> Option<&[String]> {
        match self {
            Self::AncestorOfRoots { root_rels, .. } => Some(root_rels.as_slice()),
            Self::ExactRoot { .. } | Self::NestedUnderRoot { .. } | Self::OutsideRoots { .. } => {
                None
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RustFamilyFileFact {
    family: RustValidateFamily,
    rel_path: String,
    kind: RustFamilyFileKind,
    attachment: RustFamilyFileAttachment,
}

impl RustFamilyFileFact {
    #[must_use]
    pub fn new(
        family: RustValidateFamily,
        rel_path: String,
        kind: RustFamilyFileKind,
        attachment: RustFamilyFileAttachment,
    ) -> Self {
        Self {
            family,
            rel_path,
            kind,
            attachment,
        }
    }

    #[must_use]
    pub const fn family(&self) -> RustValidateFamily {
        self.family
    }

    #[must_use]
    pub fn rel_path(&self) -> &str {
        &self.rel_path
    }

    #[must_use]
    pub const fn kind(&self) -> RustFamilyFileKind {
        self.kind
    }

    #[must_use]
    pub fn attachment(&self) -> &RustFamilyFileAttachment {
        &self.attachment
    }

    #[must_use]
    pub fn logical_owner_rel(&self) -> &str {
        self.attachment.owner_rel()
    }

    #[must_use]
    pub fn nearest_rust_root_rel(&self) -> Option<&str> {
        self.attachment.root_rel()
    }

    #[must_use]
    pub fn ancestor_root_rels(&self) -> Option<&[String]> {
        self.attachment.ancestor_root_rels()
    }

    #[must_use]
    pub fn exact_rust_root_owner(&self) -> bool {
        matches!(self.attachment, RustFamilyFileAttachment::ExactRoot { .. })
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RustOwnedSurfaceFacts {
    family_files: Vec<RustFamilyFileFact>,
}

impl RustOwnedSurfaceFacts {
    #[must_use]
    pub fn new(family_files: Vec<RustFamilyFileFact>) -> Self {
        Self { family_files }
    }

    #[must_use]
    pub fn family_files(&self) -> &[RustFamilyFileFact] {
        &self.family_files
    }
}
