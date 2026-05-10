use g3ts_astro_seo_types::G3TsAstroSeoIntegrationContractInput;
use guardrail3_check_types::G3CheckResult;

/// Static rule data.
const ID: &str = "g3ts-astro-seo/seo-packages";
/// Static rule data.
const REQUIRED_DEP: &str = "schema-dts";

/// Validates the rule and pushes findings into `results`.
/// Internal helper exported within the runtime crate.
pub(crate) fn check(
    contract: &G3TsAstroSeoIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::package_rel_path(&contract.package);
    if crate::support::package_has_dependency(&contract.package, REQUIRED_DEP) {
        results.push(crate::support::info(
            ID,
            "Astro JSON-LD type package is present",
            format!("`{rel_path}` lists `schema-dts` for typed JSON-LD data."),
            rel_path,
        ));
        return;
    }

    results.push(crate::support::error(
            ID,
            "Astro JSON-LD type package is missing",
            "This Astro app must list `schema-dts` for typed JSON-LD data. G3TS checks package presence here; rendered SEO tags and JSON-LD presence are delegated to `@nuasite/checks`.".to_owned(),
            Some(rel_path),
        ));
}
