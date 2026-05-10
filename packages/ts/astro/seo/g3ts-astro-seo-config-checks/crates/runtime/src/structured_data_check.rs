use g3ts_astro_seo_types::{G3TsAstroConfigSurfaceState, G3TsAstroSeoIntegrationContractInput};
use guardrail3_check_types::G3CheckResult;

/// Static rule data.
const ID: &str = "g3ts-astro-seo/structured-data-check";
/// Static rule data.
const DEPENDENCY_NAME: &str = "g3ts-astro-nuasite-checks";

/// Validates the rule and pushes findings into `results`.
/// Internal helper exported within the runtime crate.
pub(crate) fn check(
    contract: &G3TsAstroSeoIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::astro_config_rel_path(&contract.astro_config);
    let has_package = crate::support::package_has_dependency(&contract.package, DEPENDENCY_NAME);
    let has_wiring = match &contract.astro_config {
        G3TsAstroConfigSurfaceState::Parsed { snapshot } => {
            crate::nuasite_options::checks_options_include_structured_data_check(snapshot)
        }
        G3TsAstroConfigSurfaceState::Missing { .. }
        | G3TsAstroConfigSurfaceState::Unreadable { .. }
        | G3TsAstroConfigSurfaceState::ParseError { .. } => false,
    };

    if has_package && has_wiring {
        results.push(crate::support::info(
                ID,
                "JSON-LD presence check is delegated to Nuasite",
                format!("`{rel_path}` wires `structuredDataPresentCheck` imported from `{DEPENDENCY_NAME}` through `checks({{ customChecks: [...] }})`."),
                rel_path,
            ));
        return;
    }

    results.push(crate::support::error(
            ID,
            "JSON-LD presence check is not delegated to Nuasite",
            format!(
                "This Astro app must list `{DEPENDENCY_NAME}` and pass `structuredDataPresentCheck` imported from `{DEPENDENCY_NAME}` in `checks({{ customChecks: [structuredDataPresentCheck] }})`. Inline app-local custom checks do not satisfy this contract because the validator implementation must be shared.",
            ),
            Some(rel_path),
        ));
}
