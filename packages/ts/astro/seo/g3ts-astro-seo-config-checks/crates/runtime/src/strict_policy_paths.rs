use g3ts_astro_seo_types::{G3TsAstroSeoIntegrationContractInput, G3TsAstroSeoPolicySnapshot};
use guardrail3_check_types::G3CheckResult;
use std::path::{Component, Path};

/// Static rule data.
const SEO_ID: &str = "g3ts-astro-seo/strict-policy-paths";

/// Function pointer that returns invalid entries for a policy snapshot.
type InvalidEntriesFn = fn(&G3TsAstroSeoPolicySnapshot) -> Vec<String>;

/// Static-text inputs to a policy-paths check.
struct PolicyPathsCheckSpec {
    /// Stable rule identifier surfaced in findings.
    id: &'static str,
    /// Title used when the check passes.
    info_title: &'static str,
    /// Title used when the check fails.
    error_title: &'static str,
    /// Message template for inventory findings.
    info_message: &'static str,
    /// Message template for error findings.
    error_message: &'static str,
    /// Computes invalid entries for the policy snapshot.
    invalid_entries: InvalidEntriesFn,
}

/// Internal helper exported within the runtime crate.
pub(crate) fn check_seo(
    contract: &G3TsAstroSeoIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    check_policy_paths(
        contract,
        &PolicyPathsCheckSpec {
            id: SEO_ID,
            info_title: "Astro SEO policy paths are structurally valid",
            error_title: "Astro SEO policy paths are invalid",
            info_message: "`{}` uses app-relative `[ts.astro.seo]` helper paths without parent traversal.",
            error_message: "`{}` has invalid Astro SEO policy paths: {}. Use app-relative values only, do not use absolute paths, `..`, backslashes, or glob metacharacters.",
            invalid_entries: invalid_seo_policy_entries,
        },
        results,
    );
}

/// Internal helper used by the rule.
fn check_policy_paths(
    contract: &G3TsAstroSeoIntegrationContractInput,
    spec: &PolicyPathsCheckSpec,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::seo_policy_rel_path(&contract.astro_policy);
    let Some(policy) = crate::support::parsed_seo_policy(&contract.astro_policy) else {
        return;
    };

    let errors = (spec.invalid_entries)(policy);
    if errors.is_empty() {
        results.push(crate::support::info(
            spec.id,
            spec.info_title,
            format_message(spec.info_message, &policy.rel_path, None),
            &policy.rel_path,
        ));
        return;
    }

    results.push(crate::support::error(
        spec.id,
        spec.error_title,
        format_message(spec.error_message, rel_path, Some(&errors.join("; "))),
        Some(rel_path),
    ));
}

/// Internal helper used by the rule.
fn format_message(template: &str, rel_path: &str, errors: Option<&str>) -> String {
    errors.map_or_else(
        || template.replacen("{}", rel_path, 1),
        |errors| {
            template
                .replacen("{}", rel_path, 1)
                .replacen("{}", errors, 1)
        },
    )
}

/// Internal helper used by the rule.
fn invalid_seo_policy_entries(policy: &G3TsAstroSeoPolicySnapshot) -> Vec<String> {
    let mut errors = Vec::new();
    collect_invalid_helper_list(
        "[ts.astro.seo].metadata_helpers",
        &policy.metadata_helpers,
        &mut errors,
    );
    collect_invalid_helper_list(
        "[ts.astro.seo].json_ld_helpers",
        &policy.json_ld_helpers,
        &mut errors,
    );
    errors
}

/// Internal helper used by the rule.
fn collect_invalid_helper_list(field: &str, values: &[String], errors: &mut Vec<String>) {
    for value in values {
        if !is_app_relative_dir(value) {
            errors.push(format!("{field} contains `{value}`"));
        }
    }
}

/// Internal helper used by the rule.
fn is_app_relative_pattern(value: &str) -> bool {
    !value.is_empty()
        && !value.contains('\\')
        && !Path::new(value).is_absolute()
        && !has_parent_component(value)
}

/// Internal helper used by the rule.
fn is_app_relative_dir(value: &str) -> bool {
    is_app_relative_pattern(value) && !contains_glob_metachar(value)
}

/// Internal helper used by the rule.
fn has_parent_component(value: &str) -> bool {
    Path::new(value)
        .components()
        .any(|component| component == Component::ParentDir)
}

/// Internal helper used by the rule.
fn contains_glob_metachar(value: &str) -> bool {
    value
        .chars()
        .any(|character| matches!(character, '*' | '?' | '[' | ']' | '{' | '}'))
}
