use g3ts_astro_types::{
    G3TsAstroConfigChecksInput, G3TsAstroContentMode, G3TsAstroIntegrationContractInput,
    G3TsAstroSyncpackConfigState,
};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-SETUP-CONFIG-10";
const ASTRO_SEO_BAN_REASON: &str = "`astro-seo` is forbidden because `astro-seo@1.1.0` exports TypeScript source directly from the package entry point. Astro apps must use the approved SEO path instead: typed content/layout data, `schema-dts` for JSON-LD types, `@nuasite/checks` with `g3ts-astro-nuasite-checks` for rendered-output verification, `@astrojs/sitemap`, and `astro-robots`.";

pub(crate) fn check(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.integration_contracts {
        match &contract.syncpack_config {
            G3TsAstroSyncpackConfigState::Parsed { snapshot } => {
                let package_path =
                    g3ts_astro_check_support::core::package_rel_path(contract).unwrap_or("package.json");
                let active_forbidden_deps = active_forbidden_deps(contract);
                if !snapshot.source_covers_package_manifest {
                    let expected_source = g3ts_astro_check_support::core::expected_syncpack_source_entry(
                        &snapshot.rel_path,
                        package_path,
                    )
                    .unwrap_or_else(|| package_path.to_owned());
                    results.push(g3ts_astro_check_support::core::error(
                        ID,
                        "Syncpack does not ban forbidden Astro deps",
                        format!(
                            "`{}` does not include exact `source` entry `{expected_source}` for `{package_path}`, so `syncpack lint` cannot reject forbidden dependencies for this Astro app. {ASTRO_SEO_BAN_REASON}",
                            snapshot.rel_path,
                        ),
                        Some(&snapshot.rel_path),
                    ));
                }

                let missing_forbidden_bans = snapshot
                    .missing_forbidden_bans
                    .iter()
                    .filter(|dependency| {
                        active_forbidden_deps
                            .iter()
                            .any(|active| *active == dependency.as_str())
                    })
                    .cloned()
                    .collect::<Vec<_>>();

                if missing_forbidden_bans.is_empty() && snapshot.source_covers_package_manifest {
                    results.push(g3ts_astro_check_support::core::info(
                        ID,
                        "Syncpack bans forbidden Astro deps",
                        format!(
                            "`{}` bans forbidden Astro deps through Syncpack: {}.",
                            snapshot.rel_path,
                            active_forbidden_deps_message(&active_forbidden_deps)
                        ),
                        &snapshot.rel_path,
                    ));
                    continue;
                }

                if !missing_forbidden_bans.is_empty() {
                    results.push(g3ts_astro_check_support::core::error(
                        ID,
                        "Syncpack does not ban forbidden Astro deps",
                        format!(
                            "`{}` is missing Syncpack banned versionGroups for: {}. Add exactly one canonical banned versionGroup per listed dependency, with exact `dependencies`, `dependencyTypes` containing exactly `prod`, `dev`, `optional`, and `peer`, `isBanned: true`, and no `packages` or `specifierTypes`.{}",
                            snapshot.rel_path,
                            missing_forbidden_bans
                                .iter()
                                .map(|dependency| format!("`{dependency}`"))
                                .collect::<Vec<_>>()
                                .join(", "),
                            forbidden_dependency_explanation(&missing_forbidden_bans)
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
    let package_path =
        g3ts_astro_check_support::core::package_rel_path(contract).unwrap_or("package.json");
    results.push(g3ts_astro_check_support::core::error(
        ID,
        "Syncpack does not ban forbidden Astro deps",
        format!(
            "`{rel_path}` {reason}, so the Astro family cannot prove Syncpack bans forbidden Astro deps for `{package_path}`. Add a parseable `{rel_path}` with canonical `isBanned: true` versionGroups for {}. {ASTRO_SEO_BAN_REASON}",
            active_forbidden_deps_message(&active_forbidden_deps(contract))
        ),
        Some(rel_path),
    ));
}

fn active_forbidden_deps(contract: &G3TsAstroIntegrationContractInput) -> Vec<&str> {
    contract
        .forbidden_syncpack_deps
        .iter()
        .map(String::as_str)
        .filter(|dependency| {
            content_mode_requires_parallel_content_ban(contract.content_mode)
                || !is_contentlayer_dependency(dependency)
        })
        .collect()
}

fn active_forbidden_deps_message(dependencies: &[&str]) -> String {
    dependencies
        .iter()
        .map(|dependency| format!("`{dependency}`"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn content_mode_requires_parallel_content_ban(content_mode: G3TsAstroContentMode) -> bool {
    matches!(
        content_mode,
        G3TsAstroContentMode::BuildCollections | G3TsAstroContentMode::LiveCollections
    )
}

fn is_contentlayer_dependency(dependency: &str) -> bool {
    matches!(
        dependency,
        "contentlayer" | "next-contentlayer" | "@contentlayer/core" | "@contentlayer/source-files"
    )
}

fn forbidden_dependency_explanation(missing_forbidden_bans: &[String]) -> String {
    if missing_forbidden_bans
        .iter()
        .any(|dependency| dependency == "astro-seo")
    {
        format!(" {ASTRO_SEO_BAN_REASON}")
    } else {
        String::new()
    }
}
