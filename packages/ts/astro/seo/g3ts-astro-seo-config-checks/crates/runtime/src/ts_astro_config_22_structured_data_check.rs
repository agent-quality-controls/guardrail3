use g3ts_astro_types::{G3TsAstroSeoIntegrationContractInput, G3TsAstroConfigSurfaceState};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-SEO-CONFIG-22";
const DEPENDENCY_NAME: &str = "g3ts-astro-nuasite-checks";

pub(crate) fn check(contracts: &[G3TsAstroSeoIntegrationContractInput], results: &mut Vec<G3CheckResult>) {
    for contract in contracts {
        let rel_path = g3ts_astro_check_support::core::astro_config_rel_path(contract);
        let has_package =
            g3ts_astro_check_support::core::package_has_dependency(contract, DEPENDENCY_NAME);
        let has_wiring = match &contract.astro_config {
            G3TsAstroConfigSurfaceState::Parsed { snapshot } => {
                g3ts_astro_check_support::core::checks_options_include_structured_data_check(snapshot)
            }
            G3TsAstroConfigSurfaceState::Missing { .. }
            | G3TsAstroConfigSurfaceState::Unreadable { .. }
            | G3TsAstroConfigSurfaceState::ParseError { .. } => false,
        };

        if has_package && has_wiring {
            if let Some(rel_path) = rel_path {
                results.push(g3ts_astro_check_support::core::info(
                    ID,
                    "JSON-LD presence check is delegated to Nuasite",
                    format!("`{rel_path}` wires `structuredDataPresentCheck` imported from `{DEPENDENCY_NAME}` through `checks({{ customChecks: [...] }})`."),
                    rel_path,
                ));
            }
            continue;
        }

        results.push(g3ts_astro_check_support::core::error(
            ID,
            "JSON-LD presence check is not delegated to Nuasite",
            format!(
                "This Astro app must list `{DEPENDENCY_NAME}` and pass `structuredDataPresentCheck` imported from `{DEPENDENCY_NAME}` in `checks({{ customChecks: [structuredDataPresentCheck] }})`. Inline app-local custom checks do not satisfy this contract because the validator implementation must be shared.",
            ),
            rel_path,
        ));
    }
}
