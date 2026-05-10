use g3ts_astro_media_types::{
    G3TsAstroMediaIntegrationContractInput, G3TsAstroMediaPolicySnapshot,
};
use guardrail3_check_types::G3CheckResult;

/// Internal constant `STRICT_ID`.
const STRICT_ID: &str = "g3ts-astro-media/strict-policy-configured";
/// Internal constant `PATHS_ID`.
const PATHS_ID: &str = "g3ts-astro-media/policy-paths-valid";

/// Internal function `check`.
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
                "`{rel_path}` must define `[ts.astro.media]` with explicit `favicon`, `app_icons`, `default_social_image`, `allow_svg_icons`, `public_source_globs`, `media_helper_modules`, `approved_media_helpers`, `content_image_components`, `content_image_key_props`, `banned_image_source_props`, `banned_image_alt_props`, `allowed_public_image_paths`, `checked_image_extensions`, and `metadata_image_property_names`. G3TS does not invent media defaults."
            ),
            Some(rel_path),
        ));
        return;
    };

    check_required_fields(policy, results);
    check_paths(policy, results);
}

/// Internal function `check_required_fields`.
fn check_required_fields(policy: &G3TsAstroMediaPolicySnapshot, results: &mut Vec<G3CheckResult>) {
    let missing = missing_fields(policy);
    if missing.is_empty() && policy.extra_fields.is_empty() {
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

    if !policy.extra_fields.is_empty() {
        results.push(crate::support::error(
            STRICT_ID,
            "Astro media strict policy has unknown fields",
            format!(
                "`{}` contains unknown `[ts.astro.media]` fields: {}. Delete them or move them to the family that owns them; media policy accepts only the documented media keys.",
                policy.rel_path,
                policy.extra_fields.join(", ")
            ),
            Some(&policy.rel_path),
        ));
    }

    if missing.is_empty() {
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

/// Internal function `missing_fields`.
fn missing_fields(policy: &G3TsAstroMediaPolicySnapshot) -> Vec<&'static str> {
    let mut missing = Vec::new();
    if policy.favicon.trim().is_empty() {
        missing.push("favicon");
    }
    if has_empty_value(&policy.app_icons) {
        missing.push("app_icons");
    }
    if policy.default_social_image.trim().is_empty() {
        missing.push("default_social_image");
    }
    if policy.allow_svg_icons.is_none() {
        missing.push("allow_svg_icons");
    }
    if has_empty_value(&policy.public_source_globs) {
        missing.push("public_source_globs");
    }
    if has_empty_value(&policy.media_helper_modules) {
        missing.push("media_helper_modules");
    }
    if has_empty_value(&policy.approved_media_helpers) {
        missing.push("approved_media_helpers");
    }
    if has_empty_value(&policy.content_image_components) {
        missing.push("content_image_components");
    }
    if has_empty_value(&policy.content_image_key_props) {
        missing.push("content_image_key_props");
    }
    if has_empty_value(&policy.banned_image_source_props) {
        missing.push("banned_image_source_props");
    }
    if has_empty_value(&policy.banned_image_alt_props) {
        missing.push("banned_image_alt_props");
    }
    if has_empty_value(&policy.allowed_public_image_paths) {
        missing.push("allowed_public_image_paths");
    }
    if has_empty_value(&policy.checked_image_extensions) {
        missing.push("checked_image_extensions");
    }
    if has_empty_value(&policy.metadata_image_property_names) {
        missing.push("metadata_image_property_names");
    }
    missing
}

/// Internal function `has_empty_value`.
fn has_empty_value(values: &[String]) -> bool {
    values.is_empty() || values.iter().any(|value| value.trim().is_empty())
}

/// Internal function `check_paths`.
fn check_paths(policy: &G3TsAstroMediaPolicySnapshot, results: &mut Vec<G3CheckResult>) {
    let invalid = policy
        .app_icons
        .iter()
        .map(|path| ("app_icons", path))
        .chain(std::iter::once(("favicon", &policy.favicon)))
        .chain(std::iter::once((
            "default_social_image",
            &policy.default_social_image,
        )))
        .chain(
            policy
                .allowed_public_image_paths
                .iter()
                .map(|path| ("allowed_public_image_paths", path)),
        )
        .filter(|(_, path)| invalid_output_asset_path(path))
        .chain(
            policy
                .public_source_globs
                .iter()
                .map(|path| ("public_source_globs", path))
                .chain(
                    policy
                        .media_helper_modules
                        .iter()
                        .map(|path| ("media_helper_modules", path)),
                )
                .filter(|(_, path)| invalid_app_relative_path(path)),
        )
        .map(|(field, path)| format!("{field}={path}"))
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
            "`{}` contains invalid media paths/globs: {}. `favicon`, `app_icons`, `default_social_image`, and `allowed_public_image_paths` must be root-relative public output paths like `/favicon.svg`; `public_source_globs` and `media_helper_modules` must be app-relative paths/globs like `src/media`. No value may be empty, external, use backslashes, encoded separators, or `..` traversal.",
            policy.rel_path,
            invalid.join(", ")
        ),
        Some(&policy.rel_path),
    ));
}

/// Internal function `invalid_output_asset_path`.
fn invalid_output_asset_path(path: &str) -> bool {
    let trimmed = path.trim();
    invalid_common_path(trimmed)
        || !trimmed.starts_with('/')
        || trimmed.starts_with("//")
        || external_url(trimmed)
}

/// Internal function `invalid_app_relative_path`.
fn invalid_app_relative_path(path: &str) -> bool {
    let trimmed = path.trim();
    invalid_common_path(trimmed) || trimmed.starts_with('/') || external_url(trimmed)
}

/// Internal function `invalid_common_path`.
fn invalid_common_path(path: &str) -> bool {
    path.is_empty()
        || path.contains('\\')
        || path.to_ascii_lowercase().contains("%2f")
        || path.to_ascii_lowercase().contains("%5c")
        || has_encoded_parent_segment(path)
        || path.split('/').any(|part| part == "..")
}

/// Internal function `has_encoded_parent_segment`.
fn has_encoded_parent_segment(path: &str) -> bool {
    path.split('/').any(|part| {
        let lower = part.to_ascii_lowercase();
        lower == "%2e%2e" || lower == "%2e." || lower == ".%2e"
    })
}

/// Internal function `external_url`.
fn external_url(path: &str) -> bool {
    path.contains("://")
        || path.split_once(':').is_some_and(|(scheme, _)| {
            !scheme.is_empty() && scheme.chars().all(|c| c.is_ascii_alphabetic())
        })
}
