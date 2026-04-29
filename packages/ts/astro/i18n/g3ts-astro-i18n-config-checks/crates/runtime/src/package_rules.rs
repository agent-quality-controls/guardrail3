use g3ts_astro_i18n_types::G3TsAstroI18nIntegrationContractInput;
use guardrail3_check_types::G3CheckResult;

const I18NEXT_ID: &str = "g3ts-astro-i18n/i18next-plugin-package-present";
const POLICY_PLUGIN_ID: &str = "g3ts-astro-i18n/i18n-policy-plugin-package-present";
const ESLINT_COMMENTS_ID: &str = "g3ts-astro-i18n/eslint-comments-plugin-package-present";
const I18NEXT_PACKAGE: &str = "eslint-plugin-i18next";
const POLICY_PACKAGE: &str = "g3ts-eslint-plugin-astro-i18n-policy";
const ESLINT_COMMENTS_PACKAGE: &str = "@eslint-community/eslint-plugin-eslint-comments";

pub(crate) fn check(contract: &G3TsAstroI18nIntegrationContractInput, results: &mut Vec<G3CheckResult>) {
    check_package(contract, results, I18NEXT_ID, I18NEXT_PACKAGE);
    check_package(contract, results, POLICY_PLUGIN_ID, POLICY_PACKAGE);
    check_package(contract, results, ESLINT_COMMENTS_ID, ESLINT_COMMENTS_PACKAGE);
}

fn check_package(
    contract: &G3TsAstroI18nIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
    id: &str,
    package_name: &str,
) {
    let rel_path = crate::support::package_rel_path(&contract.package);
    if crate::support::package_has_dependency(&contract.package, package_name) {
        if let Some(rel_path) = rel_path {
            results.push(crate::support::info(
                id,
                "Astro i18n delegated package is installed",
                format!("`{rel_path}` lists `{package_name}`. Astro i18n guardrails delegate source checks to ESLint packages instead of parsing source in G3TS."),
                rel_path,
            ));
        }
        return;
    }

    results.push(crate::support::error(
        id,
        "Astro i18n delegated package is missing",
        format!(
            "`{}` must list `{package_name}` in dependencies or devDependencies. Astro i18n source enforcement is delegated to this package; G3TS only verifies it is installed and wired.",
            rel_path.unwrap_or("package.json")
        ),
        rel_path,
    ));
}
