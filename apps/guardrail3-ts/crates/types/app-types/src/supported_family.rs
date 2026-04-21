/// Family names supported by the app CLI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SupportedFamily {
    /// ESLint checks.
    Eslint,
    /// TS architecture checks.
    Arch,
    /// TS app architecture checks.
    Apparch,
    /// tsconfig checks.
    Tsconfig,
    /// package.json checks.
    Package,
    /// .npmrc checks.
    Npmrc,
    /// .jscpd.json checks.
    Jscpd,
}

/// Stable family iteration order used by the app.
pub const SUPPORTED_FAMILIES: [SupportedFamily; 7] = [
    SupportedFamily::Eslint,
    SupportedFamily::Arch,
    SupportedFamily::Apparch,
    SupportedFamily::Tsconfig,
    SupportedFamily::Package,
    SupportedFamily::Npmrc,
    SupportedFamily::Jscpd,
];
