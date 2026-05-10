//! Shared apparch types: layers, dependencies, crates, and check inputs.

use guardrail3_rs_toml_parser::types::{RustProfile, WaiverConfig};

/// Architectural layer assigned to a crate within an apparch workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3RsApparchLayer {
    /// Pure data types layer.
    Types,
    /// Domain logic layer.
    Logic,
    /// Inbound IO layer.
    IoInbound,
    /// Outbound IO layer.
    IoOutbound,
}

/// Kind of cargo dependency edge between crates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3RsApparchDependencyKind {
    /// Regular `[dependencies]` entry.
    Dependency,
    /// `[dev-dependencies]` entry.
    DevDependency,
    /// `[build-dependencies]` entry.
    BuildDependency,
    /// `[target.*.dependencies]` entry.
    TargetDependency,
    /// `[target.*.dev-dependencies]` entry.
    TargetDevDependency,
    /// `[target.*.build-dependencies]` entry.
    TargetBuildDependency,
}

impl G3RsApparchDependencyKind {
    /// Returns true if this dependency kind is one of the dev-only forms.
    #[must_use]
    pub const fn is_dev(self) -> bool {
        matches!(self, Self::DevDependency | Self::TargetDevDependency)
    }

    /// Returns the cargo manifest table label for this dependency kind.
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

/// Kind of `[patch]` or `[replace]` table entry in a Cargo manifest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3RsApparchPatchKind {
    /// `[patch.*]` table entry.
    Patch,
    /// `[replace]` table entry.
    Replace,
}

impl G3RsApparchPatchKind {
    /// Returns the manifest table label for this patch kind.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Patch => "patch",
            Self::Replace => "replace",
        }
    }
}

/// Parse state of the per-crate `guardrail3-rs.toml` rust policy file.
#[derive(Debug, Clone, PartialEq)]
pub enum G3RsApparchRustPolicyState {
    /// No rust policy file present.
    Missing,
    /// Rust policy file exists but could not be read.
    Unreadable {
        /// Repo-relative path to the policy file.
        rel_path: String,
        /// Reason the file could not be read.
        reason: String,
    },
    /// Rust policy file exists and was read but failed to parse.
    ParseError {
        /// Repo-relative path to the policy file.
        rel_path: String,
        /// Reason the file failed to parse.
        reason: String,
    },
    /// Rust policy file was parsed successfully.
    Parsed {
        /// Repo-relative path to the policy file.
        rel_path: String,
        /// Optional declared rust profile.
        profile: Option<RustProfile>,
        /// Allow-listed external dependencies declared by the policy.
        allowed_deps: Vec<String>,
        /// Waivers declared by the policy.
        waivers: Vec<WaiverConfig>,
    },
}

/// A crate observed within an apparch workspace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsApparchCrate {
    /// Crate name as declared in the manifest.
    pub crate_name: String,
    /// Repo-relative path to the crate's `Cargo.toml`.
    pub cargo_rel_path: String,
    /// Repo-relative directory containing the crate.
    pub rel_dir: String,
    /// Architectural layer the crate belongs to, if any.
    pub layer: Option<G3RsApparchLayer>,
}

/// A directed dependency edge between two crates within the workspace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsApparchDependencyEdge {
    /// Source crate Cargo.toml path.
    pub from_cargo_rel_path: String,
    /// Target crate Cargo.toml path.
    pub to_cargo_rel_path: String,
    /// Dependency entry name as declared.
    pub dep_name: String,
    /// Kind of dependency edge.
    pub kind: G3RsApparchDependencyKind,
}

/// A dependency on a crate outside the workspace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsApparchExternalDependency {
    /// Cargo.toml path of the crate declaring the dependency.
    pub cargo_rel_path: String,
    /// External dependency name.
    pub dep_name: String,
    /// Kind of dependency entry.
    pub kind: G3RsApparchDependencyKind,
}

/// A dependency edge resolved to its target crate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsApparchBoundDependency {
    /// Dependency entry name as declared.
    pub dep_name: String,
    /// Kind of dependency entry.
    pub kind: G3RsApparchDependencyKind,
    /// Resolved target crate.
    pub target: G3RsApparchCrate,
}

