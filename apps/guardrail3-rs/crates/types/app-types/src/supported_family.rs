/// Family names supported by the app CLI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SupportedFamily {
    /// Topology checks.
    Topology,
    /// Toolchain checks.
    Toolchain,
    /// Rustfmt checks.
    Fmt,
    /// Cargo checks.
    Cargo,
    /// Clippy checks.
    Clippy,
    /// cargo-deny checks.
    Deny,
    /// Source-code checks.
    Code,
    /// Package architecture checks.
    Arch,
    /// Dependency checks.
    Deps,
    /// Garde checks.
    Garde,
    /// Test-structure checks.
    Test,
    /// Release checks.
    Release,
    /// Hook checks.
    Hooks,
    /// App-layer architecture checks.
    Apparch,
}

/// Stable family iteration order used by the app.
pub const SUPPORTED_FAMILIES: [SupportedFamily; 14] = [
    SupportedFamily::Topology,
    SupportedFamily::Toolchain,
    SupportedFamily::Fmt,
    SupportedFamily::Cargo,
    SupportedFamily::Clippy,
    SupportedFamily::Deny,
    SupportedFamily::Code,
    SupportedFamily::Arch,
    SupportedFamily::Deps,
    SupportedFamily::Garde,
    SupportedFamily::Test,
    SupportedFamily::Release,
    SupportedFamily::Hooks,
    SupportedFamily::Apparch,
];
