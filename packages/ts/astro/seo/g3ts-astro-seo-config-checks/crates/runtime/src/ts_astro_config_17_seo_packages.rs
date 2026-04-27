use g3ts_astro_types::G3TsAstroSeoIntegrationContractInput;
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-SEO-CONFIG-17";
const REQUIRED_DEP: &str = "schema-dts";

pub(crate) fn check(contracts: &[G3TsAstroSeoIntegrationContractInput], results: &mut Vec<G3CheckResult>) {
    for contract in contracts {
        let rel_path = g3ts_astro_check_support::core::package_rel_path(contract);
        if g3ts_astro_check_support::core::package_has_dependency(contract, REQUIRED_DEP) {
            if let Some(rel_path) = rel_path {
                results.push(g3ts_astro_check_support::core::info(
                    ID,
                    "Astro JSON-LD type package is present",
                    format!("`{rel_path}` lists `schema-dts` for typed JSON-LD data."),
                    rel_path,
                ));
            }
            continue;
        }

        results.push(g3ts_astro_check_support::core::error(
            ID,
            "Astro JSON-LD type package is missing",
            format!(
                "This Astro app must list `schema-dts` for typed JSON-LD data. G3TS checks package presence here; rendered SEO tags and JSON-LD presence are delegated to `@nuasite/checks`."
            ),
            rel_path,
        ));
    }
}
