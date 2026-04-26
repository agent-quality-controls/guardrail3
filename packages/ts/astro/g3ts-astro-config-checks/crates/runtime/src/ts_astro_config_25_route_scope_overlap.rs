use g3ts_astro_types::G3TsAstroConfigChecksInput;
use globset::{Glob, GlobSet, GlobSetBuilder};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-CONFIG-25";

pub(crate) fn check(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.integration_contracts {
        let rel_path = crate::support::astro_policy_rel_path(contract);
        let Some(policy) = crate::support::parsed_astro_policy(contract) else {
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
            results.push(crate::support::info(
                ID,
                "Astro content and non-content route scopes are disjoint",
                format!(
                    "`{}` classifies discovered route pages without overlap between `content_routes` and `non_content_routes`.",
                    policy.rel_path
                ),
                &policy.rel_path,
            ));
            continue;
        }

        results.push(crate::support::error(
            ID,
            "Astro content and non-content route scopes overlap",
            format!(
                "`{}` matches these discovered route pages as both content and non-content: {}. Adjust `[ts.astro].content_routes` or `[ts.astro].non_content_routes` so each route has exactly one policy role.",
                rel_path.unwrap_or("guardrail3-rs.toml"),
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
    crate::support::error(
        ID,
        "Astro route scope policy contains an invalid glob",
        format!(
            "`{}` contains a `content_routes` or `non_content_routes` entry that `globset` cannot compile. Use valid app-relative glob syntax.",
            rel_path.unwrap_or("guardrail3-rs.toml")
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
