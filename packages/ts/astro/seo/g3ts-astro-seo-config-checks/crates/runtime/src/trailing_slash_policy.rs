use g3ts_astro_seo_types::{
    G3TsAstroConfigSurfaceState, G3TsAstroSeoIntegrationContractInput, G3TsAstroTrailingSlashPolicy,
};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "g3ts-astro-seo/trailing-slash-policy";

pub(crate) fn check(
    contract: &G3TsAstroSeoIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    match &contract.astro_config {
        G3TsAstroConfigSurfaceState::Parsed { snapshot }
            if snapshot.trailing_slash == Some(G3TsAstroTrailingSlashPolicy::Always) =>
        {
            results.push(crate::support::info(
                ID,
                "Astro config uses canonical trailing slashes",
                format!("`{}` sets `trailingSlash: \"always\"`.", snapshot.rel_path),
                &snapshot.rel_path,
            ));
        }
        G3TsAstroConfigSurfaceState::Parsed { snapshot } => {
            results.push(crate::support::error(
                ID,
                "Astro config must use canonical trailing slashes",
                format!(
                    "`{}` must set `trailingSlash: \"always\"`. Missing, `ignore`, and `never` are rejected by the default SEO policy.",
                    snapshot.rel_path
                ),
                Some(&snapshot.rel_path),
            ));
        }
        G3TsAstroConfigSurfaceState::Missing { rel_path } => {
            push_unavailable(rel_path, "is missing", results);
        }
        G3TsAstroConfigSurfaceState::Unreadable { rel_path, reason }
        | G3TsAstroConfigSurfaceState::ParseError { rel_path, reason } => {
            push_unavailable(rel_path, reason, results);
        }
    }
}

fn push_unavailable(rel_path: &str, reason: &str, results: &mut Vec<G3CheckResult>) {
    results.push(crate::support::error(
        ID,
        "Astro config must use canonical trailing slashes",
        format!("`{rel_path}` {reason}, so G3TS cannot prove the trailing slash policy."),
        Some(rel_path),
    ));
}
