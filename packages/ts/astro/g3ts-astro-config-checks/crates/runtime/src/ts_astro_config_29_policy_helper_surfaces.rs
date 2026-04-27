use g3ts_astro_types::{G3TsAstroConfigChecksInput, G3TsAstroPolicySnapshot};
use guardrail3_check_types::G3CheckResult;
use std::path::{Component, Path};

const ID: &str = "TS-ASTRO-CONFIG-29";

pub(crate) fn check(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.integration_contracts {
        let rel_path = crate::support::astro_policy_rel_path(contract);
        let Some(policy) = crate::support::parsed_astro_policy(contract) else {
            continue;
        };

        let violations = helper_surface_violations(policy);
        if violations.is_empty() {
            results.push(crate::support::info(
                ID,
                "Astro strict content policy declares approved helper surfaces",
                format!(
                    "`{}` declares non-empty app-relative `[ts.astro.mdx].component_maps`, `[ts.astro.seo].metadata_helpers`, and `[ts.astro.seo].json_ld_helpers`, and those helper surfaces do not overlap `[ts.astro.content].root`.",
                    policy.rel_path
                ),
                &policy.rel_path,
            ));
            continue;
        }

        results.push(crate::support::error(
            ID,
            "Astro strict content policy is missing approved helper surfaces",
            format!(
                "`{}` must declare non-empty app-relative `[ts.astro.mdx].component_maps`, `[ts.astro.seo].metadata_helpers`, and `[ts.astro.seo].json_ld_helpers` that do not overlap `[ts.astro.content].root`. Violations: {}. These are approved module surfaces, not hardcoded required filenames.",
                rel_path.unwrap_or("guardrail3-ts.toml"),
                violations.join(", ")
            ),
            rel_path,
        ));
    }
}

fn helper_surface_violations(policy: &G3TsAstroPolicySnapshot) -> Vec<String> {
    let mut violations = Vec::new();
    collect_helper_surface_violations(
        "[ts.astro.mdx].component_maps",
        &policy.mdx_component_maps,
        &policy.content_root,
        &mut violations,
    );
    collect_helper_surface_violations(
        "[ts.astro.seo].metadata_helpers",
        &policy.metadata_helpers,
        &policy.content_root,
        &mut violations,
    );
    collect_helper_surface_violations(
        "[ts.astro.seo].json_ld_helpers",
        &policy.json_ld_helpers,
        &policy.content_root,
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
