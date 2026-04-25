use g3ts_astro_types::G3TsAstroConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-CONFIG-17";
const REQUIRED_DEPS: [&str; 2] = ["astro-seo", "schema-dts"];

pub(crate) fn check(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.integration_contracts {
        let rel_path = crate::support::package_rel_path(contract);
        let missing = REQUIRED_DEPS
            .into_iter()
            .filter(|dependency| !crate::support::package_has_dependency(contract, dependency))
            .collect::<Vec<_>>();

        if missing.is_empty() {
            if let Some(rel_path) = rel_path {
                results.push(crate::support::info(
                    ID,
                    "Astro SEO generation packages are present",
                    format!("`{rel_path}` lists `astro-seo` and `schema-dts`."),
                    rel_path,
                ));
            }
            continue;
        }

        results.push(crate::support::error(
            ID,
            "Astro SEO generation packages are missing",
            format!(
                "This Astro app must list `astro-seo` for rendered SEO tags and `schema-dts` for typed JSON-LD data. Missing package entries: {}. G3TS checks package presence here; rendered SEO correctness is delegated to `@nuasite/checks`.",
                missing
                    .iter()
                    .map(|dependency| format!("`{dependency}`"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            rel_path,
        ));
    }
}
