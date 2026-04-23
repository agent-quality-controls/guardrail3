use g3ts_astro_types::G3TsAstroConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-CONFIG-03";
const DEPENDENCY_NAME: &str = "eslint-plugin-astro";

pub(crate) fn check(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.integration_contracts {
        let rel_path = crate::support::package_rel_path(contract);
        if crate::support::package_has_dependency(contract, DEPENDENCY_NAME) {
            if let Some(rel_path) = rel_path {
                results.push(crate::support::info(
                    ID,
                    "astro ESLint plugin package present",
                    format!("`{rel_path}` includes `{DEPENDENCY_NAME}`."),
                    rel_path,
                ));
            }
            continue;
        }

        let message = match rel_path {
            Some(rel_path) => format!("`{rel_path}` does not include `{DEPENDENCY_NAME}`."),
            None => format!(
                "Could not verify `{DEPENDENCY_NAME}` because no package manifest was available."
            ),
        };
        results.push(crate::support::error(
            ID,
            "astro ESLint plugin package missing",
            message,
            rel_path,
        ));
    }
}
