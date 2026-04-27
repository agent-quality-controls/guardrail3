use g3ts_astro_types::{G3TsAstroConfigChecksInput, G3TsAstroConfigSurfaceState};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-SEO-CONFIG-14";
const DEPENDENCY_NAME: &str = "@astrojs/sitemap";

pub(crate) fn check(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.integration_contracts {
        let rel_path = g3ts_astro_check_support::core::astro_config_rel_path(contract);
        let has_package =
            g3ts_astro_check_support::core::package_has_dependency(contract, DEPENDENCY_NAME);
        let has_wiring = match &contract.astro_config {
            G3TsAstroConfigSurfaceState::Parsed { snapshot } => {
                g3ts_astro_check_support::core::astro_config_site_is_https(snapshot)
                    && g3ts_astro_check_support::core::astro_config_has_zero_arg_integration(
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
                results.push(g3ts_astro_check_support::core::info(
                    ID,
                    "Astro sitemap integration is installed and wired",
                    format!("`{rel_path}` wires default `sitemap()` from `{DEPENDENCY_NAME}` and has an HTTPS `site`."),
                    rel_path,
                ));
            }
            continue;
        }

        results.push(g3ts_astro_check_support::core::error(
            ID,
            "Astro sitemap integration is not installed and wired",
            format!(
                "This Astro app must list `{DEPENDENCY_NAME}`, set an absolute HTTPS `site`, and include `sitemap()` imported as the default export from `{DEPENDENCY_NAME}` in `integrations`. Missing or wrong pieces: {}.",
                missing_parts(has_package, has_wiring).join(", ")
            ),
            rel_path,
        ));
    }
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