/// A `[patch]` or `[replace]` entry observed in the workspace manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsApparchPatchBypass {
    /// Workspace Cargo.toml path declaring the bypass.
    pub cargo_rel_path: String,
    /// Manifest key for the bypass entry.
    pub key: String,
    /// Bypass kind (patch or replace).
    pub kind: G3RsApparchPatchKind,
    /// Cargo.toml path of the target crate.
    pub target_cargo_rel_path: String,
    /// Repo-relative directory of the target crate.
    pub target_rel_dir: String,
    /// Architectural layer of the target crate, if any.
    pub target_layer: Option<G3RsApparchLayer>,
}

/// Kind of public item exposed by a crate's surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3RsApparchPublicItemKind {
    /// Public trait declaration.
    Trait,
    /// Free standing public function.
    FreeFunction,
    /// Public method on an inherent impl block.
    InherentMethod,
}

/// A public item exposed from a crate's source tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsApparchPublicItem {
    /// Cargo.toml path of the owning crate.
    pub cargo_rel_path: String,
    /// Repo-relative source file path containing the item.
    pub rel_path: String,
    /// Item identifier.
    pub item_name: String,
    /// Owning type name when the item is a method.
    pub owner_name: Option<String>,
    /// Kind of the public item.
    pub kind: G3RsApparchPublicItemKind,
}

/// Input for the io-traits source check on a single crate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsApparchIoTraitsSourceChecksInput {
    /// Crate under inspection.
    pub krate: G3RsApparchCrate,
    /// Public traits exposed by the crate.
    pub public_traits: Vec<G3RsApparchPublicItem>,
}

/// Input for the types-public-surface source check on a single crate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsApparchTypesPublicSurfaceChecksInput {
    /// Crate under inspection.
    pub krate: G3RsApparchCrate,
    /// Public items that expose behavior on the crate's surface.
    pub public_behavior_items: Vec<G3RsApparchPublicItem>,
}

/// Input for the crate-dependency check on a single crate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsApparchCrateDependencyChecksInput {
    /// Crate under inspection.
    pub krate: G3RsApparchCrate,
    /// Internal dependencies bound to their target crates.
    pub internal_dependencies: Vec<G3RsApparchBoundDependency>,
}

/// Input for the crate-purity check on a single crate.
#[derive(Debug, Clone, PartialEq)]
pub struct G3RsApparchCratePurityChecksInput {
    /// Crate under inspection.
    pub krate: G3RsApparchCrate,
    /// External dependencies declared by the crate.
    pub external_dependencies: Vec<G3RsApparchExternalDependency>,
    /// Resolved rust policy state for the crate.
    pub rust_policy: G3RsApparchRustPolicyState,
}

/// Input for the patch-bypass check on a single bypass entry.
#[derive(Debug, Clone, PartialEq)]
pub struct G3RsApparchPatchBypassChecksInput {
    /// Bypass entry under inspection.
    pub patch: G3RsApparchPatchBypass,
    /// Resolved rust policy state for the workspace.
    pub rust_policy: G3RsApparchRustPolicyState,
}

/// A dependency edge between two crates assigned to the same layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsApparchSameLayerDependencyEdge {
    /// Source crate.
    pub from: G3RsApparchCrate,
    /// Target crate.
    pub to: G3RsApparchCrate,
}

/// Input for the same-layer cycles check across the workspace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsApparchSameLayerCyclesChecksInput {
    /// Same-layer dependency edges in the workspace.
    pub edges: Vec<G3RsApparchSameLayerDependencyEdge>,
}

/// Aggregated input for all apparch config checks.
#[derive(Debug, Clone, PartialEq)]
pub struct G3RsApparchConfigChecksInput {
    /// Per-crate dependency checks.
    pub crate_dependency_checks: Vec<G3RsApparchCrateDependencyChecksInput>,
    /// Per-crate purity checks.
    pub crate_purity_checks: Vec<G3RsApparchCratePurityChecksInput>,
    /// Per-bypass patch checks.
    pub patch_bypass_checks: Vec<G3RsApparchPatchBypassChecksInput>,
    /// Workspace same-layer cycles check.
    pub same_layer_cycles_check: G3RsApparchSameLayerCyclesChecksInput,
}

/// Aggregated input for all apparch source checks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsApparchSourceChecksInput {
    /// Per-crate io-traits source checks.
    pub io_traits_checks: Vec<G3RsApparchIoTraitsSourceChecksInput>,
    /// Per-crate types-public-surface source checks.
    pub types_public_surface_checks: Vec<G3RsApparchTypesPublicSurfaceChecksInput>,
}
