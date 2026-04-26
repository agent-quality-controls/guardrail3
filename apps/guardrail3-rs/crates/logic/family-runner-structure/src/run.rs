use g3rs_workspace_crawl::G3RsWorkspaceCrawl;
use guardrail3_rs_app_types::{FamilyResults, FamilyRunError, SupportedFamily};

/// Runs the topology, arch, or apparch family group against the prepared crawl.
///
/// # Errors
///
/// Returns [`FamilyRunError`] when ingestion for the selected family fails.
pub fn run(
    family: SupportedFamily,
    crawl: &G3RsWorkspaceCrawl,
) -> Result<FamilyResults, FamilyRunError> {
    match family {
        SupportedFamily::Topology => {
            let input =
                g3rs_topology_ingestion::ingest_for_file_tree_checks(crawl).map_err(|error| {
                    FamilyRunError {
                        message: format!("{error:?}"),
                    }
                })?;
            Ok(g3rs_topology_file_tree_checks::check(&input))
        }
        SupportedFamily::Arch => {
            let config_inputs =
                g3rs_arch_ingestion::ingest_for_config_checks(crawl).map_err(|error| {
                    FamilyRunError {
                        message: format!("{error:?}"),
                    }
                })?;
            let file_tree_input =
                g3rs_arch_ingestion::ingest_for_file_tree_checks(crawl).map_err(|error| {
                    FamilyRunError {
                        message: format!("{error:?}"),
                    }
                })?;
            let source_inputs =
                g3rs_arch_ingestion::ingest_for_source_checks(crawl).map_err(|error| {
                    FamilyRunError {
                        message: format!("{error:?}"),
                    }
                })?;

            let mut results = Vec::new();
            for config_input in config_inputs {
                results.extend(g3rs_arch_config_checks::check(&config_input));
            }
            results.extend(g3rs_arch_file_tree_checks::check(&file_tree_input));
            for source_input in source_inputs {
                results.extend(g3rs_arch_source_checks::check(&source_input));
            }
            Ok(results)
        }
        SupportedFamily::Apparch => {
            let config_input =
                g3rs_apparch_ingestion::ingest_for_config_checks(crawl).map_err(|error| {
                    FamilyRunError {
                        message: format!("{error:?}"),
                    }
                })?;
            let source_input =
                g3rs_apparch_ingestion::ingest_for_source_checks(crawl).map_err(|error| {
                    FamilyRunError {
                        message: format!("{error:?}"),
                    }
                })?;

            let mut results = g3rs_apparch_config_checks::check(&config_input);
            results.extend(g3rs_apparch_source_checks::check(&source_input));
            Ok(results)
        }
        SupportedFamily::Toolchain
        | SupportedFamily::Fmt
        | SupportedFamily::Cargo
        | SupportedFamily::Clippy
        | SupportedFamily::Deny
        | SupportedFamily::Code
        | SupportedFamily::Deps
        | SupportedFamily::Garde
        | SupportedFamily::Test
        | SupportedFamily::Release
        | SupportedFamily::Hooks => Err(FamilyRunError {
            message: format!("structure group does not handle {family:?}"),
        }),
    }
}
