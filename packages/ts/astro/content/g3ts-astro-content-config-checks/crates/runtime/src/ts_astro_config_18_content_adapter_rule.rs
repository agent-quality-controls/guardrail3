use g3ts_astro_content_types::{
    G3TsAstroContentEslintSurfaceState, G3TsAstroContentPolicyEslintContractInput,
    G3TsAstroContentPolicySnapshot, G3TsAstroPipelineRuleScopeSnapshot,
};
use guardrail3_check_types::G3CheckResult;
use std::collections::BTreeSet;

const ID: &str = "TS-ASTRO-CONTENT-CONFIG-18";
const PLUGIN_NAME: &str = "astro-pipeline";
const PLUGIN_PACKAGE_NAME: &str = "g3ts-eslint-plugin-astro-pipeline";
const RULE_NAME: &str = "astro-pipeline/require-approved-content-adapter-in-routes";

pub(crate) fn check(
    contract: &G3TsAstroContentPolicyEslintContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = eslint_rel_path(&contract.eslint_config);
    let policy = crate::support::parsed_content_policy(&contract.astro_policy);
    if has_required_lanes(&contract.eslint_config)
        && policy.is_some_and(|policy| has_content_adapter_modules(&contract.eslint_config, policy))
    {
        if let Some(rel_path) = rel_path {
            results.push(crate::support::info(
                ID,
                "Astro content adapter route rule is effective",
                format!("`{rel_path}` enforces `{RULE_NAME}` from `{PLUGIN_PACKAGE_NAME}` with route coverage, endpoint coverage, and `approvedContentAdapterModules` matching `[ts.astro.content].adapters` as equivalent recursive file globs on Astro, TS, and TSX source probes."),
                rel_path,
            ));
        }
        return;
    }

    results.push(crate::support::error(
        ID,
        "Astro content adapter route rule is not effective",
        format!(
            "`{}` must import `{PLUGIN_PACKAGE_NAME}`, register it as `{PLUGIN_NAME}`, and activate rule `{RULE_NAME}` at `error` on Astro, TS, and TSX source probes with `routeGlobs`, `endpointGlobs`, and `approvedContentAdapterModules` covering `[ts.astro.content].adapters` as recursive file globs. Directory policy entries like `src/content` may be represented in ESLint as `src/content/**` or `src/content/**/*`. Public page routes must import an approved content adapter instead of reading content directly.",
            rel_path.unwrap_or("eslint.config.*")
        ),
        rel_path,
    ));
}

fn eslint_rel_path(config: &G3TsAstroContentEslintSurfaceState) -> Option<&str> {
    match config {
        G3TsAstroContentEslintSurfaceState::Missing { rel_path }
        | G3TsAstroContentEslintSurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroContentEslintSurfaceState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsAstroContentEslintSurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}

fn has_required_lanes(config: &G3TsAstroContentEslintSurfaceState) -> bool {
    let G3TsAstroContentEslintSurfaceState::Parsed { snapshot } = config else {
        return false;
    };

    lane_has_route_rule(
        snapshot.astro_source_probe_present,
        snapshot.astro_source_probe_ignored,
        &snapshot.astro_source_plugins,
        &snapshot.astro_source_error_rules,
        &snapshot.astro_source_route_scoped_pipeline_rule_scopes,
    ) && lane_has_route_rule(
        snapshot.ts_source_probe_present,
        snapshot.ts_source_probe_ignored,
        &snapshot.ts_source_plugins,
        &snapshot.ts_source_error_rules,
        &snapshot.ts_source_route_scoped_pipeline_rule_scopes,
    ) && lane_has_route_rule(
        snapshot.tsx_source_probe_present,
        snapshot.tsx_source_probe_ignored,
        &snapshot.tsx_source_plugins,
        &snapshot.tsx_source_error_rules,
        &snapshot.tsx_source_route_scoped_pipeline_rule_scopes,
    )
}

fn lane_has_route_rule(
    probe_present: bool,
    probe_ignored: bool,
    plugins: &[String],
    error_rules: &[String],
    route_scoped_rules: &[G3TsAstroPipelineRuleScopeSnapshot],
) -> bool {
    probe_present
        && !probe_ignored
        && plugins.iter().any(|plugin| plugin == PLUGIN_NAME)
        && error_rules.iter().any(|rule| rule == RULE_NAME)
        && route_scoped_rules.iter().any(|rule| {
            rule.rule_name == RULE_NAME
                && !rule.route_globs.is_empty()
                && !rule.endpoint_globs.is_empty()
        })
}

fn has_content_adapter_modules(
    config: &G3TsAstroContentEslintSurfaceState,
    policy: &G3TsAstroContentPolicySnapshot,
) -> bool {
    let expected_modules = expected_module_globs(&policy.content_adapters);
    let G3TsAstroContentEslintSurfaceState::Parsed { snapshot } = config else {
        return false;
    };

    !expected_modules.is_empty()
        && string_arrays_match_as_sets(
            &snapshot.astro_source_effective_content_adapter_modules,
            &expected_modules,
        )
        && string_arrays_match_as_sets(
            &snapshot.ts_source_effective_content_adapter_modules,
            &expected_modules,
        )
        && string_arrays_match_as_sets(
            &snapshot.tsx_source_effective_content_adapter_modules,
            &expected_modules,
        )
}

fn expected_module_globs(configured_paths: &[String]) -> Vec<String> {
    configured_paths
        .iter()
        .map(|configured_path| configured_path.trim())
        .filter(|configured_path| !configured_path.is_empty())
        .map(canonical_recursive_module_glob)
        .collect()
}

fn string_arrays_match_as_sets(left: &[String], right: &[String]) -> bool {
    let left: BTreeSet<String> = left
        .iter()
        .map(|value| canonical_recursive_module_glob(value))
        .collect();
    let right: BTreeSet<String> = right
        .iter()
        .map(|value| canonical_recursive_module_glob(value))
        .collect();
    left == right
}

fn canonical_recursive_module_glob(value: &str) -> String {
    let trimmed = value.trim().trim_end_matches('/');
    if let Some(prefix) = trimmed.strip_suffix("/**/*") {
        return format!("{prefix}/**");
    }
    if let Some(prefix) = trimmed.strip_suffix("/**") {
        return format!("{prefix}/**");
    }
    if trimmed.contains('*') {
        trimmed.to_owned()
    } else {
        format!("{trimmed}/**")
    }
}
