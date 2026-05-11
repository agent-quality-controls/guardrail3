use g3_workspace_crawl::G3WorkspaceCrawl;
use guardrail3_rs_app_types::{FamilyResults, FamilyRunError, SupportedFamily};

/// Runs the test family against the prepared crawl.
///
/// # Errors
///
/// Returns [`FamilyRunError`] when ingestion for the selected family fails.
pub fn run(
    family: SupportedFamily,
    crawl: &G3WorkspaceCrawl,
) -> Result<FamilyResults, FamilyRunError> {
    match family {
        SupportedFamily::Test => {
            let config_inputs =
                g3rs_test_ingestion::ingest_for_config_checks(crawl).map_err(|error| {
                    FamilyRunError {
                        message: format!("{error:?}"),
                    }
                })?;
            let filetree_inputs =
                g3rs_test_ingestion::ingest_for_file_tree_checks(crawl).map_err(|error| {
                    FamilyRunError {
                        message: format!("{error:?}"),
                    }
                })?;
            let source_inputs =
                g3rs_test_ingestion::ingest_for_source_checks(crawl).map_err(|error| {
                    FamilyRunError {
                        message: format!("{error:?}"),
                    }
                })?;

            let mut results = Vec::new();
            for input in config_inputs {
                results.extend(g3rs_test_config_checks::check(&input));
            }
            for input in filetree_inputs {
                results.extend(g3rs_test_file_tree_checks::check(&input));
            }
            for input in source_inputs {
                results.extend(g3rs_test_source_checks::check(&input));
            }

            Ok(results)
        }
        SupportedFamily::Topology
        | SupportedFamily::Toolchain
        | SupportedFamily::Fmt
        | SupportedFamily::Cargo
        | SupportedFamily::Clippy
        | SupportedFamily::Deny
        | SupportedFamily::Code
        | SupportedFamily::Arch
        | SupportedFamily::Deps
        | SupportedFamily::Garde
        | SupportedFamily::Release
        | SupportedFamily::Hooks
        | SupportedFamily::Apparch => Err(FamilyRunError {
            message: format!("test group does not handle {family:?}"),
        }),
    }
}
