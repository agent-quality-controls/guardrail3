use guardrail3_ts_app_types::{SUPPORTED_FAMILIES, SupportedFamily, ValidateRequest};

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

#[must_use]
pub fn selected_families(request: &ValidateRequest) -> Vec<SupportedFamily> {
    if request.families.is_empty() {
        // reason: per-package validate excludes repo-only families like Hooks
        // and Topology; those run from validate-repo.
        return SUPPORTED_FAMILIES
            .into_iter()
            .filter(|family| !is_repo_only_family(*family))
            .collect();
    }

    SUPPORTED_FAMILIES
        .into_iter()
        .filter(|family| request.families.contains(family))
        .collect()
}

/// Returns true when `family` only runs from `validate-repo` and is excluded
/// from per-package `validate --path` defaults.
const fn is_repo_only_family(family: SupportedFamily) -> bool {
    matches!(family, SupportedFamily::Hooks | SupportedFamily::Topology,)
}

/// Returns the families to run for a per-package validate, after applying the
/// package's `guardrail3-ts.toml` opt-out for disabled families.
#[must_use]
pub fn selected_families_with_opt_out(
    request: &ValidateRequest,
    disabled: &[SupportedFamily],
) -> Vec<SupportedFamily> {
    selected_families(request)
        .into_iter()
        .filter(|family| !disabled.contains(family))
        .collect()
}
