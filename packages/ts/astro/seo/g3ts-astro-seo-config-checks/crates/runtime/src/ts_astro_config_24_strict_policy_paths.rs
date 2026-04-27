use g3ts_astro_seo_types::{G3TsAstroSeoIntegrationContractInput, G3TsAstroSeoPolicySnapshot};
use guardrail3_check_types::G3CheckResult;
use std::path::{Component, Path};

const SEO_ID: &str = "TS-ASTRO-SEO-CONFIG-24";

pub(crate) fn check_seo(
    contract: &G3TsAstroSeoIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    check_policy_paths(
        contract,
        "Astro SEO policy paths are structurally valid",
        "Astro SEO policy paths are invalid",
        "`{}` uses app-relative `[ts.astro.seo]` helper paths without parent traversal.",
        "`{}` has invalid Astro SEO policy paths: {}. Use app-relative values only, do not use absolute paths, `..`, backslashes, or glob metacharacters.",
        invalid_seo_policy_entries,
        SEO_ID,
        results,
    );
}

fn check_policy_paths(
    contract: &G3TsAstroSeoIntegrationContractInput,
    info_title: &str,
    error_title: &str,
    info_message: &str,
    error_message: &str,
    invalid_entries: fn(&G3TsAstroSeoPolicySnapshot) -> Vec<String>,
    id: &str,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::seo_policy_rel_path(&contract.astro_policy);
    let Some(policy) = crate::support::parsed_seo_policy(&contract.astro_policy) else {
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
