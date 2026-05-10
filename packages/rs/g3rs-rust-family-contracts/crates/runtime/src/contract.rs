use g3rs_hooks_contract_types::G3HookRequirement;
use g3rs_rust_family_contracts_architecture::{
    ArchitectureFamily, hook_contract as architecture_hook_contract,
};
use g3rs_rust_family_contracts_tooling::{ToolingFamily, hook_contract as tooling_hook_contract};

/// One Rust family whose hook contract this aggregator can resolve.
///
/// The variants are split into two internal groupings - tool-driven families
/// (cargo, fmt, clippy, ...) and architecture-shape families (topology, arch,
/// apparch). The router below dispatches each variant into the correct sibling
/// sub-crate so that callers never need to import any per-family contract crate
/// directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RustFamily {
    /// Application-architecture family.
    Apparch,
    /// Crate-architecture family.
    Arch,
    /// Cargo manifest / lockfile family.
    Cargo,
    /// Clippy lint family.
    Clippy,
    /// Source code style and content family.
    Code,
    /// Cargo-deny dependency policy family.
    Deny,
    /// Rust dependency hygiene family.
    Deps,
    /// Rustfmt formatting family.
    Fmt,
    /// Garde validation family.
    Garde,
    /// Release / publishing family.
    Release,
    /// Test-layout family.
    Test,
    /// Toolchain pinning family.
    Toolchain,
    /// Workspace topology family.
    Topology,
}

/// Returns the hook contract requirements owned by `family`.
///
/// Each family's contract crate is the single source of truth for the runnable
/// commands and trigger patterns that family demands; this function only routes
/// to it.
#[must_use]
pub fn family_hook_contract(family: RustFamily) -> Vec<G3HookRequirement> {
    match family {
        RustFamily::Apparch => architecture_hook_contract(ArchitectureFamily::Apparch),
        RustFamily::Arch => architecture_hook_contract(ArchitectureFamily::Arch),
        RustFamily::Topology => architecture_hook_contract(ArchitectureFamily::Topology),
        RustFamily::Cargo => tooling_hook_contract(ToolingFamily::Cargo),
        RustFamily::Clippy => tooling_hook_contract(ToolingFamily::Clippy),
        RustFamily::Code => tooling_hook_contract(ToolingFamily::Code),
        RustFamily::Deny => tooling_hook_contract(ToolingFamily::Deny),
        RustFamily::Deps => tooling_hook_contract(ToolingFamily::Deps),
        RustFamily::Fmt => tooling_hook_contract(ToolingFamily::Fmt),
        RustFamily::Garde => tooling_hook_contract(ToolingFamily::Garde),
        RustFamily::Release => tooling_hook_contract(ToolingFamily::Release),
        RustFamily::Test => tooling_hook_contract(ToolingFamily::Test),
        RustFamily::Toolchain => tooling_hook_contract(ToolingFamily::Toolchain),
    }
}
