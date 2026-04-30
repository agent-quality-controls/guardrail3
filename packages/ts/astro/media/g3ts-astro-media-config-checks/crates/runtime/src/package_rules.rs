use g3ts_astro_media_types::G3TsAstroMediaIntegrationContractInput;
use guardrail3_check_types::G3CheckResult;

const ASSETS_ID: &str = "g3ts-astro-media/media-assets-package-present";
const POLICY_PLUGIN_ID: &str = "g3ts-astro-media/media-policy-plugin-package-present";
const ESLINT_COMMENTS_ID: &str = "g3ts-astro-media/eslint-comments-plugin-package-present";
const ASSETS_PACKAGE: &str = "g3ts-astro-media-assets";
const ASSETS_VERSION: &str = "0.1.2";
const POLICY_PACKAGE: &str = "g3ts-eslint-plugin-astro-media-policy";
const POLICY_VERSION: &str = "0.1.10";
const ESLINT_COMMENTS_PACKAGE: &str = "@eslint-community/eslint-plugin-eslint-comments";
const ESLINT_COMMENTS_VERSION: &str = "4.7.1";

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
        if let Some(rel_path) = rel_path {
            results.push(crate::support::info(
                id,
                "Astro media delegated package is installed",
                format!("`{rel_path}` lists `{package_name}` for {purpose}. G3TS verifies the package contract instead of reimplementing that work."),
                rel_path,
            ));
        }
        return;
    }

    results.push(crate::support::error(
        id,
        "Astro media delegated package is missing",
        format!(
            "`{}` must list `{package_name}` at exact version `{package_version}` in dependencies or devDependencies for {purpose}. Syncpack owns the exact version pin; this rule verifies the package is present for the media contract.",
            rel_path.unwrap_or("package.json")
        ),
        rel_path,
    ));
}
