/// Family names supported by the app CLI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SupportedFamily {
    /// `ESLint` checks.
    Eslint,
    /// Astro setup checks.
    AstroSetup,
    /// Astro content checks.
    AstroContent,
    /// Astro MDX checks.
    AstroMdx,
    /// Astro i18n checks.
    AstroI18n,
    /// Astro media checks.
    AstroMedia,
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
    /// Framework-independent TS style checks.
    Style,
    /// Git hook contract checks.
    Hooks,
}

/// Stable family iteration order used by the app.
pub const SUPPORTED_FAMILIES: [SupportedFamily; 16] = [
    SupportedFamily::Eslint,
    SupportedFamily::AstroSetup,
    SupportedFamily::AstroContent,
    SupportedFamily::AstroMdx,
    SupportedFamily::AstroI18n,
    SupportedFamily::AstroMedia,
    SupportedFamily::AstroSeo,
    SupportedFamily::AstroState,
    SupportedFamily::Arch,
    SupportedFamily::Apparch,
    SupportedFamily::Tsconfig,
    SupportedFamily::Package,
    SupportedFamily::Npmrc,
    SupportedFamily::Jscpd,
    SupportedFamily::Style,
    SupportedFamily::Hooks,
];
