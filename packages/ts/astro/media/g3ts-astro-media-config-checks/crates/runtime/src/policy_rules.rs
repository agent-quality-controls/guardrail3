use g3ts_astro_media_types::{
    G3TsAstroMediaIntegrationContractInput, G3TsAstroMediaPolicySnapshot,
};
use guardrail3_check_types::G3CheckResult;

const STRICT_ID: &str = "g3ts-astro-media/strict-policy-configured";
const PATHS_ID: &str = "g3ts-astro-media/policy-paths-valid";

pub(crate) fn check(
    contract: &G3TsAstroMediaIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::media_policy_rel_path(&contract.astro_policy);
    let Some(policy) = crate::support::parsed_media_policy(&contract.astro_policy) else {
        results.push(crate::support::error(
            STRICT_ID,
            "Astro media strict policy is missing",
            format!(
                "`{}` must define `[ts.astro.media]` with explicit favicon, app icon, default social image, source globs, helper modules, content image component policy, and checked image extensions. G3TS does not invent media defaults.",
                rel_path.unwrap_or("guardrail3-ts.toml")
            ),
            rel_path,
        ));
        return;
    };

    check_required_fields(policy, results);
    check_paths(policy, results);
}

fn check_required_fields(policy: &G3TsAstroMediaPolicySnapshot, results: &mut Vec<G3CheckResult>) {
    let missing = missing_fields(policy);
    if missing.is_empty() {
        results.push(crate::support::info(
            STRICT_ID,
            "Astro media strict policy is configured",
            format!(
                "`{}` defines the required `[ts.astro.media]` fields with no hidden defaults.",
                policy.rel_path
            ),
            &policy.rel_path,
        ));
        return;
    }

    results.push(crate::support::error(
        STRICT_ID,
        "Astro media strict policy is incomplete",
        format!(
            "`{}` must define non-empty `[ts.astro.media]` fields: {}.",
            policy.rel_path,
            missing.join(", ")
        ),
        Some(&policy.rel_path),
    ));
}

fn missing_fields(policy: &G3TsAstroMediaPolicySnapshot) -> Vec<&'static str> {
    let mut missing = Vec::new();
    if policy.favicon.trim().is_empty() {
        missing.push("favicon");
    }
    if policy.app_icons.is_empty() {
        missing.push("app_icons");
    }
    if policy.default_social_image.trim().is_empty() {
        missing.push("default_social_image");
    }
    if policy.public_source_globs.is_empty() {
        missing.push("public_source_globs");
    }
    if policy.media_helper_modules.is_empty() {
        missing.push("media_helper_modules");
    }
    if policy.approved_media_helpers.is_empty() {
        missing.push("approved_media_helpers");
    }
    if policy.content_image_components.is_empty() {
        missing.push("content_image_components");
    }
    if policy.content_image_key_props.is_empty() {
        missing.push("content_image_key_props");
    }
    if policy.banned_image_source_props.is_empty() {
        missing.push("banned_image_source_props");
    }
    if policy.banned_image_alt_props.is_empty() {
        missing.push("banned_image_alt_props");
    }
    if policy.checked_image_extensions.is_empty() {
        missing.push("checked_image_extensions");
    }
    if policy.metadata_image_property_names.is_empty() {
        missing.push("metadata_image_property_names");
    }
    missing
}

fn check_paths(policy: &G3TsAstroMediaPolicySnapshot, results: &mut Vec<G3CheckResult>) {
    let invalid = policy
        .app_icons
        .iter()
        .chain(std::iter::once(&policy.favicon))
        .chain(std::iter::once(&policy.default_social_image))
        .chain(policy.public_source_globs.iter())
        .chain(policy.media_helper_modules.iter())
        .chain(policy.allowed_public_image_paths.iter())
        .filter(|path| invalid_path(path))
        .cloned()
        .collect::<Vec<_>>();

    if invalid.is_empty() {
        results.push(crate::support::info(
            PATHS_ID,
            "Astro media policy paths are app-relative",
            format!("`{}` media asset paths, helper paths, and globs are app-relative or root-relative asset paths.", policy.rel_path),
            &policy.rel_path,
        ));
        return;
    }

    results.push(crate::support::error(
        PATHS_ID,
        "Astro media policy path is invalid",
        format!(
            "`{}` contains invalid media paths/globs: {}. Paths must be app-relative, non-empty, and must not traverse with `..`.",
            policy.rel_path,
            invalid.join(", ")
        ),
        Some(&policy.rel_path),
    ));
}

fn invalid_path(path: &str) -> bool {
    path.trim().is_empty() || path.split('/').any(|part| part == "..")
}
