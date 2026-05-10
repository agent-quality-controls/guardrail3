use g3ts_astro_media_types::G3TsAstroMediaIntegrationContractInput;
use guardrail3_check_types::G3CheckResult;

/// Internal constant `ID`.
const ID: &str = "g3ts-astro-media/media-assets-integration-wired";
/// Internal constant `PACKAGE_NAME`.
const PACKAGE_NAME: &str = "g3ts-astro-media-assets";

/// Internal function `check`.
pub(crate) fn check(
    contract: &G3TsAstroMediaIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::astro_config_rel_path(&contract.astro_config);
    if integration_matches_policy(contract) {
        results.push(crate::support::info(
            ID,
            "Astro media assets integration is wired",
            format!("`{rel_path}` wires `{PACKAGE_NAME}` with favicon, appIcons, defaultSocialImage, and allowSvgIcons matching `[ts.astro.media]`."),
            rel_path,
        ));
        return;
    }

    results.push(crate::support::error(
            ID,
            "Astro media assets integration is not wired",
            format!(
            "`{rel_path}` must add `g3tsAstroMediaAssets(...)` from `{PACKAGE_NAME}` to Astro integrations. Its `favicon`, `appIcons`, `defaultSocialImage`, and `allowSvgIcons` options must exactly match `[ts.astro.media]`. `favicon`, `appIcons`, and `defaultSocialImage` are root-relative public output paths like `/favicon.svg`; they are not app-relative source paths."
        ),
        Some(rel_path),
    ));
}

/// Internal function `integration_matches_policy`.
fn integration_matches_policy(contract: &G3TsAstroMediaIntegrationContractInput) -> bool {
    let Some(config) = crate::support::parsed_astro_config(&contract.astro_config) else {
        return false;
    };
    let Some(policy) = crate::support::parsed_media_policy(&contract.astro_policy) else {
        return false;
    };
    let Some(first_arg) = crate::support::astro_config_integration_first_arg(config, PACKAGE_NAME)
    else {
        return false;
    };
    let Some(properties) = crate::support::object_properties(first_arg) else {
        return false;
    };

    crate::support::object_has_only_allowed_keys(
        properties,
        &["favicon", "appIcons", "defaultSocialImage", "allowSvgIcons"],
    ) && crate::support::property_string(properties, "favicon") == Some(policy.favicon.as_str())
        && crate::support::property_string(properties, "defaultSocialImage")
            == Some(policy.default_social_image.as_str())
        && policy.allow_svg_icons.is_some_and(|expected| {
            crate::support::property_bool(properties, "allowSvgIcons") == Some(expected)
        })
        && crate::support::property_string_array(properties, "appIcons")
            .is_some_and(|actual| actual == policy.app_icons)
}
