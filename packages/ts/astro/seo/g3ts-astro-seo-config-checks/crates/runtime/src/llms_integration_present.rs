use g3ts_astro_seo_types::{G3TsAstroConfigSurfaceState, G3TsAstroSeoIntegrationContractInput};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "g3ts-astro-seo/llms-integration-present";
const GENERATOR_DEPENDENCY_NAME: &str = "g3ts-astro-llms-generator";
const AUDITOR_DEPENDENCY_NAME: &str = "g3ts-astro-llms-auditor";

pub(crate) fn check(
    contract: &G3TsAstroSeoIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    if !crate::support::strict_ai_readable_enabled(&contract.astro_policy) {
        return;
    }

    let rel_path = crate::support::astro_config_rel_path(&contract.astro_config);
    let (has_generator, has_auditor) = match &contract.astro_config {
        G3TsAstroConfigSurfaceState::Parsed { snapshot } => (
            llms_generator_config_is_strict(snapshot),
            llms_auditor_config_is_strict(snapshot),
        ),
        G3TsAstroConfigSurfaceState::Missing { .. }
        | G3TsAstroConfigSurfaceState::Unreadable { .. }
        | G3TsAstroConfigSurfaceState::ParseError { .. } => (false, false),
    };

    if has_generator && has_auditor {
        if let Some(rel_path) = rel_path {
            results.push(crate::support::info(
                ID,
                "Astro llms integration is wired",
                format!("`{rel_path}` wires `{GENERATOR_DEPENDENCY_NAME}` and `{AUDITOR_DEPENDENCY_NAME}` with static required config for strict AI-readable output."),
                rel_path,
            ));
        }
        return;
    }

    results.push(crate::support::error(
        ID,
        "Astro llms integration is not wired",
        format!("Strict AI-readable policy requires `{GENERATOR_DEPENDENCY_NAME}` and `{AUDITOR_DEPENDENCY_NAME}` as Astro integrations. Both integrations must use static config objects; the auditor config must include HTTPS `site`, `requiredSections`, `requiredRoutePatterns`, `allowedExternalUrls`, `allowedNonPageUrls`, and `ignoredHtmlFiles`."),
        rel_path,
    ));
}

fn llms_generator_config_is_strict(
    snapshot: &g3ts_astro_seo_types::G3TsAstroConfigSurfaceSnapshot,
) -> bool {
    let Some(value) =
        crate::support::astro_config_integration_first_arg(snapshot, GENERATOR_DEPENDENCY_NAME)
    else {
        return false;
    };
    let Some(properties) = crate::support::object_properties(value) else {
        return false;
    };
    if crate::support::object_has_duplicate_keys(properties) {
        return false;
    }
    if !crate::support::object_has_only_allowed_keys(properties, &["title", "site", "sections"]) {
        return false;
    }
    crate::support::property_string(properties, "title").is_some()
        && crate::support::property_string(properties, "site")
            .is_some_and(|site| url::Url::parse(site).is_ok_and(|url| url.scheme() == "https"))
        && crate::support::property_array(properties, "sections")
            .is_some_and(|sections| !sections.is_empty())
}

fn llms_auditor_config_is_strict(
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
            "requiredSections",
            "requiredRoutePatterns",
            "allowedExternalUrls",
            "allowedNonPageUrls",
            "ignoredHtmlFiles",
        ],
    ) {
        return false;
    }
    crate::support::property_string(properties, "site")
        .is_some_and(|site| url::Url::parse(site).is_ok_and(|url| url.scheme() == "https"))
        && [
            "requiredSections",
            "requiredRoutePatterns",
            "allowedExternalUrls",
            "allowedNonPageUrls",
            "ignoredHtmlFiles",
        ]
        .iter()
        .all(|key| crate::support::property_array(properties, key).is_some())
}
