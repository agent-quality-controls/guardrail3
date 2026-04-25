use g3ts_astro_types::G3TsAstroConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-CONFIG-06";
const DEPENDENCY_NAME: &str = "g3ts-eslint-plugin-astro-pipeline";

pub(crate) fn check(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.integration_contracts {
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
                "`{rel_path}` does not list `{DEPENDENCY_NAME}` in dependencies or devDependencies. Add `{DEPENDENCY_NAME}` to `{rel_path}`. Astro source-pipeline rules must come from the shared ESLint plugin so route bypasses fail in lint."
            ),
            None => format!(
                "The Astro pipeline ESLint plugin package contract could not be checked because `package.json` was not available. Restore the app package manifest and declare `{DEPENDENCY_NAME}` there. Astro source-pipeline rules must come from the shared ESLint plugin so route bypasses fail in lint."
            ),
        };
        results.push(crate::support::error(
            ID,
            "Astro app package is missing `g3ts-eslint-plugin-astro-pipeline`",
            message,
            rel_path,
        ));
    }
}
