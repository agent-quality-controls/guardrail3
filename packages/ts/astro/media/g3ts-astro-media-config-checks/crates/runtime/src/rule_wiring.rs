use g3ts_astro_media_types::{
    G3TsAstroMediaEslintPluginContractInput, G3TsAstroMediaEslintSurfaceState,
};
use guardrail3_check_types::G3CheckResult;

/// Internal constant `POLICY_ID`.
const POLICY_ID: &str = "g3ts-astro-media/media-policy-plugin-wired";
/// Internal constant `RAW_PATH_RULE_ID`.
const RAW_PATH_RULE_ID: &str = "g3ts-astro-media/no-raw-public-image-paths-rule";
/// Internal constant `ALT_RULE_ID`.
const ALT_RULE_ID: &str = "g3ts-astro-media/no-inline-image-alt-rule";
/// Internal constant `IMAGE_KEY_RULE_ID`.
const IMAGE_KEY_RULE_ID: &str = "g3ts-astro-media/require-content-image-key-rule";
/// Internal constant `HELPER_RULE_ID`.
const HELPER_RULE_ID: &str = "g3ts-astro-media/require-approved-media-helper-rule";
/// Internal constant `DISABLE_ID`.
const DISABLE_ID: &str = "g3ts-astro-media/protected-media-rule-disables-restricted";
/// Internal constant `RAW_PATH_RULE`.
const RAW_PATH_RULE: &str = "astro-media-policy/no-raw-public-image-paths";
/// Internal constant `ALT_RULE`.
const ALT_RULE: &str = "astro-media-policy/no-inline-image-alt";
/// Internal constant `IMAGE_KEY_RULE`.
const IMAGE_KEY_RULE: &str = "astro-media-policy/require-content-image-key";
/// Internal constant `HELPER_RULE`.
const HELPER_RULE: &str = "astro-media-policy/require-approved-media-helper";
/// Internal constant `PROTECTED_DISABLES`.
const PROTECTED_DISABLES: [&str; 1] = ["astro-media-policy/*"];

/// Internal function `check`.
pub(crate) fn check(
    contract: &G3TsAstroMediaEslintPluginContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    check_policy_plugin(contract, results);
    check_rule(contract, results, RAW_PATH_RULE_ID, RAW_PATH_RULE);
    check_rule(contract, results, ALT_RULE_ID, ALT_RULE);
    check_rule(contract, results, IMAGE_KEY_RULE_ID, IMAGE_KEY_RULE);
    check_rule(contract, results, HELPER_RULE_ID, HELPER_RULE);
    check_disable_protection(contract, results);
}

/// Internal function `check_policy_plugin`.
fn check_policy_plugin(
    contract: &G3TsAstroMediaEslintPluginContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::eslint_rel_path(&contract.config);
    if public_config(contract).is_some_and(|snapshot| {
        snapshot
            .public_plugins
            .iter()
            .any(|plugin| plugin == "astro-media-policy")
            && snapshot
                .public_plugin_package_names
                .get("astro-media-policy")
                .is_some_and(|packages| {
                    packages
                        .iter()
                        .any(|package| package == "g3ts-eslint-plugin-astro-media-policy")
                })
    }) {
        results.push(crate::support::info(
            POLICY_ID,
            "Astro media policy plugin is wired",
            format!("`{rel_path}` activates `astro-media-policy` from `g3ts-eslint-plugin-astro-media-policy`."),
            rel_path,
        ));
        return;
    }

    results.push(crate::support::error(
        POLICY_ID,
        "Astro media policy plugin is not wired",
        format!(
            "`{rel_path}` must activate plugin namespace `astro-media-policy` from `g3ts-eslint-plugin-astro-media-policy` on `[ts.astro.media].public_source_globs`."
        ),
        Some(rel_path),
    ));
}

/// Internal function `check_rule`.
fn check_rule(
    contract: &G3TsAstroMediaEslintPluginContractInput,
    results: &mut Vec<G3CheckResult>,
    id: &str,
    rule_name: &str,
) {
    let rel_path = crate::support::eslint_rel_path(&contract.config);
    if public_config(contract).is_some_and(|snapshot| {
        snapshot
            .public_media_policy_rules
            .iter()
            .any(|rule| rule == rule_name)
    }) {
        results.push(crate::support::info(
            id,
            "Astro media policy rule is effective",
            format!(
                "`{rel_path}` activates `{rule_name}` at error severity with explicit options."
            ),
            rel_path,
        ));
        return;
    }

    results.push(crate::support::error(
        id,
        "Astro media policy rule is not effective",
        format!(
            "`{rel_path}` must activate `{rule_name}` at `error` with explicit options matching `[ts.astro.media]`: `publicSourceGlobs`, `mediaHelperModules`, `approvedMediaHelpers`, `contentImageComponents`, `contentImageKeyProps`, `bannedImageSourceProps`, `bannedImageAltProps`, `allowedPublicImagePaths`, `checkedImageExtensions`, and `metadataImagePropertyNames`."
        ),
        Some(rel_path),
    ));
}

/// Internal function `check_disable_protection`.
fn check_disable_protection(
    contract: &G3TsAstroMediaEslintPluginContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::eslint_rel_path(&contract.config);
    if public_config(contract).is_some_and(|snapshot| {
        PROTECTED_DISABLES.iter().all(|pattern| {
            snapshot
                .public_restricted_disable_patterns
                .iter()
                .any(|candidate| candidate == pattern)
        })
    }) {
        results.push(crate::support::info(
            DISABLE_ID,
            "Astro media delegated rule disables are restricted",
            format!("`{rel_path}` protects media delegated rules with `@eslint-community/eslint-comments/no-restricted-disable`."),
            rel_path,
        ));
        return;
    }

    results.push(crate::support::error(
        DISABLE_ID,
        "Astro media delegated rule disables are not restricted",
        format!(
            "`{rel_path}` must configure `@eslint-community/eslint-comments/no-restricted-disable` for `astro-media-policy/*`."
        ),
        Some(rel_path),
    ));
}

/// Internal function `fn`.
const fn public_config(
    contract: &G3TsAstroMediaEslintPluginContractInput,
) -> Option<&g3ts_astro_media_types::G3TsAstroMediaEslintSurfaceSnapshot> {
    match &contract.config {
        G3TsAstroMediaEslintSurfaceState::Parsed { snapshot }
            if snapshot.public_probe_present && !snapshot.public_probe_ignored =>
        {
            Some(snapshot)
        }
        G3TsAstroMediaEslintSurfaceState::Missing { .. }
        | G3TsAstroMediaEslintSurfaceState::Unreadable { .. }
        | G3TsAstroMediaEslintSurfaceState::ParseError { .. }
        | G3TsAstroMediaEslintSurfaceState::Parsed { .. } => None,
    }
}
