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
pub struct G3RsApparchBoundDependency {
    pub dep_name: String,
    pub kind: G3RsApparchDependencyKind,
    pub target: G3RsApparchCrate,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsApparchIoTraitsSourceChecksInput {
    pub krate: G3RsApparchCrate,
    pub public_traits: Vec<G3RsApparchPublicItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsApparchTypesPublicSurfaceChecksInput {
    pub krate: G3RsApparchCrate,
    pub public_behavior_items: Vec<G3RsApparchPublicItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct G3RsApparchCrateDependencyChecksInput {
    pub krate: G3RsApparchCrate,
    pub internal_dependencies: Vec<G3RsApparchBoundDependency>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct G3RsApparchCratePurityChecksInput {
    pub krate: G3RsApparchCrate,
    pub external_dependencies: Vec<G3RsApparchExternalDependency>,
    pub rust_policy: G3RsApparchRustPolicyState,
}

#[derive(Debug, Clone, PartialEq)]
pub struct G3RsApparchPatchBypassChecksInput {
    pub patch: G3RsApparchPatchBypass,
    pub rust_policy: G3RsApparchRustPolicyState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsApparchSameLayerDependencyEdge {
    pub from_cargo_rel_path: String,
    pub to_cargo_rel_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsApparchSameLayerCyclesChecksInput {
    pub crates: Vec<G3RsApparchCrate>,
    pub edges: Vec<G3RsApparchSameLayerDependencyEdge>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct G3RsApparchConfigChecksInput {
    pub crate_dependency_checks: Vec<G3RsApparchCrateDependencyChecksInput>,
    pub crate_purity_checks: Vec<G3RsApparchCratePurityChecksInput>,
    pub patch_bypass_checks: Vec<G3RsApparchPatchBypassChecksInput>,
    pub same_layer_cycles_check: G3RsApparchSameLayerCyclesChecksInput,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsApparchSourceChecksInput {
    pub io_traits_checks: Vec<G3RsApparchIoTraitsSourceChecksInput>,
    pub types_public_surface_checks: Vec<G3RsApparchTypesPublicSurfaceChecksInput>,
}
