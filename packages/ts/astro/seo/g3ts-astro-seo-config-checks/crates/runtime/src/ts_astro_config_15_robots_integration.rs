use g3ts_astro_seo_types::{G3TsAstroConfigSurfaceState, G3TsAstroSeoIntegrationContractInput};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-SEO-CONFIG-15";
const DEPENDENCY_NAME: &str = "astro-robots";

pub(crate) fn check(
    contract: &G3TsAstroSeoIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::astro_config_rel_path(&contract.astro_config);
    let has_package = crate::support::package_has_dependency(&contract.package, DEPENDENCY_NAME);
    let has_wiring = match &contract.astro_config {
        G3TsAstroConfigSurfaceState::Parsed { snapshot } => {
            crate::support::astro_config_site_is_https(snapshot)
                && crate::support::astro_config_has_zero_arg_integration(
                    snapshot,
                    DEPENDENCY_NAME,
                    &[None],
                )
        }
        G3TsAstroConfigSurfaceState::Missing { .. }
        | G3TsAstroConfigSurfaceState::Unreadable { .. }
        | G3TsAstroConfigSurfaceState::ParseError { .. } => false,
    };

    if has_package && has_wiring {
        if let Some(rel_path) = rel_path {
            results.push(crate::support::info(
                    ID,
                    "Astro robots integration is installed and wired",
                    format!("`{rel_path}` wires default `robots()` from `{DEPENDENCY_NAME}` and has an HTTPS `site`."),
                    rel_path,
                ));
        }
        return;
    }

    results.push(crate::support::error(
            ID,
            "Astro robots integration is not installed and wired",
            format!(
                "This Astro app must list `{DEPENDENCY_NAME}`, set an absolute HTTPS `site`, and include `robots()` imported as the default export from `{DEPENDENCY_NAME}` in `integrations`. Hand-authored `public/robots.txt` does not satisfy the default Astro contract. Missing or wrong pieces: {}.",
                missing_parts(has_package, has_wiring).join(", ")
            ),
            rel_path,
        ));
}

fn missing_parts(has_package: bool, has_wiring: bool) -> Vec<&'static str> {
    let mut parts = Vec::new();
    if !has_package {
        parts.push("package dependency");
    }
    if !has_wiring {
        parts.push("Astro config integration or HTTPS site");
    }
    parts
}
