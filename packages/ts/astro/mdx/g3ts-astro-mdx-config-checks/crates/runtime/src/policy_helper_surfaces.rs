use g3ts_astro_mdx_types::{G3TsAstroMdxIntegrationContractInput, G3TsAstroMdxPolicySnapshot};
use guardrail3_check_types::G3CheckResult;
use std::path::{Component, Path};

/// Internal constant `MDX_ID`.
const MDX_ID: &str = "g3ts-astro-mdx/policy-helper-surfaces";

/// Strategy function returning helper-surface violations for a given snapshot.
type ViolationsFn = fn(&G3TsAstroMdxPolicySnapshot) -> Vec<String>;

/// Bundle of formatting strings and the violation collector used by a helper-surface check.
struct HelperSurfacesCheck {
    /// Info-result title used when no violations are found.
    info_title: &'static str,
    /// Error-result title used when violations are found.
    error_title: &'static str,
    /// Info-result message template with one `{}` placeholder for `rel_path`.
    info_message: &'static str,
    /// Error-result message template with two `{}` placeholders: `rel_path` and joined violations.
    error_message: &'static str,
    /// Strategy function that returns helper-surface violations.
    violations_for_policy: ViolationsFn,
    /// Stable identifier for the rule emitting findings.
    id: &'static str,
}

/// Internal function `check_mdx`.
pub(crate) fn check_mdx(
    contract: &G3TsAstroMdxIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    check_helper_surfaces(
        contract,
        &HelperSurfacesCheck {
            info_title: "Astro strict content policy declares approved MDX component-map surfaces",
            error_title: "Astro strict content policy is missing approved MDX component-map surfaces",
            info_message: "`{}` declares non-empty app-relative `[ts.astro.mdx].component_maps`.",
            error_message: "`{}` must declare non-empty app-relative `[ts.astro.mdx].component_maps`. Violations: {}. These are approved module surfaces, not hardcoded required filenames.",
            violations_for_policy: mdx_helper_surface_violations,
            id: MDX_ID,
        },
        results,
    );
}

/// Internal function `check_helper_surfaces`.
fn check_helper_surfaces(
    contract: &G3TsAstroMdxIntegrationContractInput,
    spec: &HelperSurfacesCheck,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::mdx_policy_rel_path(&contract.astro_policy);
    let Some(policy) = crate::support::parsed_mdx_policy(&contract.astro_policy) else {
        return;
    };

    let violations = (spec.violations_for_policy)(policy);
    if violations.is_empty() {
        results.push(crate::support::info(
            spec.id,
            spec.info_title,
            spec.info_message.replacen("{}", &policy.rel_path, 1),
            &policy.rel_path,
        ));
        return;
    }

    results.push(crate::support::error(
        spec.id,
        spec.error_title,
        spec.error_message
            .replacen("{}", rel_path, 1)
            .replacen("{}", &violations.join(", "), 1),
        Some(rel_path),
    ));
}

/// Internal function `mdx_helper_surface_violations`.
fn mdx_helper_surface_violations(policy: &G3TsAstroMdxPolicySnapshot) -> Vec<String> {
    let mut violations = Vec::new();
    collect_helper_surface_violations(
        "[ts.astro.mdx].component_maps",
        &policy.mdx_component_maps,
        &mut violations,
    );
    violations
}

/// Internal function `collect_helper_surface_violations`.
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

/// Internal function `is_app_relative_dir`.
fn is_app_relative_dir(value: &str) -> bool {
    !value.is_empty()
        && !value.contains('\\')
        && !Path::new(value).is_absolute()
        && !has_parent_component(value)
        && !contains_glob_metachar(value)
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
