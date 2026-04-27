use g3ts_astro_types::G3TsAstroSetupIntegrationContractInput;
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-SETUP-CONFIG-03";
const DEPENDENCY_NAME: &str = "eslint-plugin-astro";

pub(crate) fn check(contracts: &[G3TsAstroSetupIntegrationContractInput], results: &mut Vec<G3CheckResult>) {
    for contract in contracts {
        let rel_path = g3ts_astro_check_support::core::package_rel_path(contract);
        if g3ts_astro_check_support::core::package_has_dependency(contract, DEPENDENCY_NAME) {
            if let Some(rel_path) = rel_path {
                results.push(g3ts_astro_check_support::core::info(
                    ID,
                    "astro ESLint plugin package present",
                    format!("`{rel_path}` includes `{DEPENDENCY_NAME}`."),
                    rel_path,
                ));
            }
            continue;
        }

        let message = match rel_path {
            Some(rel_path) => format!(
                "`{rel_path}` does not list `{DEPENDENCY_NAME}` in dependencies or devDependencies. Add `{DEPENDENCY_NAME}` to `{rel_path}`. Astro source files need the Astro ESLint plugin so Astro-specific lint rules can run."
            ),
            None => format!(
                "The Astro ESLint plugin package contract could not be checked because `package.json` was not available. Restore the app package manifest and declare `{DEPENDENCY_NAME}` there. Astro source files need that plugin so Astro-specific lint rules can run."
            ),
        };
        results.push(g3ts_astro_check_support::core::error(
            ID,
            "Astro app package is missing `eslint-plugin-astro`",
            message,
            rel_path,
        ));
    }
}
