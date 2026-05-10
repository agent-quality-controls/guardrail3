use g3ts_astro_seo_types::{G3TsAstroConfigSurfaceState, G3TsAstroSeoIntegrationContractInput};
use guardrail3_check_types::G3CheckResult;

/// Static rule data.
const ID: &str = "g3ts-astro-seo/static-output-config";

/// Validates the rule and pushes findings into `results`.
/// Internal helper exported within the runtime crate.
pub(crate) fn check(
    contract: &G3TsAstroSeoIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    match &contract.astro_config {
        G3TsAstroConfigSurfaceState::Parsed { snapshot }
            if crate::support::astro_config_is_static(&contract.astro_config) =>
        {
            results.push(crate::support::info(
                ID,
                "Astro config uses static output",
                format!("`{}` sets `output: \"static\"`.", snapshot.rel_path),
                &snapshot.rel_path,
            ));
        }
        G3TsAstroConfigSurfaceState::Parsed { snapshot } => {
            results.push(crate::support::error(
                ID,
                "Astro SEO apps must use explicit static output",
                format!("`{}` must set `output: \"static\"`.", snapshot.rel_path),
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

/// Internal helper used by the rule.
fn push_unavailable(rel_path: &str, reason: &str, results: &mut Vec<G3CheckResult>) {
    results.push(crate::support::error(
        ID,
        "Astro SEO apps must use explicit static output",
        format!("`{rel_path}` {reason}, so G3TS cannot prove static SEO artifacts."),
        Some(rel_path),
    ));
}
