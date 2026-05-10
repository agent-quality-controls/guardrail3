use g3ts_astro_mdx_types::{G3TsAstroMdxIntegrationContractInput, G3TsAstroMdxPolicySnapshot};
use guardrail3_check_types::G3CheckResult;
use std::path::{Component, Path};

/// Internal constant `MDX_ID`.
const MDX_ID: &str = "g3ts-astro-mdx/strict-policy-paths";

/// Strategy function returning invalid policy entries for a given snapshot.
type InvalidEntriesFn = fn(&G3TsAstroMdxPolicySnapshot) -> Vec<String>;

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

/// Internal function `check_mdx`.
pub(crate) fn check_mdx(
    contract: &G3TsAstroMdxIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    check_policy_paths(
        contract,
        &PolicyPathsCheck {
            info_title: "Astro MDX policy paths are structurally valid",
            error_title: "Astro MDX policy paths are invalid",
            info_message: "`{}` uses app-relative `[ts.astro.mdx].component_maps` paths without parent traversal.",
            error_message: "`{}` has invalid Astro MDX policy paths: {}. Use app-relative values only, do not use absolute paths, `..`, backslashes, or glob metacharacters.",
            invalid_entries: invalid_mdx_policy_entries,
            id: MDX_ID,
        },
        results,
    );
}

/// Internal function `check_policy_paths`.
fn check_policy_paths(
    contract: &G3TsAstroMdxIntegrationContractInput,
    spec: &PolicyPathsCheck,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::mdx_policy_rel_path(&contract.astro_policy);
    let Some(policy) = crate::support::parsed_mdx_policy(&contract.astro_policy) else {
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

/// Internal function `invalid_mdx_policy_entries`.
fn invalid_mdx_policy_entries(policy: &G3TsAstroMdxPolicySnapshot) -> Vec<String> {
    let mut errors = Vec::new();
    collect_invalid_helper_list(
        "[ts.astro.mdx].component_maps",
        &policy.mdx_component_maps,
        &mut errors,
    );
    errors
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
