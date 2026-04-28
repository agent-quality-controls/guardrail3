use g3ts_astro_seo_types::G3TsAstroSeoIntegrationContractInput;
use guardrail3_check_types::G3CheckResult;

const SITEMAP_GENERATOR_ID: &str = "g3ts-astro-seo/sitemap-generator-package-present";
const ROBOTS_GENERATOR_ID: &str = "g3ts-astro-seo/robots-generator-package-present";
const SITEMAP_ID: &str = "g3ts-astro-seo/sitemap-checks-package-present";
const ROBOTS_ID: &str = "g3ts-astro-seo/robots-checks-package-present";
const LLMS_ID: &str = "g3ts-astro-seo/llms-checks-package-present";
const LLMS_GENERATOR_ID: &str = "g3ts-astro-seo/llms-generator-package-present";

pub(crate) fn check(
    contract: &G3TsAstroSeoIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    check_required(contract, results, SITEMAP_GENERATOR_ID, "@astrojs/sitemap");
    check_required(contract, results, ROBOTS_GENERATOR_ID, "astro-robots");
    check_required(contract, results, SITEMAP_ID, "g3ts-astro-sitemap-checks");
    check_required(contract, results, ROBOTS_ID, "g3ts-astro-robots-checks");
    if crate::support::strict_ai_readable_enabled(&contract.astro_policy) {
        check_required(contract, results, LLMS_GENERATOR_ID, "g3ts-astro-llms");
        check_required(contract, results, LLMS_ID, "g3ts-astro-llms-checks");
    }
}

fn check_required(
    contract: &G3TsAstroSeoIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
    id: &str,
    dependency_name: &str,
) {
    let rel_path = crate::support::package_rel_path(&contract.package);
    if crate::support::package_has_dependency(&contract.package, dependency_name) {
        if let Some(rel_path) = rel_path {
            results.push(crate::support::info(
                id,
                "Astro crawler package is present",
                format!("`{rel_path}` lists `{dependency_name}`."),
                rel_path,
            ));
        }
        return;
    }

    results.push(crate::support::error(
        id,
        "Astro crawler package is missing",
        format!("This Astro app must list `{dependency_name}`."),
        rel_path,
    ));
}
