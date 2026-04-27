use g3ts_astro_types::G3TsAstroConfigChecksInput;
use globset::{Glob, GlobSet, GlobSetBuilder};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-CONTENT-CONFIG-25";

pub(crate) fn check(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.integration_contracts {
        let rel_path = g3ts_astro_check_support::core::astro_policy_rel_path(contract);
        let Some(policy) = g3ts_astro_check_support::core::parsed_astro_policy(contract) else {
            continue;
        };

        let Ok(content_routes) = glob_set(&policy.content_routes) else {
            results.push(invalid_glob_error(rel_path));
            continue;
        };
        let Ok(non_content_routes) = glob_set(&policy.non_content_routes) else {
            results.push(invalid_glob_error(rel_path));
            continue;
        };

        let overlapping_routes = contract
            .route_page_paths
            .iter()
            .filter(|path| content_routes.is_match(path) && non_content_routes.is_match(path))
            .cloned()
            .collect::<Vec<_>>();

        if overlapping_routes.is_empty() {
            results.push(g3ts_astro_check_support::core::info(
                ID,
                "Astro content and non-content route scopes are disjoint",
                format!(
                    "`{}` classifies discovered route pages without overlap between `[ts.astro.routes].content` and `[ts.astro.routes].non_content`.",
                    policy.rel_path
                ),
                &policy.rel_path,
            ));
            continue;
        }

        results.push(g3ts_astro_check_support::core::error(
            ID,
            "Astro content and non-content route scopes overlap",
            format!(
                "`{}` matches these discovered route pages as both content and non-content: {}. Adjust `[ts.astro.routes].content` or `[ts.astro.routes].non_content` so each route has exactly one policy role.",
                rel_path.unwrap_or("guardrail3-ts.toml"),
                format_paths(&overlapping_routes)
            ),
            rel_path,
        ));
    }
}

fn glob_set(patterns: &[String]) -> Result<GlobSet, globset::Error> {
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
        let _ = builder.add(Glob::new(pattern)?);
    }
    builder.build()
}

fn invalid_glob_error(rel_path: Option<&str>) -> G3CheckResult {
    g3ts_astro_check_support::core::error(
        ID,
        "Astro route scope policy contains an invalid glob",
        format!(
            "`{}` contains a `[ts.astro.routes].content` or `[ts.astro.routes].non_content` entry that `globset` cannot compile. Use valid app-relative glob syntax.",
            rel_path.unwrap_or("guardrail3-ts.toml")
        ),
        rel_path,
    )
}

fn format_paths(paths: &[String]) -> String {
    paths
        .iter()
        .map(|path| format!("`{path}`"))
        .collect::<Vec<_>>()
        .join(", ")
}
