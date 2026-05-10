use g3ts_astro_setup_types::G3TsAstroSetupIntegrationContractInput;
use guardrail3_check_types::G3CheckResult;

/// Stable rule identifier surfaced in findings.
const ID: &str = "g3ts-astro-setup/eslint-comments-plugin-package-present";
/// Static rule data.
const PACKAGE_NAME: &str = "@eslint-community/eslint-plugin-eslint-comments";

/// Validates the rule and pushes findings into `results`.
pub(crate) fn check(
    contract: &G3TsAstroSetupIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::package_rel_path(&contract.package);

    if crate::support::package_has_dependency(&contract.package, PACKAGE_NAME) {
        results.push(crate::support::info(
            ID,
            "Astro app installs eslint-comments plugin",
            format!(
                "`{rel_path}` installs `{PACKAGE_NAME}`. Astro apps need this plugin so `eslint-disable` escape hatches require descriptions, unused disables fail, and protected delegated rules cannot be silently disabled."
            ),
            rel_path,
        ));
        return;
    }

    results.push(crate::support::error(
        ID,
        "Astro app is missing eslint-comments plugin",
        format!(
            "`{rel_path}` must install `{PACKAGE_NAME}` in dependencies or devDependencies. G3TS delegates ESLint directive policy to that package instead of parsing comments itself."
        ),
        Some(rel_path),
    ));
}
