use g3ts_astro_setup_types::{G3TsAstroConfigSurfaceState, G3TsAstroSetupIntegrationContractInput};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "g3ts-astro-setup/required-integrations";
const REQUIRED_PACKAGES: [&str; 9] = [
    "@astrojs/react",
    "@astrojs/mdx",
    "@astrojs/sitemap",
    "g3ts-astro-sitemap-auditor",
    "astro-robots",
    "g3ts-astro-robots-auditor",
    "@nuasite/checks",
    "g3ts-astro-llms-generator",
    "g3ts-astro-llms-auditor",
];

pub(crate) fn check(
    contract: &G3TsAstroSetupIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::astro_config_rel_path(&contract.astro_config);
    let missing_packages = REQUIRED_PACKAGES
        .into_iter()
        .filter(|dependency| !crate::support::package_has_dependency(&contract.package, dependency))
        .collect::<Vec<_>>();
    let missing_integrations = match &contract.astro_config {
        G3TsAstroConfigSurfaceState::Parsed { snapshot } => required_integrations()
            .into_iter()
            .filter(|integration| !match integration.argument {
                RequiredIntegrationArgument::None => {
                    crate::support::astro_config_has_zero_arg_integration(
                        snapshot,
                        integration.module,
                        integration.accepted_imports,
                    )
                }
                RequiredIntegrationArgument::Some => {
                    crate::support::astro_config_has_object_arg_integration(
                        snapshot,
                        integration.module,
                        integration.accepted_imports,
                    )
                }
            })
            .map(|integration| integration.module)
            .collect::<Vec<_>>(),
        G3TsAstroConfigSurfaceState::Missing { .. }
        | G3TsAstroConfigSurfaceState::Unreadable { .. }
        | G3TsAstroConfigSurfaceState::ParseError { .. } => required_integrations()
            .into_iter()
            .map(|integration| integration.module)
            .collect(),
    };

    if missing_packages.is_empty() && missing_integrations.is_empty() {
        if let Some(rel_path) = rel_path {
            results.push(crate::support::info(
                ID,
                "Required Astro integrations are present",
                format!("`{rel_path}` wires React, MDX, sitemap generator/auditor, robots generator/auditor, Nuasite checks, and llms generator/auditor integrations from the approved packages."),
                rel_path,
            ));
        }
        return;
    }

    results.push(crate::support::error(
            ID,
            "Required Astro integrations are missing",
            format!(
                "This Astro app must list and wire the approved integration packages. Missing packages: {}. Missing integrations: {}. Wrappers, wrong source modules, and dynamic spreads do not satisfy this contract.",
                format_missing(&missing_packages),
                format_missing(&missing_integrations)
            ),
            rel_path,
        ));
}

struct RequiredIntegration {
    module: &'static str,
    accepted_imports: &'static [Option<&'static str>],
    argument: RequiredIntegrationArgument,
}

enum RequiredIntegrationArgument {
    None,
    Some,
}

fn required_integrations() -> Vec<RequiredIntegration> {
    vec![
        RequiredIntegration {
            module: "@astrojs/react",
            accepted_imports: &[None],
            argument: RequiredIntegrationArgument::None,
        },
        RequiredIntegration {
            module: "@astrojs/mdx",
            accepted_imports: &[None],
            argument: RequiredIntegrationArgument::None,
        },
        RequiredIntegration {
            module: "@astrojs/sitemap",
            accepted_imports: &[None],
            argument: RequiredIntegrationArgument::None,
        },
        RequiredIntegration {
            module: "g3ts-astro-sitemap-auditor",
            accepted_imports: &[None],
            argument: RequiredIntegrationArgument::Some,
        },
        RequiredIntegration {
            module: "astro-robots",
            accepted_imports: &[None],
            argument: RequiredIntegrationArgument::None,
        },
        RequiredIntegration {
            module: "g3ts-astro-robots-auditor",
            accepted_imports: &[None],
            argument: RequiredIntegrationArgument::Some,
        },
        RequiredIntegration {
            module: "@nuasite/checks",
            accepted_imports: &[None, Some("checks")],
            argument: RequiredIntegrationArgument::Some,
        },
        RequiredIntegration {
            module: "g3ts-astro-llms-generator",
            accepted_imports: &[None],
            argument: RequiredIntegrationArgument::Some,
        },
        RequiredIntegration {
            module: "g3ts-astro-llms-auditor",
            accepted_imports: &[None],
            argument: RequiredIntegrationArgument::Some,
        },
    ]
}

fn format_missing(values: &[&str]) -> String {
    if values.is_empty() {
        return "none".to_owned();
    }

    values
        .iter()
        .map(|value| format!("`{value}`"))
        .collect::<Vec<_>>()
        .join(", ")
}
