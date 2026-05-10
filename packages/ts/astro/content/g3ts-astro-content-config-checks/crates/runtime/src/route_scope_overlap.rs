use g3ts_astro_content_types::G3TsAstroContentIntegrationContractInput;
use globset::{Glob, GlobSet, GlobSetBuilder};
use guardrail3_check_types::G3CheckResult;

/// Internal constant `ID`.
const ID: &str = "g3ts-astro-content/route-scope-overlap";

/// Internal function `check`.
pub(crate) fn check(
    contract: &G3TsAstroContentIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::content_policy_rel_path(&contract.astro_policy);
    let Some(policy) = crate::support::parsed_content_policy(&contract.astro_policy) else {
        return;
    };

    let Ok(content_routes) = glob_set(&policy.content_routes) else {
        results.push(invalid_glob_error(rel_path));
        return;
    };
    let Ok(non_content_routes) = glob_set(&policy.non_content_routes) else {
        results.push(invalid_glob_error(rel_path));
        return;
    };

    let overlapping_routes = contract
        .route_page_paths
        .iter()
        .filter(|path| content_routes.is_match(path) && non_content_routes.is_match(path))
        .cloned()
        .collect::<Vec<_>>();

    if overlapping_routes.is_empty() {
        results.push(crate::support::info(
                ID,
                "Astro content and non-content route scopes are disjoint",
                format!(
                    "`{}` classifies discovered route pages without overlap between `[ts.astro.routes].content` and `[ts.astro.routes].non_content`.",
                    policy.rel_path
                ),
                &policy.rel_path,
            ));
        return;
    }

    results.push(crate::support::error(
            ID,
            "Astro content and non-content route scopes overlap",
            format!(
                "`{rel_path}` matches these discovered route pages as both content and non-content: {paths}. Adjust `[ts.astro.routes].content` or `[ts.astro.routes].non_content` so each route has exactly one policy role.",
                paths = format_paths(&overlapping_routes)
            ),
            Some(rel_path),
        ));
}

/// Internal function `glob_set`.
fn glob_set(patterns: &[String]) -> Result<GlobSet, globset::Error> {
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
        let _ = builder.add(Glob::new(pattern)?);
    }
    builder.build()
}

/// Internal function `invalid_glob_error`.
fn invalid_glob_error(rel_path: &str) -> G3CheckResult {
    crate::support::error(
        ID,
        "Astro route scope policy contains an invalid glob",
        format!(
            "`{rel_path}` contains a `[ts.astro.routes].content` or `[ts.astro.routes].non_content` entry that `globset` cannot compile. Use valid app-relative glob syntax."
        ),
        Some(rel_path),
    )
}

/// Internal function `format_paths`.
fn format_paths(paths: &[String]) -> String {
    paths
        .iter()
        .map(|path| format!("`{path}`"))
        .collect::<Vec<_>>()
        .join(", ")
}
