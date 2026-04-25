use g3ts_astro_types::{
    G3TsAstroConfigChecksInput, G3TsAstroIntegrationContractInput, G3TsAstroSyncpackConfigState,
};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-CONFIG-10";

pub(crate) fn check(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.integration_contracts {
        match &contract.syncpack_config {
            G3TsAstroSyncpackConfigState::Parsed { snapshot } => {
                let package_path =
                    crate::support::package_rel_path(contract).unwrap_or("package.json");
                if !snapshot.source_covers_package_manifest {
                    let expected_source = crate::support::expected_syncpack_source_entry(
                        &snapshot.rel_path,
                        package_path,
                    )
                    .unwrap_or_else(|| package_path.to_owned());
                    results.push(crate::support::error(
                        ID,
                        "Syncpack does not ban forbidden Astro deps",
                        format!(
                            "`{}` does not include exact `source` entry `{expected_source}` for `{package_path}`, so `syncpack lint` cannot reject forbidden dependencies for this Astro app.",
                            snapshot.rel_path,
                        ),
                        Some(&snapshot.rel_path),
                    ));
                }

                if snapshot.missing_forbidden_bans.is_empty()
                    && snapshot.source_covers_package_manifest
                {
                    results.push(crate::support::info(
                        ID,
                        "Syncpack bans forbidden Astro deps",
                        format!(
                            "`{}` bans forbidden Astro deps through Syncpack: {}.",
                            snapshot.rel_path,
                            crate::support::forbidden_syncpack_deps_message(contract)
                        ),
                        &snapshot.rel_path,
                    ));
                    continue;
                }

                if !snapshot.missing_forbidden_bans.is_empty() {
                    results.push(crate::support::error(
                        ID,
                        "Syncpack does not ban forbidden Astro deps",
                        format!(
                        "`{}` is missing Syncpack banned versionGroups for: {}. Add one canonical banned versionGroup per listed dependency before any app-specific groups, with exact `dependencies`, `dependencyTypes: [\"prod\", \"dev\", \"optional\", \"peer\"]`, `isBanned: true`, and no `packages` or `specifierTypes`.",
                        snapshot.rel_path,
                        snapshot
                            .missing_forbidden_bans
                            .iter()
                            .map(|dependency| format!("`{dependency}`"))
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
    contract: &G3TsAstroIntegrationContractInput,
    rel_path: &str,
    reason: &str,
    results: &mut Vec<G3CheckResult>,
) {
    let package_path = crate::support::package_rel_path(contract).unwrap_or("package.json");
    results.push(crate::support::error(
        ID,
        "Syncpack does not ban forbidden Astro deps",
        format!(
            "`{rel_path}` {reason}, so the Astro family cannot prove Syncpack bans forbidden Astro deps for `{package_path}`. Add a parseable `{rel_path}` with canonical `isBanned: true` versionGroups for {}.",
            crate::support::forbidden_syncpack_deps_message(contract)
        ),
        Some(rel_path),
    ));
}
