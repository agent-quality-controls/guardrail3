use g3ts_astro_seo_types::{G3TsAstroConfigSurfaceState, G3TsAstroSeoIntegrationContractInput};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "g3ts-astro-seo/robots-integration-present";
const DEPENDENCY_NAME: &str = "astro-robots";

pub(crate) fn check(
    contract: &G3TsAstroSeoIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::astro_config_rel_path(&contract.astro_config);
    let has_wiring = match &contract.astro_config {
        G3TsAstroConfigSurfaceState::Parsed { snapshot } => {
            crate::support::astro_config_has_integration(snapshot, DEPENDENCY_NAME)
        }
        G3TsAstroConfigSurfaceState::Missing { .. }
        | G3TsAstroConfigSurfaceState::Unreadable { .. }
        | G3TsAstroConfigSurfaceState::ParseError { .. } => false,
    };

    if has_wiring {
        if let Some(rel_path) = rel_path {
            results.push(crate::support::info(
                ID,
                "Astro robots integration is wired",
                format!("`{rel_path}` wires an integration imported from `{DEPENDENCY_NAME}`."),
                rel_path,
            ));
        }
        return;
    }

    results.push(crate::support::error(
            ID,
            "Astro robots integration is not wired",
            format!(
                "This Astro app must include an integration imported from `{DEPENDENCY_NAME}` in `astro.config`. Hand-authored `public/robots.txt` does not satisfy the default Astro contract."
            ),
            rel_path,
        ));
}
