use g3ts_astro_content_types::G3TsAstroContentIntegrationContractInput;
use guardrail3_check_types::G3CheckResult;

/// Internal constant `ID`.
const ID: &str = "g3ts-astro-content/pipeline-plugin-package-present";
/// Internal constant `DEPENDENCY_NAME`.
const DEPENDENCY_NAME: &str = "g3ts-eslint-plugin-astro-pipeline";

/// Internal function `check`.
pub(crate) fn check(
    contract: &G3TsAstroContentIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::package_rel_path(&contract.package);
    if crate::support::package_has_dependency(&contract.package, DEPENDENCY_NAME) {
        results.push(crate::support::info(
            ID,
            "Astro content pipeline ESLint plugin package present",
            format!("`{rel_path}` includes `{DEPENDENCY_NAME}`."),
            rel_path,
        ));
        return;
    }

    results.push(crate::support::error(
        ID,
        "Astro content pipeline ESLint plugin package is missing",
        format!(
            "`{rel_path}` must list `{DEPENDENCY_NAME}` in dependencies or devDependencies. Astro content boundary checks are delegated to that ESLint plugin; setup checks do not own this package."
        ),
        Some(rel_path),
    ));
}
