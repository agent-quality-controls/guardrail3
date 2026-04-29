use g3ts_astro_i18n_types::{
    G3TsAstroI18nIntegrationContractInput, G3TsAstroI18nPolicySnapshot,
};
use guardrail3_check_types::G3CheckResult;

const STRICT_ID: &str = "g3ts-astro-i18n/strict-policy-configured";
const PATHS_ID: &str = "g3ts-astro-i18n/policy-paths-valid";

pub(crate) fn check(contract: &G3TsAstroI18nIntegrationContractInput, results: &mut Vec<G3CheckResult>) {
    let rel_path = crate::support::i18n_policy_rel_path(&contract.astro_policy);
    let Some(policy) = crate::support::parsed_i18n_policy(&contract.astro_policy) else {
        results.push(crate::support::error(
            STRICT_ID,
            "Astro i18n strict policy is missing",
            format!(
                "`{}` must define `[ts.astro.i18n]` with explicit `locales`, `public_source_globs`, content route prefixes, approved helpers, and content image policy. G3TS does not invent i18n defaults.",
                rel_path.unwrap_or("guardrail3-ts.toml")
            ),
            rel_path,
        ));
        return;
    };

    check_required_fields(policy, results);
    check_paths(policy, results);
}

fn check_required_fields(policy: &G3TsAstroI18nPolicySnapshot, results: &mut Vec<G3CheckResult>) {
    let missing = missing_fields(policy);
    if missing.is_empty() {
        results.push(crate::support::info(
            STRICT_ID,
            "Astro i18n strict policy is configured",
            format!("`{}` defines the required `[ts.astro.i18n]` fields with no hidden defaults.", policy.rel_path),
            &policy.rel_path,
        ));
        return;
    }

    results.push(crate::support::error(
        STRICT_ID,
        "Astro i18n strict policy is incomplete",
        format!(
            "`{}` must define non-empty `[ts.astro.i18n]` fields: {}.",
            policy.rel_path,
            missing.join(", ")
        ),
        Some(&policy.rel_path),
    ));
}

fn missing_fields(policy: &G3TsAstroI18nPolicySnapshot) -> Vec<&'static str> {
    let mut missing = Vec::new();
    if policy.locales.is_empty() {
        missing.push("locales");
    }
    if policy.public_source_globs.is_empty() {
        missing.push("public_source_globs");
    }
    if policy.require_locale_prefix_for_content_routes && policy.content_route_prefixes.is_empty() {
        missing.push("content_route_prefixes");
    }
    if policy.approved_internal_link_helpers.is_empty() {
        missing.push("approved_internal_link_helpers");
    }
    if policy.approved_localized_link_components.is_empty() {
        missing.push("approved_localized_link_components");
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
    missing
}

fn check_paths(policy: &G3TsAstroI18nPolicySnapshot, results: &mut Vec<G3CheckResult>) {
    let invalid = policy
        .approved_date_format_helpers
        .iter()
        .chain(policy.approved_number_format_helpers.iter())
        .chain(policy.public_source_globs.iter())
        .chain(policy.helper_source_globs.iter())
        .filter(|path| invalid_path(path))
        .cloned()
        .collect::<Vec<_>>();

    if invalid.is_empty() {
        results.push(crate::support::info(
            PATHS_ID,
            "Astro i18n policy paths are app-relative",
            format!("`{}` i18n helper paths and globs are app-relative and non-empty.", policy.rel_path),
            &policy.rel_path,
        ));
        return;
    }

    results.push(crate::support::error(
        PATHS_ID,
        "Astro i18n policy path is invalid",
        format!(
            "`{}` contains invalid i18n paths/globs: {}. Paths must be app-relative, non-empty, and must not traverse with `..`.",
            policy.rel_path,
            invalid.join(", ")
        ),
        Some(&policy.rel_path),
    ));
}

fn invalid_path(path: &str) -> bool {
    path.trim().is_empty() || path.starts_with('/') || path.split('/').any(|part| part == "..")
}
