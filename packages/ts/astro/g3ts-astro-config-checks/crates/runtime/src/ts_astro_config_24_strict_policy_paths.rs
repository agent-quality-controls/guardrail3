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
                    format!("`{}` uses app-relative nested Astro policy paths without parent traversal in `[ts.astro.routes]`, `[ts.astro.content]`, `[ts.astro.mdx]`, `[ts.astro.seo]`, and `[ts.astro.state]`.", policy.rel_path),
                &policy.rel_path,
            ));
            continue;
        }

        results.push(crate::support::error(
            ID,
            "Astro strict content policy paths are invalid",
            format!(
                "`{}` has invalid nested Astro policy paths: {}. Use app-relative values only, do not use absolute paths, `..`, or backslashes, and keep `[ts.astro.content].root` separate from `[ts.astro.content].adapters` and helper surfaces.",
                rel_path.unwrap_or("guardrail3-ts.toml"),
                errors.join("; ")
            ),
            rel_path,
        ));
    }
}

fn invalid_policy_entries(policy: &G3TsAstroPolicySnapshot) -> Vec<String> {
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
    collect_invalid_helper_list(
        "[ts.astro.mdx].component_maps",
        &policy.mdx_component_maps,
        &mut errors,
    );
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
    collect_invalid_list(
        "[ts.astro.state].forbidden",
        &policy.forbidden_state,
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

    if let Some(content_root) = &policy.content_root {
        collect_content_root_overlaps(
            "[ts.astro.mdx].component_maps",
            content_root,
            &policy.mdx_component_maps,
            &mut errors,
        );
        collect_content_root_overlaps(
            "[ts.astro.seo].metadata_helpers",
            content_root,
            &policy.metadata_helpers,
            &mut errors,
        );
        collect_content_root_overlaps(
            "[ts.astro.seo].json_ld_helpers",
            content_root,
            &policy.json_ld_helpers,
            &mut errors,
        );
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

fn collect_content_root_overlaps(
    field: &str,
    content_root: &str,
    values: &[String],
    errors: &mut Vec<String>,
) {
    for value in values {
        if dirs_overlap(content_root, value) {
            errors.push(format!(
                "[ts.astro.content].root overlaps {field} `{value}`"
            ));
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
