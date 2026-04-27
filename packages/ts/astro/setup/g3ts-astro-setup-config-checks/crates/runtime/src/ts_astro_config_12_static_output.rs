use g3ts_astro_setup_types::{G3TsAstroConfigSurfaceState, G3TsAstroSetupIntegrationContractInput};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-SETUP-CONFIG-12";

pub(crate) fn check(
    contract: &G3TsAstroSetupIntegrationContractInput,
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
                    "Astro public content app must use explicit static output",
                    format!(
                        "`{}` must set `output: \"static\"`. Missing output is rejected even though Astro defaults to static, because agents need an explicit render contract and Nuasite must validate emitted static HTML.",
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
        "Astro public content app must use explicit static output",
        format!(
            "`{rel_path}` {reason}, so the Astro family cannot prove that this public content app emits static HTML for rendered-output validation."
        ),
        Some(rel_path),
    ));
}
