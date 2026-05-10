use g3rs_hooks_contract_types::G3HookRequirement;

/// Tool-driven Rust families whose hook contracts this crate aggregates.
///
/// Each variant maps one-to-one to a `g3rs-{family}-hook-contract` crate. These
/// families share the property that their checks run against code or
/// configuration via an external tool (cargo, rustfmt, clippy, etc.), as
/// distinct from the architecture-shape families aggregated by the sibling
/// `architecture-contracts` crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolingFamily {
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
}

/// Returns the hook contract requirements owned by `family`.
#[must_use]
pub fn hook_contract(family: ToolingFamily) -> Vec<G3HookRequirement> {
    match family {
        ToolingFamily::Cargo => g3rs_cargo_hook_contract::hook_contract(),
        ToolingFamily::Clippy => g3rs_clippy_hook_contract::hook_contract(),
        ToolingFamily::Code => g3rs_code_hook_contract::hook_contract(),
        ToolingFamily::Deny => g3rs_deny_hook_contract::hook_contract(),
        ToolingFamily::Deps => g3rs_deps_hook_contract::hook_contract(),
        ToolingFamily::Fmt => g3rs_fmt_hook_contract::hook_contract(),
        ToolingFamily::Garde => g3rs_garde_hook_contract::hook_contract(),
        ToolingFamily::Release => g3rs_release_hook_contract::hook_contract(),
        ToolingFamily::Test => g3rs_test_hook_contract::hook_contract(),
        ToolingFamily::Toolchain => g3rs_toolchain_hook_contract::hook_contract(),
    }
}
