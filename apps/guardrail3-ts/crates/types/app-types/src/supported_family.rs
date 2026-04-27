/// Family names supported by the app CLI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SupportedFamily {
    /// ESLint checks.
    Eslint,
    /// Astro setup checks.
    AstroSetup,
    /// Astro content checks.
    AstroContent,
    /// Astro MDX checks.
    AstroMdx,
    /// Astro SEO checks.
    AstroSeo,
    /// Astro generated-state checks.
    AstroState,
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
pub const SUPPORTED_FAMILIES: [SupportedFamily; 12] = [
    SupportedFamily::Eslint,
    SupportedFamily::AstroSetup,
    SupportedFamily::AstroContent,
    SupportedFamily::AstroMdx,
    SupportedFamily::AstroSeo,
    SupportedFamily::AstroState,
    SupportedFamily::Arch,
    SupportedFamily::Apparch,
    SupportedFamily::Tsconfig,
    SupportedFamily::Package,
    SupportedFamily::Npmrc,
    SupportedFamily::Jscpd,
];
