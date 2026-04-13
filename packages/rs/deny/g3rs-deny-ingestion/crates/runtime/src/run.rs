/// Public ingestion entry point.
use g3rs_deny_types::{
    G3RsDenySourceChecksInput, G3RsDenyConfigChecksInput, G3RsDenyFileTreeChecksInput,
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
    let entry = crate::select::select_deny_toml(crawl)
        .ok_or(IngestionError::DenyTomlNotFound)?;

    if !entry.readable {
        return Err(IngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    let deny = crate::parse::parse_deny_toml(&entry.path.abs_path)?;
    let deny_rel_path = entry.path.rel_path.clone();
    let guardrail = read_guardrail_state(crawl);
    Ok(crate::ingest::assemble(deny_rel_path, deny, &guardrail))
}

/// Stub source ingestion entry point for the deny family.
pub fn ingest_for_source_checks(_crawl: &G3RsWorkspaceCrawl) -> Result<G3RsDenySourceChecksInput, IngestionError> {
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
    let selected_deny_rel_path = root_deny_entries.first().map(|entry| entry.path.rel_path.clone());
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
                IngestionError::ParseFailed { reason, .. } => input_failures.push(crate::ingest::input_failure(
                    "deny input failure",
                    entry.path.rel_path.clone(),
                    format!(
                        "Failed to parse root deny config `{}` for deny checks: {reason}",
                        entry.path.rel_path
                    ),
                )),
                IngestionError::Unreadable { reason, .. } => input_failures.push(crate::ingest::input_failure(
                    "deny input failure",
                    entry.path.rel_path.clone(),
                    format!(
                        "Failed to read root deny config `{}` for deny checks: {reason}",
                        entry.path.rel_path
                    ),
                )),
                other => return Err(other),
            }
        }
    }

    if let Some(entry) = crate::select::select_guardrail3_toml(crawl) {
        if !entry.readable {
            input_failures.push(crate::ingest::input_failure(
                "deny policy context is not parseable",
                entry.path.rel_path.clone(),
                "Failed to parse root-local guardrail3.toml for deny profile resolution: file is not readable.",
            ));
        } else {
            match crate::parse::parse_raw_toml(&entry.path.abs_path) {
                Ok(raw_guardrail) => {
                    if crate::ingest::profile_name_from_guardrail(&raw_guardrail).is_err() {
                        input_failures.push(crate::ingest::input_failure(
                            "deny policy context is not parseable",
                            entry.path.rel_path.clone(),
                            "Failed to parse root-local guardrail3.toml for deny profile resolution: unsupported policy shape.".to_owned(),
                        ));
                    }
                }
                Err(IngestionError::ParseFailed { reason, .. }) => input_failures.push(crate::ingest::input_failure(
                    "deny policy context is not parseable",
                    entry.path.rel_path.clone(),
                    format!("Failed to parse root-local guardrail3.toml for deny profile resolution: {reason}"),
                )),
                Err(IngestionError::Unreadable { reason, .. }) => input_failures.push(crate::ingest::input_failure(
                    "deny policy context is not parseable",
                    entry.path.rel_path.clone(),
                    format!("Failed to parse root-local guardrail3.toml for deny profile resolution: {reason}"),
                )),
                Err(other) => return Err(other),
            }
        }
    }

    Ok(crate::ingest::filetree_input(
        selected_deny_rel_path,
        candidate_deny_rel_paths,
        input_failures,
    ))
}

fn read_guardrail_state(crawl: &G3RsWorkspaceCrawl) -> crate::ingest::GuardrailState {
    let Some(entry) = crate::select::select_guardrail3_toml(crawl) else {
        return crate::ingest::GuardrailState::default();
    };
    if !entry.readable {
        return crate::ingest::GuardrailState {
            profile_name: None,
            parse_error: true,
        };
    }

    let Ok(raw_guardrail) = crate::parse::parse_raw_toml(&entry.path.abs_path) else {
        return crate::ingest::GuardrailState {
            profile_name: None,
            parse_error: true,
        };
    };
    let Ok(profile_name) = crate::ingest::profile_name_from_guardrail(&raw_guardrail) else {
        return crate::ingest::GuardrailState {
            profile_name: None,
            parse_error: true,
        };
    };

    crate::ingest::GuardrailState {
        profile_name,
        parse_error: false,
    }
}
