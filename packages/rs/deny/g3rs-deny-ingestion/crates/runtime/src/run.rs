/// Public ingestion entry point.
use g3rs_deny_types::{
    G3RsDenyConfigChecksInput, G3RsDenyFileTreeChecksInput, G3RsDenyRustPolicyState,
    G3RsDenySourceChecksInput,
};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

/// Re-export of `G3RsDenyIngestionError` so the facade can reach it.
pub use g3rs_deny_ingestion_types::G3RsDenyIngestionError as IngestionError;

/// Ingest the deny config from a workspace crawl into a config checks input.
///
/// Looks for `deny.toml` first, then `.deny.toml`. Returns an error if
/// neither is found, the file is unreadable, or it cannot be parsed.
///
/// # Errors
///
/// Returns an error if the deny config is missing, unreadable, or unparseable.
pub fn ingest_for_config_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsDenyConfigChecksInput, IngestionError> {
    let entry = crate::select::select_deny_toml(crawl).ok_or(IngestionError::DenyTomlNotFound)?;

    if !entry.readable {
        return Err(IngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    let deny = crate::parse::parse_deny_toml(&entry.path.abs_path)?;
    let deny_rel_path = entry.path.rel_path.clone();
    let rust_policy = read_rust_policy_state(crawl);
    Ok(crate::ingest::assemble(deny_rel_path, deny, &rust_policy))
}

/// Stub source ingestion entry point for the deny family.
pub fn ingest_for_source_checks(
    _crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsDenySourceChecksInput, IngestionError> {
    Err(IngestionError::SourceIngestionNotImplemented)
}

pub fn ingest_for_file_tree_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsDenyFileTreeChecksInput, IngestionError> {
    let root_deny_entries = crate::select::root_deny_entries(crawl);
    let candidate_deny_rel_paths = root_deny_entries
        .iter()
        .map(|entry| entry.path.rel_path.clone())
        .collect::<Vec<_>>();
    let selected_deny_rel_path = root_deny_entries
        .first()
        .map(|entry| entry.path.rel_path.clone());
    let mut input_failures = Vec::new();

    for entry in root_deny_entries {
        if !entry.readable {
            input_failures.push(crate::ingest::input_failure(
                "deny input failure",
                entry.path.rel_path.clone(),
                format!(
                    "Failed to read root deny config `{}` for deny checks: file is not readable.",
                    entry.path.rel_path
                ),
            ));
        } else if let Err(error) = crate::parse::parse_deny_toml(&entry.path.abs_path) {
            match error {
                IngestionError::ParseFailed { reason, .. } => {
                    input_failures.push(crate::ingest::input_failure(
                        "deny input failure",
                        entry.path.rel_path.clone(),
                        format!(
                            "Failed to parse root deny config `{}` for deny checks: {reason}",
                            entry.path.rel_path
                        ),
                    ))
                }
                IngestionError::Unreadable { reason, .. } => {
                    input_failures.push(crate::ingest::input_failure(
                        "deny input failure",
                        entry.path.rel_path.clone(),
                        format!(
                            "Failed to read root deny config `{}` for deny checks: {reason}",
                            entry.path.rel_path
                        ),
                    ))
                }
                other => return Err(other),
            }
        }
    }

    if let Some(entry) = crate::select::select_guardrail3_rs_toml(crawl) {
        match crate::parse::parse_rust_policy_state(&entry.path.rel_path, &entry.path.abs_path) {
            G3RsDenyRustPolicyState::Missing | G3RsDenyRustPolicyState::Parsed { .. } => {}
            G3RsDenyRustPolicyState::ParseError { reason, .. } => input_failures.push(
                crate::ingest::input_failure(
                    "deny rust policy is not parseable",
                    entry.path.rel_path.clone(),
                    format!(
                        "Failed to parse root Rust policy `{}` for deny profile resolution: {reason}",
                        entry.path.rel_path
                    ),
                ),
            ),
            G3RsDenyRustPolicyState::Unreadable { reason, .. } => input_failures.push(
                crate::ingest::input_failure(
                    "deny rust policy is not parseable",
                    entry.path.rel_path.clone(),
                    format!(
                        "Failed to parse root Rust policy `{}` for deny profile resolution: {reason}",
                        entry.path.rel_path
                    ),
                ),
            ),
        }
    }

    Ok(crate::ingest::filetree_input(
        selected_deny_rel_path,
        candidate_deny_rel_paths,
        input_failures,
    ))
}

fn read_rust_policy_state(crawl: &G3RsWorkspaceCrawl) -> G3RsDenyRustPolicyState {
    let Some(entry) = crate::select::select_guardrail3_rs_toml(crawl) else {
        return G3RsDenyRustPolicyState::Missing;
    };
    crate::parse::parse_rust_policy_state(&entry.path.rel_path, &entry.path.abs_path)
}

#[cfg(test)]
#[path = "run_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod run_tests;
