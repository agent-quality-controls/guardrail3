use guardrail3_rs_toml_parser::types::{RustProfile, WaiverConfig};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3RsApparchLayer {
    Types,
    Logic,
    IoInbound,
    IoOutbound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3RsApparchDependencyKind {
    Dependency,
    DevDependency,
    BuildDependency,
    TargetDependency,
    TargetDevDependency,
    TargetBuildDependency,
}

impl G3RsApparchDependencyKind {
    #[must_use]
    pub const fn is_dev(self) -> bool {
        matches!(self, Self::DevDependency | Self::TargetDevDependency)
    }

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Dependency => "dependencies",
            Self::DevDependency => "dev-dependencies",
            Self::BuildDependency => "build-dependencies",
            Self::TargetDependency => "target.*.dependencies",
            Self::TargetDevDependency => "target.*.dev-dependencies",
            Self::TargetBuildDependency => "target.*.build-dependencies",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3RsApparchPatchKind {
    Patch,
    Replace,
}

impl G3RsApparchPatchKind {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Patch => "patch",
            Self::Replace => "replace",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum G3RsApparchRustPolicyState {
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
        allowed_deps: Vec<String>,
        waivers: Vec<WaiverConfig>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsApparchCrate {
    pub crate_name: String,
    pub cargo_rel_path: String,
    pub rel_dir: String,
    pub layer: Option<G3RsApparchLayer>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsApparchDependencyEdge {
    pub from_cargo_rel_path: String,
    pub to_cargo_rel_path: String,
    pub dep_name: String,
    pub kind: G3RsApparchDependencyKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsApparchExternalDependency {
    pub cargo_rel_path: String,
    pub dep_name: String,
    pub kind: G3RsApparchDependencyKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsApparchPatchBypass {
    pub cargo_rel_path: String,
    pub key: String,
    pub kind: G3RsApparchPatchKind,
    pub target_cargo_rel_path: String,
    pub target_rel_dir: String,
    pub target_layer: Option<G3RsApparchLayer>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3RsApparchPublicItemKind {
    Trait,
    FreeFunction,
    InherentMethod,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsApparchPublicItem {
    pub cargo_rel_path: String,
    pub rel_path: String,
    pub item_name: String,
    pub owner_name: Option<String>,
    pub kind: G3RsApparchPublicItemKind,
}

#[derive(Debug, Clone, PartialEq)]
pub struct G3RsApparchConfigChecksInput {
    pub crates: Vec<G3RsApparchCrate>,
    pub dependency_edges: Vec<G3RsApparchDependencyEdge>,
    pub external_dependencies: Vec<G3RsApparchExternalDependency>,
    pub patch_bypasses: Vec<G3RsApparchPatchBypass>,
    pub rust_policy: G3RsApparchRustPolicyState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsApparchSourceChecksInput {
    pub crates: Vec<G3RsApparchCrate>,
    pub public_items: Vec<G3RsApparchPublicItem>,
}
