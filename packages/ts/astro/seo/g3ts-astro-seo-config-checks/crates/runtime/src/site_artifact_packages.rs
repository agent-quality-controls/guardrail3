use g3ts_astro_seo_types::G3TsAstroSeoIntegrationContractInput;
use guardrail3_check_types::G3CheckResult;

/// Static rule data.
const SITEMAP_GENERATOR_ID: &str = "g3ts-astro-seo/sitemap-generator-package-present";
/// Static rule data.
const ROBOTS_GENERATOR_ID: &str = "g3ts-astro-seo/robots-generator-package-present";
/// Static rule data.
const SITEMAP_ID: &str = "g3ts-astro-seo/sitemap-auditor-package-present";
/// Static rule data.
const ROBOTS_ID: &str = "g3ts-astro-seo/robots-auditor-package-present";
/// Static rule data.
const LLMS_ID: &str = "g3ts-astro-seo/llms-auditor-package-present";
/// Static rule data.
const LLMS_GENERATOR_ID: &str = "g3ts-astro-seo/llms-generator-package-present";

/// Validates the rule and pushes findings into `results`.
/// Internal helper exported within the runtime crate.
pub(crate) fn check(
    contract: &G3TsAstroSeoIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    check_required(contract, results, SITEMAP_GENERATOR_ID, "@astrojs/sitemap");
    check_required(contract, results, ROBOTS_GENERATOR_ID, "astro-robots");
    check_required(contract, results, SITEMAP_ID, "g3ts-astro-sitemap-auditor");
    check_required(contract, results, ROBOTS_ID, "g3ts-astro-robots-auditor");
    if crate::support::strict_ai_readable_enabled(&contract.astro_policy) {
        check_required(
            contract,
            results,
            LLMS_GENERATOR_ID,
            "g3ts-astro-llms-generator",
        );
        check_required(contract, results, LLMS_ID, "g3ts-astro-llms-auditor");
    }
}

/// Internal helper used by the rule.
fn check_required(
    contract: &G3TsAstroSeoIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
    id: &str,
    dependency_name: &str,
) {
    let rel_path = crate::support::package_rel_path(&contract.package);
    if crate::support::package_has_dependency(&contract.package, dependency_name) {
        results.push(crate::support::info(
            id,
            "Astro site artifact package is present",
            format!("`{rel_path}` lists `{dependency_name}`."),
            rel_path,
        ));
        return;
    }

    results.push(crate::support::error(
        id,
        "Astro site artifact package is missing",
        format!("This Astro app must list `{dependency_name}`."),
        Some(rel_path),
    ));
}
