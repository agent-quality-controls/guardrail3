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
    /// Formatting toolchain checks.
    Fmt,
    /// Spelling toolchain checks.
    Spelling,
    /// Type coverage toolchain checks.
    Typecov,
    /// Git hook contract checks.
    Hooks,
    /// Repo-wide topology checks (run by validate-repo).
    Topology,
}

/// Stable family iteration order used by the app.
pub const SUPPORTED_FAMILIES: [SupportedFamily; 20] = [
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
    SupportedFamily::Fmt,
    SupportedFamily::Spelling,
    SupportedFamily::Typecov,
    SupportedFamily::Hooks,
    SupportedFamily::Topology,
];
