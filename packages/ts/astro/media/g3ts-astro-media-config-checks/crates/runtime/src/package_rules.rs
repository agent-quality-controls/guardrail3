use g3ts_astro_media_types::G3TsAstroMediaIntegrationContractInput;
use guardrail3_check_types::G3CheckResult;

/// Internal constant `ASSETS_ID`.
const ASSETS_ID: &str = "g3ts-astro-media/media-assets-package-present";
/// Internal constant `POLICY_PLUGIN_ID`.
const POLICY_PLUGIN_ID: &str = "g3ts-astro-media/media-policy-plugin-package-present";
/// Internal constant `ESLINT_COMMENTS_ID`.
const ESLINT_COMMENTS_ID: &str = "g3ts-astro-media/eslint-comments-plugin-package-present";
/// Internal constant `ASSETS_PACKAGE`.
const ASSETS_PACKAGE: &str = "g3ts-astro-media-assets";
/// Internal constant `ASSETS_VERSION`.
const ASSETS_VERSION: &str = "0.1.2";
/// Internal constant `POLICY_PACKAGE`.
const POLICY_PACKAGE: &str = "g3ts-eslint-plugin-astro-media-policy";
/// Internal constant `POLICY_VERSION`.
const POLICY_VERSION: &str = "0.1.10";
/// Internal constant `ESLINT_COMMENTS_PACKAGE`.
const ESLINT_COMMENTS_PACKAGE: &str = "@eslint-community/eslint-plugin-eslint-comments";
/// Internal constant `ESLINT_COMMENTS_VERSION`.
const ESLINT_COMMENTS_VERSION: &str = "4.7.1";

/// Internal function `check`.
pub(crate) fn check(
    contract: &G3TsAstroMediaIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    check_package(
        contract,
        results,
        ASSETS_ID,
        ASSETS_PACKAGE,
        ASSETS_VERSION,
        "Astro build-time media asset existence checks",
    );
    check_package(
        contract,
        results,
        POLICY_PLUGIN_ID,
        POLICY_PACKAGE,
        POLICY_VERSION,
        "ESLint media source misuse checks",
    );
    check_package(
        contract,
        results,
        ESLINT_COMMENTS_ID,
        ESLINT_COMMENTS_PACKAGE,
        ESLINT_COMMENTS_VERSION,
        "ESLint disable escape-hatch visibility checks",
    );
}

/// Internal function `check_package`.
fn check_package(
    contract: &G3TsAstroMediaIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
    id: &str,
    package_name: &str,
    package_version: &str,
    purpose: &str,
) {
    let rel_path = crate::support::package_rel_path(&contract.package);
    if crate::support::package_has_dependency(&contract.package, package_name) {
        results.push(crate::support::info(
            id,
            "Astro media delegated package is installed",
            format!("`{rel_path}` lists `{package_name}` for {purpose}. G3TS verifies the package contract instead of reimplementing that work."),
            rel_path,
        ));
        return;
    }

    results.push(crate::support::error(
        id,
        "Astro media delegated package is missing",
        format!(
            "`{rel_path}` must list `{package_name}` at exact version `{package_version}` in dependencies or devDependencies for {purpose}. Syncpack owns the exact version pin; this rule verifies the package is present for the media contract."
        ),
        Some(rel_path),
    ));
}
