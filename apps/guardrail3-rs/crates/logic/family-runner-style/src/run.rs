use g3rs_workspace_crawl::G3RsWorkspaceCrawl;
use guardrail3_rs_app_types::{FamilyResults, FamilyRunError, SupportedFamily};

/// Runs the toolchain, fmt, or cargo family group against the prepared crawl.
///
/// # Errors
///
/// Returns [`FamilyRunError`] when ingestion for the selected family fails.
pub fn run(
    family: SupportedFamily,
    crawl: &G3RsWorkspaceCrawl,
) -> Result<FamilyResults, FamilyRunError> {
    match family {
        SupportedFamily::Toolchain => {
            let mut results = match g3rs_toolchain_ingestion::ingest_for_config_checks(crawl) {
                Ok(config_input) => g3rs_toolchain_config_checks::check(&config_input),
                Err(
                    g3rs_toolchain_ingestion::G3RsToolchainIngestionError::ToolchainTomlNotFound,
                ) => Vec::new(),
                Err(error) => {
                    return Err(FamilyRunError {
                        message: format!("{error:?}"),
                    });
                }
            };
            let filetree_input = g3rs_toolchain_ingestion::ingest_for_file_tree_checks(crawl)
                .map_err(|error| FamilyRunError {
                    message: format!("{error:?}"),
                })?;
            results.extend(g3rs_toolchain_filetree_checks::check(&filetree_input));
            Ok(results)
        }
        SupportedFamily::Fmt => {
            let mut results = match g3rs_fmt_ingestion::ingest_for_config_checks(crawl) {
                Ok(config_input) => g3rs_fmt_config_checks::check(&config_input),
                Err(g3rs_fmt_ingestion::G3RsFmtIngestionError::RustfmtTomlNotFound) => Vec::new(),
                Err(error) => {
                    return Err(FamilyRunError {
                        message: format!("{error:?}"),
                    });
                }
            };
            let filetree_input =
                g3rs_fmt_ingestion::ingest_for_file_tree_checks(crawl).map_err(|error| {
                    FamilyRunError {
                        message: format!("{error:?}"),
                    }
                })?;
            results.extend(g3rs_fmt_filetree_checks::check(&filetree_input));
            Ok(results)
        }
        SupportedFamily::Cargo => {
            let config_input =
                g3rs_cargo_ingestion::ingest_for_config_checks(crawl).map_err(|error| {
                    FamilyRunError {
                        message: format!("{error:?}"),
                    }
                })?;
            let filetree_input =
                g3rs_cargo_ingestion::ingest_for_file_tree_checks(crawl).map_err(|error| {
                    FamilyRunError {
                        message: format!("{error:?}"),
                    }
                })?;

            let mut results = g3rs_cargo_config_checks::check(&config_input);
            results.extend(g3rs_cargo_filetree_checks::check(&filetree_input));
            Ok(results)
        }
        SupportedFamily::Topology
        | SupportedFamily::Clippy
        | SupportedFamily::Deny
        | SupportedFamily::Code
        | SupportedFamily::Arch
        | SupportedFamily::Deps
        | SupportedFamily::Garde
        | SupportedFamily::Test
        | SupportedFamily::Release
        | SupportedFamily::Hooks
        | SupportedFamily::Apparch => Err(FamilyRunError {
            message: format!("style group does not handle {family:?}"),
        }),
    }
}
