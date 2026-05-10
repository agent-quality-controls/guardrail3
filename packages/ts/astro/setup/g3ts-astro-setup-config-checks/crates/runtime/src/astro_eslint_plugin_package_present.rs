use g3ts_astro_setup_types::G3TsAstroSetupIntegrationContractInput;
use guardrail3_check_types::G3CheckResult;

/// Stable rule identifier surfaced in findings.
const ID: &str = "g3ts-astro-setup/astro-eslint-plugin-package-present";
/// Required npm dependency name.
const DEPENDENCY_NAME: &str = "eslint-plugin-astro";

/// Validates the rule and pushes findings into `results`.
pub(crate) fn check(
    contract: &G3TsAstroSetupIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::package_rel_path(&contract.package);
    if crate::support::package_has_dependency(&contract.package, DEPENDENCY_NAME) {
        results.push(crate::support::info(
            ID,
            "astro ESLint plugin package present",
            format!("`{rel_path}` includes `{DEPENDENCY_NAME}`."),
            rel_path,
        ));
        return;
    }

    let message = format!(
        "`{rel_path}` does not list `{DEPENDENCY_NAME}` in dependencies or devDependencies. Add `{DEPENDENCY_NAME}` to `{rel_path}`. Astro source files need the Astro ESLint plugin so Astro-specific lint rules can run."
    );
    results.push(crate::support::error(
        ID,
        "Astro app package is missing `eslint-plugin-astro`",
        message,
        Some(rel_path),
    ));
}
