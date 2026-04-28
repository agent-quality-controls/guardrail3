use g3ts_astro_seo_types::{G3TsAstroConfigSurfaceState, G3TsAstroSeoIntegrationContractInput};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "g3ts-astro-seo/canonical-site-config";

pub(crate) fn check(
    contract: &G3TsAstroSeoIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    match &contract.astro_config {
        G3TsAstroConfigSurfaceState::Parsed { snapshot }
            if crate::support::astro_config_site_is_https(snapshot) =>
        {
            results.push(crate::support::info(
                ID,
                "Astro config has canonical HTTPS site URL",
                format!("`{}` sets an absolute HTTPS `site` URL.", snapshot.rel_path),
                &snapshot.rel_path,
            ));
        }
        G3TsAstroConfigSurfaceState::Parsed { snapshot } => {
            results.push(crate::support::error(
                ID,
                "Astro config is missing a canonical HTTPS site URL",
                format!(
                    "`{}` must set `site` to one canonical absolute `https://` URL.",
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
        "Astro config is missing a canonical HTTPS site URL",
        format!("`{rel_path}` {reason}, so G3TS cannot prove the canonical site URL."),
        Some(rel_path),
    ));
}
