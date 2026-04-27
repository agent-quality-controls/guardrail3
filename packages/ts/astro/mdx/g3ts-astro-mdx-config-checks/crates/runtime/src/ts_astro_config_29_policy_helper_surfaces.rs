use g3ts_astro_mdx_types::{G3TsAstroMdxIntegrationContractInput, G3TsAstroMdxPolicySnapshot};
use guardrail3_check_types::G3CheckResult;
use std::path::{Component, Path};

const MDX_ID: &str = "TS-ASTRO-MDX-CONFIG-29";

pub(crate) fn check_mdx(
    contract: &G3TsAstroMdxIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    check_helper_surfaces(
        contract,
        "Astro strict content policy declares approved MDX component-map surfaces",
        "Astro strict content policy is missing approved MDX component-map surfaces",
        "`{}` declares non-empty app-relative `[ts.astro.mdx].component_maps`.",
        "`{}` must declare non-empty app-relative `[ts.astro.mdx].component_maps`. Violations: {}. These are approved module surfaces, not hardcoded required filenames.",
        mdx_helper_surface_violations,
        MDX_ID,
        results,
    );
}

fn check_helper_surfaces(
    contract: &G3TsAstroMdxIntegrationContractInput,
    info_title: &str,
    error_title: &str,
    info_message: &str,
    error_message: &str,
    violations_for_policy: fn(&G3TsAstroMdxPolicySnapshot) -> Vec<String>,
    id: &str,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::mdx_policy_rel_path(&contract.astro_policy);
    let Some(policy) = crate::support::parsed_mdx_policy(&contract.astro_policy) else {
        return;
    };

    let violations = violations_for_policy(policy);
    if violations.is_empty() {
        results.push(crate::support::info(
            id,
            info_title,
            info_message.replacen("{}", &policy.rel_path, 1),
            &policy.rel_path,
        ));
        return;
    }

    results.push(crate::support::error(
        id,
        error_title,
        error_message
            .replacen("{}", rel_path.unwrap_or("guardrail3-ts.toml"), 1)
            .replacen("{}", &violations.join(", "), 1),
        rel_path,
    ));
}

fn mdx_helper_surface_violations(policy: &G3TsAstroMdxPolicySnapshot) -> Vec<String> {
    let mut violations = Vec::new();
    collect_helper_surface_violations(
        "[ts.astro.mdx].component_maps",
        &policy.mdx_component_maps,
        &mut violations,
    );
    violations
}

fn collect_helper_surface_violations(field: &str, values: &[String], violations: &mut Vec<String>) {
    if values.is_empty() {
        violations.push(format!("{field} is empty"));
        return;
    }

    for value in values {
        if !is_app_relative_dir(value) {
            violations.push(format!("{field} contains invalid path `{value}`"));
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
