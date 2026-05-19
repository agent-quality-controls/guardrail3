use guardrail3_ts_app_types::{SUPPORTED_FAMILIES, SupportedFamily, ValidateWorkspaceRequest};

#[must_use]
pub const fn family_cli_name(family: SupportedFamily) -> &'static str {
    match family {
        SupportedFamily::Eslint => "eslint",
        SupportedFamily::AstroSetup => "astro-setup",
        SupportedFamily::AstroContent => "astro-content",
        SupportedFamily::AstroMdx => "astro-mdx",
        SupportedFamily::AstroI18n => "astro-i18n",
        SupportedFamily::AstroMedia => "astro-media",
        SupportedFamily::AstroSeo => "astro-seo",
        SupportedFamily::AstroState => "astro-state",
        SupportedFamily::Arch => "arch",
        SupportedFamily::Apparch => "apparch",
        SupportedFamily::Tsconfig => "tsconfig",
        SupportedFamily::Package => "package",
        SupportedFamily::Npmrc => "npmrc",
        SupportedFamily::Jscpd => "jscpd",
        SupportedFamily::Style => "style",
        SupportedFamily::Fmt => "fmt",
        SupportedFamily::Spelling => "spelling",
        SupportedFamily::Typecov => "typecov",
        SupportedFamily::Hooks => "hooks",
        SupportedFamily::Topology => "topology",
    }
}

/// Per-workspace default families (Hooks and Topology run from validate-repo).
pub const PER_WORKSPACE_DEFAULT_FAMILIES: &[SupportedFamily] = &[
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
];

/// Repo-level families (validate-repo runs only these).
pub const REPO_LEVEL_FAMILIES: &[SupportedFamily] =
    &[SupportedFamily::Hooks, SupportedFamily::Topology];

#[must_use]
pub fn selected_families(request: &ValidateWorkspaceRequest) -> Vec<SupportedFamily> {
    if request.families.is_empty() {
        return PER_WORKSPACE_DEFAULT_FAMILIES.to_vec();
    }

    SUPPORTED_FAMILIES
        .into_iter()
        .filter(|family| request.families.contains(family))
        .collect()
}

/// Returns the families to run for a per-package validate, after applying the
/// package's `guardrail3-ts.toml` opt-out for disabled families.
#[must_use]
pub fn selected_families_with_opt_out(
    request: &ValidateWorkspaceRequest,
    disabled: &[SupportedFamily],
) -> Vec<SupportedFamily> {
    selected_families(request)
        .into_iter()
        .filter(|family| !disabled.contains(family))
        .collect()
}
