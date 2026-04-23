use g3ts_astro_types::G3TsAstroConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-CONFIG-07";
const PLUGIN_NAME: &str = "astro-pipeline";
const REQUIRED_RULES: [&str; 5] = [
    "astro-pipeline/no-authored-content-fs-read",
    "astro-pipeline/no-authored-content-glob",
    "astro-pipeline/no-direct-astro-content-in-routes",
    "astro-pipeline/no-runtime-mdx-eval",
    "astro-pipeline/no-side-loader-imports",
];

pub(crate) fn check(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.eslint_contracts {
        if !contract.requires_source_pipeline_linting {
            continue;
        }

        let rel_path = crate::support::eslint_rel_path(contract);
        if crate::support::eslint_required_lanes_have_plugin_and_rules(
            contract,
            PLUGIN_NAME,
            &REQUIRED_RULES,
        ) {
            if let Some(rel_path) = rel_path {
                results.push(crate::support::info(
                    ID,
                    "astro pipeline ESLint plugin wired and effective",
                    format!(
                        "`{rel_path}` activates `{PLUGIN_NAME}` and enforces the required Astro pipeline rules at error severity."
                    ),
                    rel_path,
                ));
            }
            continue;
        }

        let message = match rel_path {
            Some(rel_path) => format!(
                "`{rel_path}` does not both activate `{PLUGIN_NAME}` and enforce the required Astro pipeline rules at error severity."
            ),
            None => format!(
                "Could not verify the required `{PLUGIN_NAME}` ESLint plugin wiring because no ESLint config was available."
            ),
        };
        results.push(crate::support::error(
            ID,
            "astro pipeline ESLint plugin not effective",
            message,
            rel_path,
        ));
    }
}
