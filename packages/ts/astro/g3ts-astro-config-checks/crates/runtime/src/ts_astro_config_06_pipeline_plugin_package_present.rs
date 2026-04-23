use g3ts_astro_types::G3TsAstroConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-CONFIG-06";
const DEPENDENCY_NAME: &str = "eslint-plugin-astro-pipeline";

pub(crate) fn check(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.integration_contracts {
        if !contract.requires_source_pipeline_linting {
            continue;
        }

        let rel_path = crate::support::package_rel_path(contract);
        if crate::support::package_has_dependency(contract, DEPENDENCY_NAME) {
            if let Some(rel_path) = rel_path {
                results.push(crate::support::info(
                    ID,
                    "astro pipeline ESLint plugin package present",
                    format!("`{rel_path}` includes `{DEPENDENCY_NAME}`."),
                    rel_path,
                ));
            }
            continue;
        }

        let message = match rel_path {
            Some(rel_path) => format!(
                "`{rel_path}` does not include the required Astro pipeline ESLint plugin package `{DEPENDENCY_NAME}`."
            ),
            None => format!(
                "Could not verify the required Astro pipeline ESLint plugin package `{DEPENDENCY_NAME}` because no package manifest was available."
            ),
        };
        results.push(crate::support::error(
            ID,
            "astro pipeline ESLint plugin package missing",
            message,
            rel_path,
        ));
    }
}
