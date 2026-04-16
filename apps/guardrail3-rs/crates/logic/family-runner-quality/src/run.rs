use g3rs_workspace_crawl::G3RsWorkspaceCrawl;
use guardrail3_rs_app_types::{FamilyResults, FamilyRunError, SupportedFamily};

/// Runs the code, deps, or garde family group against the prepared crawl.
///
/// # Errors
///
/// Returns [`FamilyRunError`] when ingestion for the selected family fails.
pub fn run(
    family: SupportedFamily,
    crawl: &G3RsWorkspaceCrawl,
) -> Result<FamilyResults, FamilyRunError> {
    match family {
        SupportedFamily::Code => run_code(crawl),
        SupportedFamily::Deps => run_deps(crawl),
        SupportedFamily::Garde => run_garde(crawl),
        SupportedFamily::Topology
        | SupportedFamily::Toolchain
        | SupportedFamily::Fmt
        | SupportedFamily::Cargo
        | SupportedFamily::Clippy
        | SupportedFamily::Deny
        | SupportedFamily::Arch
        | SupportedFamily::Test
        | SupportedFamily::Release
        | SupportedFamily::Hooks
        | SupportedFamily::Apparch => Err(FamilyRunError {
            message: format!("quality group does not handle {family:?}"),
        }),
    }
}

/// Runs the code family against the prepared crawl.
///
/// # Errors
///
/// Returns [`FamilyRunError`] when ingestion for the code family fails.
fn run_code(crawl: &G3RsWorkspaceCrawl) -> Result<FamilyResults, FamilyRunError> {
    let config_input =
        g3rs_code_ingestion::ingest_for_config_checks(crawl).map_err(|error| FamilyRunError {
            message: format!("{error:?}"),
        })?;
    let mut results = g3rs_code_config_checks::check(&config_input);

    let file_tree_input =
        g3rs_code_ingestion::ingest_for_file_tree_checks(crawl).map_err(|error| {
            FamilyRunError {
                message: format!("{error:?}"),
            }
        })?;
    results.extend(g3rs_code_file_tree_checks::check(&file_tree_input));

    let source_inputs =
        g3rs_code_ingestion::ingest_for_source_checks(crawl).map_err(|error| FamilyRunError {
            message: format!("{error:?}"),
        })?;
    for input in source_inputs {
        results.extend(g3rs_code_source_checks::check(&input));
    }

    Ok(results)
}

/// Runs the deps family against the prepared crawl.
///
/// # Errors
///
/// Returns [`FamilyRunError`] when ingestion for the deps family fails.
fn run_deps(crawl: &G3RsWorkspaceCrawl) -> Result<FamilyResults, FamilyRunError> {
    let config_inputs =
        g3rs_deps_ingestion::ingest_for_config_checks(crawl).map_err(|error| FamilyRunError {
            message: format!("{error:?}"),
        })?;
    let filetree_input =
        g3rs_deps_ingestion::ingest_for_file_tree_checks(crawl).map_err(|error| {
            FamilyRunError {
                message: format!("{error:?}"),
            }
        })?;

    let mut results = Vec::new();
    results.extend(
        config_inputs
            .into_iter()
            .flat_map(|input| g3rs_deps_config_checks::check(&input)),
    );
    results.extend(g3rs_deps_filetree_checks::check(&filetree_input));
    Ok(results)
}

/// Runs the garde family against the prepared crawl.
///
/// # Errors
///
/// Returns [`FamilyRunError`] when ingestion for the garde family fails.
fn run_garde(crawl: &G3RsWorkspaceCrawl) -> Result<FamilyResults, FamilyRunError> {
    let config_input =
        g3rs_garde_ingestion::ingest_for_config_checks(crawl).map_err(|error| FamilyRunError {
            message: format!("{error:?}"),
        })?;
    let source_input =
        g3rs_garde_ingestion::ingest_for_source_checks(crawl).map_err(|error| FamilyRunError {
            message: format!("{error:?}"),
        })?;

    let mut results = g3rs_garde_config_checks::check(&config_input);
    results.extend(g3rs_garde_source_checks::check(&source_input));
    Ok(results)
}
