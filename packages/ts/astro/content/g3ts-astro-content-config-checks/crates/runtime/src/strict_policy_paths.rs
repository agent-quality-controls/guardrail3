use g3ts_astro_content_types::{
    G3TsAstroContentIntegrationContractInput, G3TsAstroContentPolicySnapshot,
};
use guardrail3_check_types::G3CheckResult;
use std::path::{Component, Path};

const CONTENT_ID: &str = "g3ts-astro-content/strict-policy-paths";

pub(crate) fn check_content(
    contract: &G3TsAstroContentIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    check_policy_paths(
        contract,
        "Astro strict content policy paths are structurally valid",
        "Astro strict content policy paths are invalid",
        "`{}` uses app-relative nested Astro content policy paths without parent traversal in `[ts.astro.routes]` and `[ts.astro.content]`.",
        "`{}` has invalid nested Astro content policy paths: {}. Use app-relative values only, do not use absolute paths, `..`, or backslashes, and keep `[ts.astro.content].root` separate from `[ts.astro.content].adapters`.",
        invalid_content_policy_entries,
        CONTENT_ID,
        results,
    );
}

fn check_policy_paths(
    contract: &G3TsAstroContentIntegrationContractInput,
    info_title: &str,
    error_title: &str,
    info_message: &str,
    error_message: &str,
    invalid_entries: fn(&G3TsAstroContentPolicySnapshot) -> Vec<String>,
    id: &str,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::content_policy_rel_path(&contract.astro_policy);
    let Some(policy) = crate::support::parsed_content_policy(&contract.astro_policy) else {
        return;
    };

    let errors = invalid_entries(policy);
    if errors.is_empty() {
        results.push(crate::support::info(
            id,
            info_title,
            format_message(info_message, &policy.rel_path, None),
            &policy.rel_path,
        ));
        return;
    }

    results.push(crate::support::error(
        id,
        error_title,
        format_message(
            error_message,
            rel_path.unwrap_or("guardrail3-ts.toml"),
            Some(&errors.join("; ")),
        ),
        rel_path,
    ));
}

fn format_message(template: &str, rel_path: &str, errors: Option<&str>) -> String {
    if let Some(errors) = errors {
        template
            .replacen("{}", rel_path, 1)
            .replacen("{}", errors, 1)
    } else {
        template.replacen("{}", rel_path, 1)
    }
}

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
    collect_invalid_optional_dir("[ts.astro.content].root", &policy.content_root, &mut errors);
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

fn collect_invalid_list(field: &str, values: &[String], errors: &mut Vec<String>) {
    for value in values {
        if !is_app_relative_pattern(value) {
            errors.push(format!("{field} contains `{value}`"));
        }
    }
}

fn collect_invalid_optional_dir(field: &str, value: &Option<String>, errors: &mut Vec<String>) {
    let Some(value) = value else {
        return;
    };

    if !is_app_relative_dir(value) {
        errors.push(format!("{field} is `{value}`"));
    }
}

fn collect_invalid_helper_list(field: &str, values: &[String], errors: &mut Vec<String>) {
    for value in values {
        if !is_app_relative_dir(value) {
            errors.push(format!("{field} contains `{value}`"));
        }
    }
}

fn is_app_relative_pattern(value: &str) -> bool {
    !value.is_empty()
        && !value.contains('\\')
        && !Path::new(value).is_absolute()
        && !has_parent_component(value)
}

fn is_app_relative_dir(value: &str) -> bool {
    is_app_relative_pattern(value) && !contains_glob_metachar(value)
}

fn has_parent_component(value: &str) -> bool {
    Path::new(value)
        .components()
        .any(|component| component == Component::ParentDir)
}

fn contains_glob_metachar(value: &str) -> bool {
    value
        .chars()
        .any(|character| matches!(character, '*' | '?' | '[' | ']' | '{' | '}'))
}

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
