use g3ts_astro_setup_types::{G3TsAstroConfigSurfaceState, G3TsAstroSetupIntegrationContractInput};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-SETUP-CONFIG-11";

pub(crate) fn check(
    contract: &G3TsAstroSetupIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    match &contract.astro_config {
        G3TsAstroConfigSurfaceState::Parsed { snapshot }
            if crate::support::astro_config_site_is_https(snapshot) =>
        {
            results.push(crate::support::info(
                ID,
                "Astro config has HTTPS site URL",
                format!("`{}` sets an absolute HTTPS `site` URL.", snapshot.rel_path),
                &snapshot.rel_path,
            ));
        }
        G3TsAstroConfigSurfaceState::Parsed { snapshot } => {
            results.push(crate::support::error(
                    ID,
                    "Astro config is missing an absolute HTTPS `site` URL",
                    format!(
                        "`{}` must set `site` to an absolute `https://` URL. Sitemap, robots, canonical URLs, and rendered SEO checks depend on this Astro config value.",
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
        "Astro config is missing an absolute HTTPS `site` URL",
        format!(
            "`{rel_path}` {reason}, so the Astro family cannot prove that canonical URL, sitemap, robots, and rendered SEO generation use an absolute HTTPS site URL."
        ),
        Some(rel_path),
    ));
}
