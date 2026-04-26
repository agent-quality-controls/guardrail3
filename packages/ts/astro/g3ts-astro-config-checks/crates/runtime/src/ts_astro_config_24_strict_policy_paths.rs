use g3ts_astro_types::{G3TsAstroConfigChecksInput, G3TsAstroPolicySnapshot};
use guardrail3_check_types::G3CheckResult;
use std::path::{Component, Path};

const ID: &str = "TS-ASTRO-CONFIG-24";

pub(crate) fn check(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.integration_contracts {
        let rel_path = crate::support::astro_policy_rel_path(contract);
        let Some(policy) = crate::support::parsed_astro_policy(contract) else {
            continue;
        };

        let errors = invalid_policy_entries(policy);
        if errors.is_empty() {
            results.push(crate::support::info(
                ID,
                "Astro strict content policy paths are structurally valid",
                format!(
                    "`{}` uses app-relative `content_routes`, `non_content_routes`, `endpoints`, `content_root`, `content_adapter`, and `forbidden_state` values without parent traversal.",
                    policy.rel_path
                ),
                &policy.rel_path,
            ));
            continue;
        }

        results.push(crate::support::error(
            ID,
            "Astro strict content policy paths are invalid",
            format!(
                "`{}` has invalid `[ts.astro]` paths: {}. Use app-relative values only, do not use absolute paths, `..`, or backslashes, and keep `content_root` separate from `content_adapter`.",
                rel_path.unwrap_or("guardrail3-ts.toml"),
                errors.join("; ")
            ),
            rel_path,
        ));
    }
}

fn invalid_policy_entries(policy: &G3TsAstroPolicySnapshot) -> Vec<String> {
    let mut errors = Vec::new();
    collect_invalid_list("content_routes", &policy.content_routes, &mut errors);
    collect_invalid_list(
        "non_content_routes",
        &policy.non_content_routes,
        &mut errors,
    );
    collect_invalid_list("endpoints", &policy.endpoints, &mut errors);
    collect_invalid_list("forbidden_state", &policy.forbidden_state, &mut errors);
    collect_invalid_optional_dir("content_root", &policy.content_root, &mut errors);
    collect_invalid_optional_dir("content_adapter", &policy.content_adapter, &mut errors);

    if let (Some(content_root), Some(content_adapter)) =
        (&policy.content_root, &policy.content_adapter)
    {
        if dirs_overlap(content_root, content_adapter) {
            errors.push("content_root overlaps content_adapter".to_owned());
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
