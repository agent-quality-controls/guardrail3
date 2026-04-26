use g3ts_astro_types::G3TsAstroConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-CONFIG-17";
const REQUIRED_DEP: &str = "schema-dts";

pub(crate) fn check(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.integration_contracts {
        let rel_path = crate::support::package_rel_path(contract);
        if crate::support::package_has_dependency(contract, REQUIRED_DEP) {
            if let Some(rel_path) = rel_path {
                results.push(crate::support::info(
                    ID,
                    "Astro JSON-LD type package is present",
                    format!("`{rel_path}` lists `schema-dts` for typed JSON-LD data."),
                    rel_path,
                ));
            }
            continue;
        }

        results.push(crate::support::error(
            ID,
            "Astro JSON-LD type package is missing",
            format!(
                "This Astro app must list `schema-dts` for typed JSON-LD data. G3TS checks package presence here; rendered SEO tags and JSON-LD presence are delegated to `@nuasite/checks`."
            ),
            rel_path,
        ));
    }
}
