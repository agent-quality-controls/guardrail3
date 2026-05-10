use g3ts_astro_i18n_types::G3TsAstroI18nIntegrationContractInput;
use guardrail3_check_types::G3CheckResult;

/// Internal constant `I18NEXT_ID`.
const I18NEXT_ID: &str = "g3ts-astro-i18n/i18next-plugin-package-present";
/// Internal constant `POLICY_PLUGIN_ID`.
const POLICY_PLUGIN_ID: &str = "g3ts-astro-i18n/i18n-policy-plugin-package-present";
/// Internal constant `ESLINT_COMMENTS_ID`.
const ESLINT_COMMENTS_ID: &str = "g3ts-astro-i18n/eslint-comments-plugin-package-present";
/// Internal constant `I18NEXT_PACKAGE`.
const I18NEXT_PACKAGE: &str = "eslint-plugin-i18next";
/// Internal constant `I18NEXT_VERSION`.
const I18NEXT_VERSION: &str = "6.1.4";
/// Internal constant `POLICY_PACKAGE`.
const POLICY_PACKAGE: &str = "g3ts-eslint-plugin-astro-i18n-policy";
/// Internal constant `POLICY_VERSION`.
const POLICY_VERSION: &str = "0.1.2";
/// Internal constant `ESLINT_COMMENTS_PACKAGE`.
const ESLINT_COMMENTS_PACKAGE: &str = "@eslint-community/eslint-plugin-eslint-comments";
/// Internal constant `ESLINT_COMMENTS_VERSION`.
const ESLINT_COMMENTS_VERSION: &str = "4.7.1";

/// Internal function `check`.
pub(crate) fn check(
    contract: &G3TsAstroI18nIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    check_package(
        contract,
        results,
        I18NEXT_ID,
        I18NEXT_PACKAGE,
        I18NEXT_VERSION,
    );
    check_package(
        contract,
        results,
        POLICY_PLUGIN_ID,
        POLICY_PACKAGE,
        POLICY_VERSION,
    );
    check_package(
        contract,
        results,
        ESLINT_COMMENTS_ID,
        ESLINT_COMMENTS_PACKAGE,
        ESLINT_COMMENTS_VERSION,
    );
}

/// Internal function `check_package`.
fn check_package(
    contract: &G3TsAstroI18nIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
    id: &str,
    package_name: &str,
    package_version: &str,
) {
    let rel_path = crate::support::package_rel_path(&contract.package);
    if crate::support::package_has_dependency(&contract.package, package_name) {
        results.push(crate::support::info(
            id,
            "Astro i18n delegated package is installed",
            format!("`{rel_path}` lists `{package_name}`. Astro i18n guardrails delegate source checks to ESLint packages instead of parsing source in G3TS."),
            rel_path,
        ));
        return;
    }

    results.push(crate::support::error(
        id,
        "Astro i18n delegated package is missing",
        format!(
            "`{rel_path}` must list `{package_name}` at exact version `{package_version}` in dependencies or devDependencies. Syncpack owns the exact version pin; this rule verifies the package is present for the i18n contract."
        ),
        Some(rel_path),
    ));
}
