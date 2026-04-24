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
                        "`{rel_path}` activates `{PLUGIN_NAME}` and enforces the required Astro pipeline rules at error severity on the Astro, TS, and TSX source lanes."
                    ),
                    rel_path,
                ));
            }
            continue;
        }

        let message = match rel_path {
            Some(rel_path) => format!(
                "`{rel_path}` does not activate `{PLUGIN_NAME}` with all required Astro pipeline rules at error severity on the Astro, TS, and TSX source lanes. Enable the `astro-pipeline` plugin and set the required Astro pipeline rules to `error` in the Astro, TS, and TSX lane configs in `{rel_path}`. Astro source files must run through the shared pipeline rules so route bypass checks and runtime MDX checks actually execute."
            ),
            None => format!(
                "The Astro pipeline ESLint wiring contract could not be checked because `eslint.config.*` was not available. Restore the app ESLint config and enable `astro-pipeline` with the required rules on the Astro, TS, and TSX source lanes there. Astro source files must run through the shared pipeline rules so route bypass checks and runtime MDX checks actually execute."
            ),
        };
        results.push(crate::support::error(
            ID,
            "Astro ESLint lanes are not enforcing the required `astro-pipeline` rules",
            message,
            rel_path,
        ));
    }
}
