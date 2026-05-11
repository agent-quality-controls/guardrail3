use g3_workspace_crawl::G3WorkspaceCrawl;
use guardrail3_rs_app_types::{FamilyResults, FamilyRunError, SupportedFamily};

/// Runs the clippy or deny family group against the prepared crawl.
///
/// # Errors
///
/// Returns [`FamilyRunError`] when ingestion for the selected family fails.
pub fn run(
    family: SupportedFamily,
    crawl: &G3WorkspaceCrawl,
) -> Result<FamilyResults, FamilyRunError> {
    match family {
        SupportedFamily::Clippy => {
            let mut results = match g3rs_clippy_ingestion::ingest_for_config_checks(crawl) {
                Ok(config_input) => g3rs_clippy_config_checks::check(&config_input),
                Err(g3rs_clippy_ingestion::G3RsClippyIngestionError::ClippyTomlNotFound) => {
                    Vec::new()
                }
                Err(error) => {
                    return Err(FamilyRunError {
                        message: format!("{error:?}"),
                    });
                }
            };
            let filetree_input = g3rs_clippy_ingestion::ingest_for_file_tree_checks(crawl)
                .map_err(|error| FamilyRunError {
                    message: format!("{error:?}"),
                })?;
            results.extend(g3rs_clippy_filetree_checks::check(&filetree_input));
            Ok(results)
        }
        SupportedFamily::Deny => {
            let mut results = match g3rs_deny_ingestion::ingest_for_config_checks(crawl) {
                Ok(config_input) => g3rs_deny_config_checks::check(&config_input),
                Err(g3rs_deny_ingestion::G3RsDenyIngestionError::DenyTomlNotFound) => Vec::new(),
                Err(error) => {
                    return Err(FamilyRunError {
                        message: format!("{error:?}"),
                    });
                }
            };
            let filetree_input =
                g3rs_deny_ingestion::ingest_for_file_tree_checks(crawl).map_err(|error| {
                    FamilyRunError {
                        message: format!("{error:?}"),
                    }
                })?;
            results.extend(g3rs_deny_filetree_checks::check(&filetree_input));
            Ok(results)
        }
        SupportedFamily::Topology
        | SupportedFamily::Toolchain
        | SupportedFamily::Fmt
        | SupportedFamily::Cargo
        | SupportedFamily::Code
        | SupportedFamily::Arch
        | SupportedFamily::Deps
        | SupportedFamily::Garde
        | SupportedFamily::Test
        | SupportedFamily::Release
        | SupportedFamily::Hooks
        | SupportedFamily::Apparch => Err(FamilyRunError {
            message: format!("policy group does not handle {family:?}"),
        }),
    }
}
