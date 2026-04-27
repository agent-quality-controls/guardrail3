use g3ts_astro_seo_types::{G3TsAstroConfigSurfaceState, G3TsAstroSeoIntegrationContractInput};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-SEO-CONFIG-13";
const DEPENDENCY_NAME: &str = "@nuasite/checks";

pub(crate) fn check(
    contract: &G3TsAstroSeoIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::astro_config_rel_path(&contract.astro_config);
    let has_package = crate::support::package_has_dependency(&contract.package, DEPENDENCY_NAME);
    let has_build_script = crate::support::package_safely_runs_astro_build(&contract.package);
    let has_static_output = crate::support::astro_config_is_static(&contract.astro_config);
    let has_checks = match &contract.astro_config {
        G3TsAstroConfigSurfaceState::Parsed { snapshot } => {
            crate::nuasite_options::astro_config_has_nuasite_checks_with_required_options(snapshot)
        }
        G3TsAstroConfigSurfaceState::Missing { .. }
        | G3TsAstroConfigSurfaceState::Unreadable { .. }
        | G3TsAstroConfigSurfaceState::ParseError { .. } => false,
    };

    if has_package && has_build_script && has_static_output && has_checks {
        if let Some(rel_path) = rel_path {
            results.push(crate::support::info(
                    ID,
                    "Nuasite rendered-output checks are installed and wired",
                    format!("`{rel_path}` wires `checks()` from `@nuasite/checks` with fail-closed options and the package scripts safely run `astro build`."),
                    rel_path,
                ));
        }
        return;
    }

    results.push(crate::support::error(
            ID,
            "Nuasite rendered-output checks are not installed and wired",
            format!(
                "This Astro app must list `{DEPENDENCY_NAME}`, safely run `astro build`, set `output: \"static\"`, and wire `checks()` from `{DEPENDENCY_NAME}` with `mode: \"full\"`, `failOnError: true`, `failOnWarning: true`, `reportJson: true`, `ai: false`, no disabled validator lanes, and `customChecks: [structuredDataPresentCheck]`. Missing pieces: {}.",
                missing_parts(has_package, has_build_script, has_static_output, has_checks).join(", ")
            ),
            rel_path,
        ));
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
