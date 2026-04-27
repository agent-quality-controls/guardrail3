use g3ts_astro_types::{
    G3TsAstroSetupIntegrationContractInput, G3TsAstroSyncpackConfigState,
};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-SETUP-CONFIG-09";

pub(crate) fn check(contracts: &[G3TsAstroSetupIntegrationContractInput], results: &mut Vec<G3CheckResult>) {
    for contract in contracts {
        match &contract.syncpack_config {
            G3TsAstroSyncpackConfigState::Parsed { snapshot } => {
                let package_path =
                    g3ts_astro_check_support::core::package_rel_path(contract).unwrap_or("package.json");
                if !snapshot.source_covers_package_manifest {
                    let expected_source = g3ts_astro_check_support::core::expected_syncpack_source_entry(
                        &snapshot.rel_path,
                        package_path,
                    )
                    .unwrap_or_else(|| package_path.to_owned());
                    results.push(g3ts_astro_check_support::core::error(
                        ID,
                        "Syncpack does not pin the required Astro stack",
                        format!(
                            "`{}` does not include exact `source` entry `{expected_source}` for `{package_path}`, so `syncpack lint` cannot prove package policy for this Astro app.",
                            snapshot.rel_path,
                        ),
                        Some(&snapshot.rel_path),
                    ));
                }

                if snapshot.missing_required_stack_pins.is_empty()
                    && snapshot.source_covers_package_manifest
                {
                    results.push(g3ts_astro_check_support::core::info(
                        ID,
                        "Syncpack pins the required Astro stack",
                        format!(
                            "`{}` pins the required Syncpack package policy: {}.",
                            snapshot.rel_path,
                            g3ts_astro_check_support::core::required_syncpack_pins_message(contract)
                        ),
                        &snapshot.rel_path,
                    ));
                    continue;
                }

                if !snapshot.missing_required_stack_pins.is_empty() {
                    results.push(g3ts_astro_check_support::core::error(
                        ID,
                        "Syncpack does not pin the required Astro stack",
                        format!(
                        "`{}` is missing required Syncpack pinned versionGroups: {}. Add exactly one canonical versionGroup per listed package, with exact `dependencies`, `dependencyTypes` containing exactly `prod` and `dev`, no `packages`, no `specifierTypes`, and the listed `pinVersion`.",
                        snapshot.rel_path,
                        snapshot
                            .missing_required_stack_pins
                            .iter()
                            .map(|pin| format!("`{}` -> `{}`", pin.dependency, pin.version))
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                        Some(&snapshot.rel_path),
                    ));
                }
            }
            G3TsAstroSyncpackConfigState::Missing { rel_path } => {
                push_unavailable_error(contract, rel_path, "is missing", results);
            }
            G3TsAstroSyncpackConfigState::Unreadable { rel_path, reason }
            | G3TsAstroSyncpackConfigState::ParseError { rel_path, reason } => {
                push_unavailable_error(contract, rel_path, reason, results);
            }
        }
    }
}

fn push_unavailable_error(
    contract: &G3TsAstroSetupIntegrationContractInput,
    rel_path: &str,
    reason: &str,
    results: &mut Vec<G3CheckResult>,
) {
    let package_path =
        g3ts_astro_check_support::core::package_rel_path(contract).unwrap_or("package.json");
    results.push(g3ts_astro_check_support::core::error(
        ID,
        "Syncpack does not pin the required Astro stack",
        format!(
            "`{rel_path}` {reason}, so the Astro family cannot prove Syncpack pins the required Astro stack for `{package_path}`. Add a parseable `{rel_path}` with canonical pinned `versionGroups` for: {}.",
            g3ts_astro_check_support::core::required_syncpack_pins_message(contract)
        ),
        Some(rel_path),
    ));
}
