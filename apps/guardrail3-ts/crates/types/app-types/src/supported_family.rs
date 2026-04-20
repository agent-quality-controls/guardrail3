/// Family names supported by the app CLI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SupportedFamily {
    /// ESLint checks.
    Eslint,
    /// tsconfig checks.
    Tsconfig,
}

/// Stable family iteration order used by the app.
pub const SUPPORTED_FAMILIES: [SupportedFamily; 2] =
    [SupportedFamily::Eslint, SupportedFamily::Tsconfig];
