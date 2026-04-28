use g3ts_astro_seo_types::{G3TsAstroConfigSurfaceState, G3TsAstroSeoIntegrationContractInput};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "g3ts-astro-seo/broad-crawler-generator-absent";
const PACKAGE_NAME: &str = "@agentmarkup/astro";

pub(crate) fn check(
    contract: &G3TsAstroSeoIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::package_rel_path(&contract.package);
    let has_dependency =
        crate::support::package_mentions_dependency(&contract.package, PACKAGE_NAME);
    let has_integration = match &contract.astro_config {
        G3TsAstroConfigSurfaceState::Parsed { snapshot } => {
            crate::support::astro_config_has_integration(snapshot, PACKAGE_NAME)
        }
        G3TsAstroConfigSurfaceState::Missing { .. }
        | G3TsAstroConfigSurfaceState::Unreadable { .. }
        | G3TsAstroConfigSurfaceState::ParseError { .. } => false,
    };

    if !has_dependency && !has_integration {
        if let Some(rel_path) = rel_path {
            results.push(crate::support::info(
                ID,
                "Broad crawler generator is absent",
                format!("`{rel_path}` does not list `{PACKAGE_NAME}`."),
                rel_path,
            ));
        }
        return;
    }

    results.push(crate::support::error(
        ID,
        "Broad crawler generator is not allowed by default",
        format!("Use narrow sitemap, robots, and llms packages instead of `{PACKAGE_NAME}`."),
        rel_path,
    ));
}
