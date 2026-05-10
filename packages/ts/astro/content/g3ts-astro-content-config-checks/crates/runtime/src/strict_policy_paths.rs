use g3ts_astro_content_types::{
    G3TsAstroContentIntegrationContractInput, G3TsAstroContentPolicySnapshot,
};
use guardrail3_check_types::G3CheckResult;
use std::path::{Component, Path};

/// Internal constant `CONTENT_ID`.
const CONTENT_ID: &str = "g3ts-astro-content/strict-policy-paths";

/// Strategy function returning invalid policy entries for a given snapshot.
type InvalidEntriesFn = fn(&G3TsAstroContentPolicySnapshot) -> Vec<String>;

/// Bundle of formatting strings and the entry validator used by a policy-path check.
struct PolicyPathsCheck {
    /// Info-result title used when no invalid entries are found.
    info_title: &'static str,
    /// Error-result title used when invalid entries are found.
    error_title: &'static str,
    /// Info-result message template with one `{}` placeholder for `rel_path`.
    info_message: &'static str,
    /// Error-result message template with two `{}` placeholders: `rel_path` and joined errors.
    error_message: &'static str,
    /// Strategy function that returns invalid entries for the policy snapshot.
    invalid_entries: InvalidEntriesFn,
    /// Stable identifier for the rule emitting findings.
    id: &'static str,
}

/// Internal function `check_content`.
pub(crate) fn check_content(
    contract: &G3TsAstroContentIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    check_policy_paths(
        contract,
        &PolicyPathsCheck {
            info_title: "Astro strict content policy paths are structurally valid",
            error_title: "Astro strict content policy paths are invalid",
            info_message: "`{}` uses app-relative nested Astro content policy paths without parent traversal in `[ts.astro.routes]` and `[ts.astro.content]`.",
            error_message: "`{}` has invalid nested Astro content policy paths: {}. Use app-relative values only, do not use absolute paths, `..`, or backslashes, and keep `[ts.astro.content].root` separate from `[ts.astro.content].adapters`.",
            invalid_entries: invalid_content_policy_entries,
            id: CONTENT_ID,
        },
        results,
    );
}

/// Internal function `check_policy_paths`.
fn check_policy_paths(
    contract: &G3TsAstroContentIntegrationContractInput,
    spec: &PolicyPathsCheck,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::content_policy_rel_path(&contract.astro_policy);
    let Some(policy) = crate::support::parsed_content_policy(&contract.astro_policy) else {
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

/// Internal function `format_message`.
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

/// Internal function `invalid_content_policy_entries`.
fn invalid_content_policy_entries(policy: &G3TsAstroContentPolicySnapshot) -> Vec<String> {
    let mut errors = Vec::new();
    collect_invalid_list(
        "[ts.astro.routes].content",
        &policy.content_routes,
        &mut errors,
    );
    collect_invalid_list(
        "[ts.astro.routes].non_content",
        &policy.non_content_routes,
        &mut errors,
    );
    collect_invalid_list(
        "[ts.astro.routes].endpoints",
        &policy.endpoints,
        &mut errors,
    );
    collect_invalid_optional_dir(
        "[ts.astro.content].root",
        policy.content_root.as_ref(),
        &mut errors,
    );
    collect_invalid_helper_list(
        "[ts.astro.content].adapters",
        &policy.content_adapters,
        &mut errors,
    );

    if let Some(content_root) = &policy.content_root {
        for content_adapter in &policy.content_adapters {
            if dirs_overlap(content_root, content_adapter) {
                errors.push(format!(
                    "[ts.astro.content].root overlaps [ts.astro.content].adapters `{content_adapter}`"
                ));
            }
        }
    }
    errors
}

/// Internal function `collect_invalid_list`.
fn collect_invalid_list(field: &str, values: &[String], errors: &mut Vec<String>) {
    for value in values {
        if !is_app_relative_pattern(value) {
            errors.push(format!("{field} contains `{value}`"));
        }
    }
}

/// Internal function `collect_invalid_optional_dir`.
fn collect_invalid_optional_dir(field: &str, value: Option<&String>, errors: &mut Vec<String>) {
    let Some(value) = value else {
        return;
    };

    if !is_app_relative_dir(value) {
        errors.push(format!("{field} is `{value}`"));
    }
}

/// Internal function `collect_invalid_helper_list`.
fn collect_invalid_helper_list(field: &str, values: &[String], errors: &mut Vec<String>) {
    for value in values {
        if !is_app_relative_dir(value) {
            errors.push(format!("{field} contains `{value}`"));
        }
    }
}

/// Internal function `is_app_relative_pattern`.
fn is_app_relative_pattern(value: &str) -> bool {
    !value.is_empty()
        && !value.contains('\\')
        && !Path::new(value).is_absolute()
        && !has_parent_component(value)
}

/// Internal function `is_app_relative_dir`.
fn is_app_relative_dir(value: &str) -> bool {
    is_app_relative_pattern(value) && !contains_glob_metachar(value)
}

/// Internal function `has_parent_component`.
fn has_parent_component(value: &str) -> bool {
    Path::new(value)
        .components()
        .any(|component| component == Component::ParentDir)
}

/// Internal function `contains_glob_metachar`.
fn contains_glob_metachar(value: &str) -> bool {
    value
        .chars()
        .any(|character| matches!(character, '*' | '?' | '[' | ']' | '{' | '}'))
}

/// Internal function `dirs_overlap`.
fn dirs_overlap(left: &str, right: &str) -> bool {
    let left = left.trim_end_matches('/');
    let right = right.trim_end_matches('/');
    left == right
        || right
            .strip_prefix(left)
            .is_some_and(|suffix| suffix.starts_with('/'))
        || left
            .strip_prefix(right)
            .is_some_and(|suffix| suffix.starts_with('/'))
}
