use g3ts_astro_seo_types::{G3TsAstroSeoIntegrationContractInput, G3TsAstroSeoPolicySnapshot};
use guardrail3_check_types::G3CheckResult;
use std::path::{Component, Path};

/// Static rule data.
const SEO_ID: &str = "g3ts-astro-seo/policy-helper-surfaces";

/// Function pointer that returns helper-surface violations for a policy snapshot.
type ViolationsFn = fn(&G3TsAstroSeoPolicySnapshot) -> Vec<String>;

/// Static-text inputs to a helper-surface check.
struct HelperSurfaceCheckSpec {
    /// Stable rule identifier surfaced in findings.
    id: &'static str,
    /// Title used when the check passes.
    info_title: &'static str,
    /// Title used when the check fails.
    error_title: &'static str,
    /// Message template for inventory findings.
    info_message: &'static str,
    /// Message template for error findings.
    error_message: &'static str,
    /// Computes violations for the policy snapshot.
    violations_for_policy: ViolationsFn,
}

/// Internal helper exported within the runtime crate.
pub(crate) fn check_seo(
    contract: &G3TsAstroSeoIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    check_helper_surfaces(
        contract,
        &HelperSurfaceCheckSpec {
            id: SEO_ID,
            info_title: "Astro strict content policy declares approved SEO helper surfaces",
            error_title: "Astro strict content policy is missing approved SEO helper surfaces",
            info_message: "`{}` declares non-empty app-relative `[ts.astro.seo].metadata_helpers` and `[ts.astro.seo].json_ld_helpers`.",
            error_message: "`{}` must declare non-empty app-relative `[ts.astro.seo].metadata_helpers` and `[ts.astro.seo].json_ld_helpers`. Violations: {}. These are approved module surfaces, not hardcoded required filenames.",
            violations_for_policy: seo_helper_surface_violations,
        },
        results,
    );
}

/// Internal helper used by the rule.
fn check_helper_surfaces(
    contract: &G3TsAstroSeoIntegrationContractInput,
    spec: &HelperSurfaceCheckSpec,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::seo_policy_rel_path(&contract.astro_policy);
    let Some(policy) = crate::support::parsed_seo_policy(&contract.astro_policy) else {
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

/// Internal helper used by the rule.
fn seo_helper_surface_violations(policy: &G3TsAstroSeoPolicySnapshot) -> Vec<String> {
    let mut violations = Vec::new();
    collect_helper_surface_violations(
        "[ts.astro.seo].metadata_helpers",
        &policy.metadata_helpers,
        &mut violations,
    );
    collect_helper_surface_violations(
        "[ts.astro.seo].json_ld_helpers",
        &policy.json_ld_helpers,
        &mut violations,
    );
    violations
}

/// Internal helper used by the rule.
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

/// Internal helper used by the rule.
fn is_app_relative_dir(value: &str) -> bool {
    !value.is_empty()
        && !value.contains('\\')
        && !Path::new(value).is_absolute()
        && !has_parent_component(value)
        && !contains_glob_metachar(value)
}

/// Internal helper used by the rule.
fn has_parent_component(value: &str) -> bool {
    Path::new(value)
        .components()
        .any(|component| component == Component::ParentDir)
}

/// Internal helper used by the rule.
fn contains_glob_metachar(value: &str) -> bool {
    value
        .chars()
        .any(|character| matches!(character, '*' | '?' | '[' | ']' | '{' | '}'))
}
