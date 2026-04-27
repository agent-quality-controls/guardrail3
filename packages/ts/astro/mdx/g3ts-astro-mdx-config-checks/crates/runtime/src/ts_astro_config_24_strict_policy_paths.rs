use g3ts_astro_types::{G3TsAstroMdxIntegrationContractInput, G3TsAstroPolicySnapshot};
use guardrail3_check_types::G3CheckResult;
use std::path::{Component, Path};

const MDX_ID: &str = "TS-ASTRO-MDX-CONFIG-24";

pub(crate) fn check_mdx(contracts: &[G3TsAstroMdxIntegrationContractInput], results: &mut Vec<G3CheckResult>) {
    check_policy_paths(
        contracts,
        "Astro MDX policy paths are structurally valid",
        "Astro MDX policy paths are invalid",
        "`{}` uses app-relative `[ts.astro.mdx].component_maps` paths without parent traversal.",
        "`{}` has invalid Astro MDX policy paths: {}. Use app-relative values only, do not use absolute paths, `..`, backslashes, or glob metacharacters.",
        invalid_mdx_policy_entries,
        MDX_ID,
        results,
    );
}

fn check_policy_paths(
    contracts: &[G3TsAstroMdxIntegrationContractInput],
    info_title: &str,
    error_title: &str,
    info_message: &str,
    error_message: &str,
    invalid_entries: fn(&G3TsAstroPolicySnapshot) -> Vec<String>,
    id: &str,
    results: &mut Vec<G3CheckResult>,
) {
    for contract in contracts {
        let rel_path = g3ts_astro_check_support::core::astro_policy_rel_path(contract);
        let Some(policy) = g3ts_astro_check_support::core::parsed_astro_policy(contract) else {
            continue;
        };

        let errors = invalid_entries(policy);
        if errors.is_empty() {
            results.push(g3ts_astro_check_support::core::info(
                id,
                info_title,
                format_message(info_message, &policy.rel_path, None),
                &policy.rel_path,
            ));
            continue;
        }

        results.push(g3ts_astro_check_support::core::error(
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

fn invalid_mdx_policy_entries(policy: &G3TsAstroPolicySnapshot) -> Vec<String> {
    let mut errors = Vec::new();
    collect_invalid_helper_list(
        "[ts.astro.mdx].component_maps",
        &policy.mdx_component_maps,
        &mut errors,
    );
    errors
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
