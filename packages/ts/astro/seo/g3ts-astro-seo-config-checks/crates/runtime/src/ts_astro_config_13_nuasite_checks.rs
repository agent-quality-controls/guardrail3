use g3ts_astro_types::{G3TsAstroConfigChecksInput, G3TsAstroConfigSurfaceState};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-SEO-CONFIG-13";
const DEPENDENCY_NAME: &str = "@nuasite/checks";

pub(crate) fn check(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.integration_contracts {
        let rel_path = g3ts_astro_check_support::core::astro_config_rel_path(contract);
        let has_package =
            g3ts_astro_check_support::core::package_has_dependency(contract, DEPENDENCY_NAME);
        let has_build_script = g3ts_astro_check_support::core::package_safely_runs_astro_build(contract);
        let has_static_output = g3ts_astro_check_support::core::astro_config_is_static(contract);
        let has_checks = match &contract.astro_config {
            G3TsAstroConfigSurfaceState::Parsed { snapshot } => {
                g3ts_astro_check_support::core::astro_config_has_nuasite_checks_with_required_options(
                    snapshot,
                )
            }
            G3TsAstroConfigSurfaceState::Missing { .. }
            | G3TsAstroConfigSurfaceState::Unreadable { .. }
            | G3TsAstroConfigSurfaceState::ParseError { .. } => false,
        };

        if has_package && has_build_script && has_static_output && has_checks {
            if let Some(rel_path) = rel_path {
                results.push(g3ts_astro_check_support::core::info(
                    ID,
                    "Nuasite rendered-output checks are installed and wired",
                    format!("`{rel_path}` wires `checks()` from `@nuasite/checks` with fail-closed options and the package scripts safely run `astro build`."),
                    rel_path,
                ));
            }
            continue;
        }

        results.push(g3ts_astro_check_support::core::error(
            ID,
            "Nuasite rendered-output checks are not installed and wired",
            format!(
                "This Astro app must list `{DEPENDENCY_NAME}`, safely run `astro build`, set `output: \"static\"`, and wire `checks()` from `{DEPENDENCY_NAME}` with `mode: \"full\"`, `failOnError: true`, `failOnWarning: true`, `reportJson: true`, `ai: false`, no disabled validator lanes, and `customChecks: [structuredDataPresentCheck]`. Missing pieces: {}.",
                missing_parts(has_package, has_build_script, has_static_output, has_checks).join(", ")
            ),
            rel_path,
        ));
    }
}

fn missing_parts(
    has_package: bool,
    has_build_script: bool,
    has_static_output: bool,
    has_checks: bool,
) -> Vec<&'static str> {
    let mut parts = Vec::new();
    if !has_package {
        parts.push("package dependency");
    }
    if !has_build_script {
        parts.push("safe `astro build` script");
    }
    if !has_static_output {
        parts.push("explicit static output");
    }
    if !has_checks {
        parts.push("fail-closed `checks()` integration");
    }
    parts
}
