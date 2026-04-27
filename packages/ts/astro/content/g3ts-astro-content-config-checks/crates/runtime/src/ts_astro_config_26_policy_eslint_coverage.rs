use g3ts_astro_content_types::{
    G3TsAstroContentEslintSurfaceState, G3TsAstroContentPolicyEslintContractInput,
    G3TsAstroContentPolicySnapshot, G3TsAstroPipelineRuleScopeSnapshot,
};
use globset::{Glob, GlobSet, GlobSetBuilder};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-CONTENT-CONFIG-26";
const ROUTE_SCOPED_PIPELINE_RULES: [&str; 8] = [
    "astro-pipeline/no-authored-content-fs-read",
    "astro-pipeline/no-authored-content-glob",
    "astro-pipeline/no-authored-content-imports",
    "astro-pipeline/no-content-data-modules-in-routes",
    "astro-pipeline/no-direct-astro-content-in-routes",
    "astro-pipeline/require-approved-content-adapter-in-routes",
    "astro-pipeline/no-side-loader-imports",
    "astro-pipeline/no-velite-imports",
];

pub(crate) fn check(
    contract: &G3TsAstroContentPolicyEslintContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let policy_rel_path = crate::support::content_policy_rel_path(&contract.astro_policy);
    let Some(policy) = crate::support::parsed_content_policy(&contract.astro_policy) else {
        return;
    };
    let G3TsAstroContentEslintSurfaceState::Parsed { snapshot: eslint } = &contract.eslint_config
    else {
        results.push(error(
            policy_rel_path,
            "ESLint config is missing or not parsed",
        ));
        return;
    };

    let coverage = policy_coverage(contract, policy);
    let lanes = [
        (
            "Astro",
            &eslint.astro_source_route_scoped_pipeline_rule_scopes,
        ),
        ("TS", &eslint.ts_source_route_scoped_pipeline_rule_scopes),
        ("TSX", &eslint.tsx_source_route_scoped_pipeline_rule_scopes),
    ];
    let missing = lanes
        .into_iter()
        .flat_map(|(lane, scopes)| missing_lane_coverage(lane, scopes, &coverage))
        .collect::<Vec<_>>();

    if missing.is_empty() {
        results.push(crate::support::info(
            ID,
            "Astro ESLint route coverage matches strict content policy",
            format!(
                "`{}` and `{}` agree on content route, non-content route, and endpoint coverage for the required Astro pipeline rules.",
                policy.rel_path, eslint.rel_path
            ),
            &eslint.rel_path,
        ));
        return;
    }

    results.push(error(policy_rel_path, &missing.join("; ")));
}

struct PolicyCoverage {
    content_routes: Vec<String>,
    non_content_routes: Vec<String>,
    endpoints: Vec<String>,
}

fn policy_coverage(
    contract: &G3TsAstroContentPolicyEslintContractInput,
    policy: &G3TsAstroContentPolicySnapshot,
) -> PolicyCoverage {
    let content_globs = glob_set(&policy.content_routes).ok();
    let non_content_globs = glob_set(&policy.non_content_routes).ok();
    let endpoint_globs = glob_set(&policy.endpoints).ok();

    PolicyCoverage {
        content_routes: matching_paths(&contract.route_page_paths, content_globs.as_ref()),
        non_content_routes: matching_paths(&contract.route_page_paths, non_content_globs.as_ref()),
        endpoints: matching_paths(&contract.endpoint_paths, endpoint_globs.as_ref()),
    }
}

fn missing_lane_coverage(
    lane: &str,
    scopes: &[G3TsAstroPipelineRuleScopeSnapshot],
    coverage: &PolicyCoverage,
) -> Vec<String> {
    ROUTE_SCOPED_PIPELINE_RULES
        .iter()
        .filter_map(|rule_name| {
            let Some(scope) = scopes.iter().find(|scope| scope.rule_name == *rule_name) else {
                return Some(format!("{lane} lane missing `{rule_name}` scope"));
            };

            if !scope_covers_policy(scope, coverage) {
                return Some(format!(
                    "{lane} lane `{rule_name}` scope does not match policy"
                ));
            }

            None
        })
        .collect()
}

fn scope_covers_policy(
    scope: &G3TsAstroPipelineRuleScopeSnapshot,
    coverage: &PolicyCoverage,
) -> bool {
    let Ok(route_globs) = glob_set(&scope.route_globs) else {
        return false;
    };
    let Ok(endpoint_globs) = glob_set(&scope.endpoint_globs) else {
        return false;
    };

    all_match(&route_globs, &coverage.content_routes)
        && none_match(&route_globs, &coverage.non_content_routes)
        && all_match(&endpoint_globs, &coverage.endpoints)
}

fn matching_paths(paths: &[String], globs: Option<&GlobSet>) -> Vec<String> {
    let Some(globs) = globs else {
        return Vec::new();
    };

    paths
        .iter()
        .filter(|path| globs.is_match(path.as_str()))
        .cloned()
        .collect()
}

fn all_match(globs: &GlobSet, paths: &[String]) -> bool {
    paths.iter().all(|path| globs.is_match(path.as_str()))
}

fn none_match(globs: &GlobSet, paths: &[String]) -> bool {
    paths.iter().all(|path| !globs.is_match(path.as_str()))
}

fn glob_set(patterns: &[String]) -> Result<GlobSet, globset::Error> {
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
        let _ = builder.add(Glob::new(pattern)?);
    }
    builder.build()
}

fn error(policy_rel_path: Option<&str>, reason: &str) -> G3CheckResult {
    crate::support::error(
        ID,
        "Astro ESLint route coverage does not match strict content policy",
        format!(
            "`{}` must configure `g3ts-eslint-plugin-astro-pipeline` so Astro, TS, and TSX source lanes cover every discovered `[ts.astro.routes].content` page, exclude `[ts.astro.routes].non_content` pages from route-scoped content enforcement, and cover every discovered `[ts.astro.routes].endpoints` file. Mismatch: {reason}.",
            policy_rel_path.unwrap_or("guardrail3-ts.toml")
        ),
        policy_rel_path,
    )
}
