use g3ts_astro_seo_types::{G3TsAstroConfigSurfaceState, G3TsAstroSeoIntegrationContractInput};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "g3ts-astro-seo/sitemap-integration-present";
const GENERATOR_DEPENDENCY_NAME: &str = "@astrojs/sitemap";
const AUDITOR_DEPENDENCY_NAME: &str = "g3ts-astro-sitemap-auditor";

pub(crate) fn check(
    contract: &G3TsAstroSeoIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::astro_config_rel_path(&contract.astro_config);
    let (has_generator, has_auditor) = match &contract.astro_config {
        G3TsAstroConfigSurfaceState::Parsed { snapshot } => (
            crate::support::astro_config_has_integration(snapshot, GENERATOR_DEPENDENCY_NAME),
            sitemap_auditor_config_is_strict(snapshot),
        ),
        G3TsAstroConfigSurfaceState::Missing { .. }
        | G3TsAstroConfigSurfaceState::Unreadable { .. }
        | G3TsAstroConfigSurfaceState::ParseError { .. } => (false, false),
    };

    if has_generator && has_auditor {
        if let Some(rel_path) = rel_path {
            results.push(crate::support::info(
                ID,
                "Astro sitemap integration is wired",
                format!("`{rel_path}` wires `{GENERATOR_DEPENDENCY_NAME}` and `{AUDITOR_DEPENDENCY_NAME}` integrations with static required auditor config."),
                rel_path,
            ));
        }
        return;
    }

    results.push(crate::support::error(
            ID,
            "Astro sitemap integration is not wired",
            format!(
                    "This Astro app must include integrations imported from `{GENERATOR_DEPENDENCY_NAME}` and `{AUDITOR_DEPENDENCY_NAME}` in `astro.config`. `{AUDITOR_DEPENDENCY_NAME}` must be called with a static object containing HTTPS `site` and `trailingSlash: \"always\" | \"never\"`."
            ),
            rel_path,
        ));
}

fn sitemap_auditor_config_is_strict(
    snapshot: &g3ts_astro_seo_types::G3TsAstroConfigSurfaceSnapshot,
) -> bool {
    let Some(value) =
        crate::support::astro_config_integration_first_arg(snapshot, AUDITOR_DEPENDENCY_NAME)
    else {
        return false;
    };
    let Some(properties) = crate::support::object_properties(value) else {
        return false;
    };
    if crate::support::object_has_duplicate_keys(properties) {
        return false;
    }
    if !crate::support::object_has_only_allowed_keys(
        properties,
        &[
            "site",
            "indexFilename",
            "trailingSlash",
            "allowedMissingRoutes",
            "allowedExtraUrls",
            "ignoredHtmlFiles",
        ],
    ) {
        return false;
    }
    let Some(site) = crate::support::property_string(properties, "site") else {
        return false;
    };
    url::Url::parse(site).is_ok_and(|url| url.scheme() == "https")
        && matches!(
            crate::support::property_string(properties, "trailingSlash"),
            Some("always" | "never")
        )
}
