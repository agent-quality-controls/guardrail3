use g3ts_astro_types::{G3TsAstroConfigChecksInput, G3TsAstroPolicySnapshot};
use guardrail3_check_types::G3CheckResult;
use std::path::{Component, Path};

const SEO_ID: &str = "TS-ASTRO-SEO-CONFIG-29";

pub(crate) fn check_seo(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    check_helper_surfaces(
        input,
        "Astro strict content policy declares approved SEO helper surfaces",
        "Astro strict content policy is missing approved SEO helper surfaces",
        "`{}` declares non-empty app-relative `[ts.astro.seo].metadata_helpers` and `[ts.astro.seo].json_ld_helpers`.",
        "`{}` must declare non-empty app-relative `[ts.astro.seo].metadata_helpers` and `[ts.astro.seo].json_ld_helpers`. Violations: {}. These are approved module surfaces, not hardcoded required filenames.",
        seo_helper_surface_violations,
        SEO_ID,
        results,
    );
}

fn check_helper_surfaces(
    input: &G3TsAstroConfigChecksInput,
    info_title: &str,
    error_title: &str,
    info_message: &str,
    error_message: &str,
    violations_for_policy: fn(&G3TsAstroPolicySnapshot) -> Vec<String>,
    id: &str,
    results: &mut Vec<G3CheckResult>,
) {
    for contract in &input.integration_contracts {
        let rel_path = g3ts_astro_check_support::core::astro_policy_rel_path(contract);
        let Some(policy) = g3ts_astro_check_support::core::parsed_astro_policy(contract) else {
            continue;
        };

        let violations = violations_for_policy(policy);
        if violations.is_empty() {
            results.push(g3ts_astro_check_support::core::info(
                id,
                info_title,
                info_message.replacen("{}", &policy.rel_path, 1),
                &policy.rel_path,
            ));
            continue;
        }

        results.push(g3ts_astro_check_support::core::error(
            id,
            error_title,
            error_message
                .replacen("{}", rel_path.unwrap_or("guardrail3-ts.toml"), 1)
                .replacen("{}", &violations.join(", "), 1),
            rel_path,
        ));
    }
}

fn seo_helper_surface_violations(policy: &G3TsAstroPolicySnapshot) -> Vec<String> {
    let mut violations = Vec::new();
    collect_helper_surface_violations(
        "[ts.astro.seo].metadata_helpers",
        &policy.metadata_helpers,
        &None,
        &mut violations,
    );
    collect_helper_surface_violations(
        "[ts.astro.seo].json_ld_helpers",
        &policy.json_ld_helpers,
        &None,
        &mut violations,
    );
    violations
}

fn collect_helper_surface_violations(
    field: &str,
    values: &[String],
    content_root: &Option<String>,
    violations: &mut Vec<String>,
) {
    if values.is_empty() {
        violations.push(format!("{field} is empty"));
        return;
    }

    for value in values {
        if !is_app_relative_dir(value) {
            violations.push(format!("{field} contains invalid path `{value}`"));
        }

        if content_root
            .as_deref()
            .is_some_and(|root| dirs_overlap(root, value))
        {
            violations.push(format!(
                "{field} overlaps [ts.astro.content].root at `{value}`"
            ));
        }
    }
}

fn is_app_relative_dir(value: &str) -> bool {
    !value.is_empty()
        && !value.contains('\\')
        && !Path::new(value).is_absolute()
        && !has_parent_component(value)
        && !contains_glob_metachar(value)
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
