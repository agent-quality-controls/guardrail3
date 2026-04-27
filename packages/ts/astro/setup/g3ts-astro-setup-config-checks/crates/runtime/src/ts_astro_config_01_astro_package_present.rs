use g3ts_astro_types::G3TsAstroConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-SETUP-CONFIG-01";
const PACKAGE_NAME: &str = "astro";

pub(crate) fn check(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.integration_contracts {
        let rel_path = g3ts_astro_check_support::core::package_rel_path(contract);
        if g3ts_astro_check_support::core::package_has_dependency(contract, PACKAGE_NAME) {
            if let Some(rel_path) = rel_path {
                results.push(g3ts_astro_check_support::core::info(
                    ID,
                    "astro package present",
                    format!("`{rel_path}` includes `{PACKAGE_NAME}`."),
                    rel_path,
                ));
            }
            continue;
        }

        let message = match rel_path {
            Some(rel_path) => format!(
                "`{rel_path}` does not list `{PACKAGE_NAME}` in dependencies or devDependencies. Add `{PACKAGE_NAME}` to `{rel_path}`. Without that dependency entry, this app can drift away from the Astro framework contract without the package surface showing it."
            ),
            None => "The Astro package contract could not be checked because `package.json` was not available. Restore the app package manifest and declare `astro` there. Without that manifest, the app has no package surface that states it is an Astro app.".to_owned(),
        };

        results.push(g3ts_astro_check_support::core::error(
            ID,
            "Astro app package is missing `astro`",
            message,
            rel_path,
        ));
    }
}
