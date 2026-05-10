use g3ts_astro_setup_types::G3TsAstroSetupIntegrationContractInput;
use guardrail3_check_types::G3CheckResult;

/// Stable rule identifier surfaced in findings.
const ID: &str = "g3ts-astro-setup/astro-package-present";
/// Static rule data.
const PACKAGE_NAME: &str = "astro";

/// Validates the rule and pushes findings into `results`.
pub(crate) fn check(
    contract: &G3TsAstroSetupIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::package_rel_path(&contract.package);
    if crate::support::package_has_dependency(&contract.package, PACKAGE_NAME) {
        results.push(crate::support::info(
            ID,
            "astro package present",
            format!("`{rel_path}` includes `{PACKAGE_NAME}`."),
            rel_path,
        ));
        return;
    }

    let message = format!(
        "`{rel_path}` does not list `{PACKAGE_NAME}` in dependencies or devDependencies. Add `{PACKAGE_NAME}` to `{rel_path}`. Without that dependency entry, this app can drift away from the Astro framework contract without the package surface showing it."
    );

    results.push(crate::support::error(
        ID,
        "Astro app package is missing `astro`",
        message,
        Some(rel_path),
    ));
}
