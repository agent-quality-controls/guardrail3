use g3rs_hooks_contract_types::G3HookRequirement;

/// Architecture-shape Rust families whose hook contracts this crate aggregates.
///
/// These families describe the workspace's structural shape (topology of
/// crates, per-crate architecture, application-level architecture). They are
/// grouped together because they share a common purpose, distinct from the
/// tool-driven families aggregated by the sibling `tooling-contracts` crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArchitectureFamily {
    /// Application-architecture family.
    Apparch,
    /// Crate-architecture family.
    Arch,
    /// Workspace topology family.
    Topology,
}

/// Returns the hook contract requirements owned by `family`.
#[must_use]
pub fn hook_contract(family: ArchitectureFamily) -> Vec<G3HookRequirement> {
    match family {
        ArchitectureFamily::Apparch => g3rs_apparch_hook_contract::hook_contract(),
        ArchitectureFamily::Arch => g3rs_arch_hook_contract::hook_contract(),
        ArchitectureFamily::Topology => g3rs_topology_hook_contract::hook_contract(),
    }
}
