use g3_workspace_crawl::G3WorkspaceCrawl;
use g3rs_cargo_types::{
    G3RsCargoConfigChecksInput, G3RsCargoFileTreeChecksInput, G3RsCargoInputFailure,
    G3RsCargoPolicyRootKind, G3RsCargoRustPolicyState, G3RsCargoSourceChecksInput,
    G3RsCargoWorkspaceMember,
};

pub use g3rs_cargo_ingestion_types::G3RsCargoIngestionError as IngestionError;

/// Alias for fallible ingestion outputs of this crate.
type IngestResult<T> = Result<T, IngestionError>;

/// Build the config-checks input for a workspace crawl.
///
/// # Errors
///
/// Returns an error when the workspace root `Cargo.toml` is missing, unreadable,
/// or fails strict parsing, or when workspace-member resolution surfaces a
/// non-recoverable error.
pub fn ingest_for_config_checks(
    crawl: &G3WorkspaceCrawl,
) -> IngestResult<G3RsCargoConfigChecksInput> {
    let root_entry =
        crate::select::select_root_cargo_toml(crawl).ok_or(IngestionError::CargoTomlNotFound)?;
    if !root_entry.readable {
        return Err(IngestionError::Unreadable {
            path: root_entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    let raw_cargo = crate::parse::parse_raw_toml(&root_entry.path.abs_path)?;
    let cargo = crate::parse::parse_root_cargo_toml(&root_entry.path.abs_path)?;
    let rust_policy = read_rust_policy_state(crawl);
    let root = crate::ingest::build_root(root_entry.path.rel_path.clone(), cargo, rust_policy);

    let workspace_members = if root.kind == G3RsCargoPolicyRootKind::WorkspaceRoot {
        collect_config_members(crawl, &raw_cargo)?
    } else {
        Vec::new()
    };

    Ok(G3RsCargoConfigChecksInput {
        root,
        workspace_members,
    })
}

/// Build the source-checks input for a workspace crawl.
///
/// # Errors
///
/// Currently always returns [`IngestionError::SourceIngestionNotImplemented`].
pub const fn ingest_for_source_checks(
    _crawl: &G3WorkspaceCrawl,
) -> IngestResult<G3RsCargoSourceChecksInput> {
    Err(IngestionError::SourceIngestionNotImplemented)
}

/// Build the file-tree-checks input for a workspace crawl.
///
/// # Errors
///
/// Returns an error only for non-recoverable ingestion failures; soft failures
/// (parse error, unreadable) are converted into `input_failures` entries.
pub fn ingest_for_file_tree_checks(
    crawl: &G3WorkspaceCrawl,
) -> IngestResult<G3RsCargoFileTreeChecksInput> {
    let root_entry =
        crate::select::select_root_cargo_toml(crawl).ok_or(IngestionError::CargoTomlNotFound)?;
    let rust_policy_rel_path =
        crate::select::select_root_rust_policy_toml(crawl).map(|entry| entry.path.rel_path.clone());

    let mut acc = FileTreeIngestAccumulator::default();
    if root_entry.readable {
        ingest_root_for_file_tree(crawl, root_entry, &mut acc)?;
    } else {
        acc.input_failures.push(crate::ingest::input_failure(
            root_entry.path.rel_path.clone(),
            "Failed to parse owned policy-root Cargo.toml for cargo lint policy checks: file is not readable.",
        ));
    }

    append_rust_policy_input_failures(crawl, &mut acc.input_failures);

    Ok(G3RsCargoFileTreeChecksInput {
        root: crate::ingest::filetree_root(acc.kind, rust_policy_rel_path, acc.members_parse_error),
        missing_members: acc.missing_members,
        input_failures: acc.input_failures,
    })
}

/// Accumulator for the file-tree ingestion pass.
#[derive(Default)]
struct FileTreeIngestAccumulator {
    /// Detected policy-root kind, if the root Cargo.toml parsed successfully.
    kind: Option<G3RsCargoPolicyRootKind>,
    /// Soft failures to surface as `input_failures`.
    input_failures: Vec<G3RsCargoInputFailure>,
    /// Declared workspace members that were not found in the crawl.
    missing_members: Vec<g3rs_cargo_types::G3RsCargoMissingMember>,
    /// `true` when the `[workspace].members` array itself failed to parse.
    members_parse_error: bool,
}

/// Ingest the workspace root Cargo.toml into the file-tree accumulator.
fn ingest_root_for_file_tree(
    crawl: &G3WorkspaceCrawl,
    root_entry: &g3_workspace_crawl::G3WorkspaceEntry,
    acc: &mut FileTreeIngestAccumulator,
) -> IngestResult<()> {
    let raw_cargo = match crate::parse::parse_raw_toml(&root_entry.path.abs_path) {
        Ok(raw_cargo) => raw_cargo,
        Err(error) => {
            push_root_parse_failure(error, &root_entry.path.rel_path, &mut acc.input_failures)?;
            return Ok(());
        }
    };
    acc.kind = Some(crate::select::workspace_root_kind(&raw_cargo));

    if let Err(error) = crate::parse::parse_root_cargo_toml(&root_entry.path.abs_path) {
        push_root_typed_parse_failure(error, &root_entry.path.rel_path, &mut acc.input_failures)?;
    }

    if acc.kind == Some(G3RsCargoPolicyRootKind::WorkspaceRoot) {
        ingest_workspace_members_for_file_tree(crawl, &raw_cargo, root_entry, acc)?;
    }
    Ok(())
}

/// Push an input failure for a raw-parse failure on the policy root, or return
/// a hard error for other ingestion variants.
fn push_root_parse_failure(
    error: IngestionError,
    rel_path: &str,
    input_failures: &mut Vec<G3RsCargoInputFailure>,
) -> IngestResult<()> {
    match error {
        IngestionError::ParseFailed { reason, .. } | IngestionError::Unreadable { reason, .. } => {
            input_failures.push(crate::ingest::input_failure(
                rel_path.to_owned(),
                format!(
                    "Failed to parse owned policy-root Cargo.toml for cargo lint policy checks: {reason}"
                ),
            ));
            Ok(())
        }
        other @ (IngestionError::CargoTomlNotFound
        | IngestionError::SourceIngestionNotImplemented
        | IngestionError::FileTreeIngestionNotImplemented
        | IngestionError::NormalizationFailed { .. }) => Err(other),
    }
}

/// Push an input failure for a typed-parse failure on the policy root, or
/// return a hard error for other ingestion variants.
fn push_root_typed_parse_failure(
    error: IngestionError,
    rel_path: &str,
    input_failures: &mut Vec<G3RsCargoInputFailure>,
) -> IngestResult<()> {
    match error {
        IngestionError::ParseFailed { reason, .. } | IngestionError::Unreadable { reason, .. } => {
            input_failures.push(crate::ingest::input_failure(
                rel_path.to_owned(),
                format!(
                    "Failed to parse owned policy-root Cargo.toml against cargo-toml-parser for cargo config checks: {reason}"
                ),
            ));
            Ok(())
        }
        other @ (IngestionError::CargoTomlNotFound
        | IngestionError::SourceIngestionNotImplemented
        | IngestionError::FileTreeIngestionNotImplemented
        | IngestionError::NormalizationFailed { .. }) => Err(other),
    }
}

/// Ingest declared workspace members into the file-tree accumulator.
fn ingest_workspace_members_for_file_tree(
    crawl: &G3WorkspaceCrawl,
    raw_cargo: &toml::Value,
    root_entry: &g3_workspace_crawl::G3WorkspaceEntry,
    acc: &mut FileTreeIngestAccumulator,
) -> IngestResult<()> {
    let member_rels = match crate::select::collect_declared_member_rels(crawl, raw_cargo) {
        Ok(member_rels) => member_rels,
        Err(reason) => {
            acc.members_parse_error = true;
            acc.input_failures.push(crate::ingest::input_failure(
                root_entry.path.rel_path.clone(),
                format!(
                    "Failed to parse `[workspace].members` for cargo workspace membership checks: {reason}"
                ),
            ));
            return Ok(());
        }
    };

    for member_rel in member_rels {
        match crate::select::select_member_manifest(crawl, &member_rel) {
            Some(member_entry) if !member_entry.readable => {
                acc.input_failures.push(crate::ingest::input_failure(
                    member_entry.path.rel_path.clone(),
                    "Failed to parse workspace member Cargo.toml for cargo lint policy checks: file is not readable.",
                ));
            }
            Some(member_entry) => {
                if let Err(error) = crate::parse::parse_raw_toml(&member_entry.path.abs_path) {
                    push_member_parse_failure(
                        error,
                        &member_entry.path.rel_path,
                        &mut acc.input_failures,
                    )?;
                }
            }
            None => {
                acc.missing_members
                    .push(crate::ingest::missing_member(member_rel));
            }
        }
    }
    Ok(())
}

/// Push an input failure for a member parse failure, or return a hard error.
fn push_member_parse_failure(
    error: IngestionError,
    rel_path: &str,
    input_failures: &mut Vec<G3RsCargoInputFailure>,
) -> IngestResult<()> {
    match error {
        IngestionError::ParseFailed { reason, .. } | IngestionError::Unreadable { reason, .. } => {
            input_failures.push(crate::ingest::input_failure(
                rel_path.to_owned(),
                format!(
                    "Failed to parse workspace member Cargo.toml for cargo lint policy checks: {reason}"
                ),
            ));
            Ok(())
        }
        other @ (IngestionError::CargoTomlNotFound
        | IngestionError::SourceIngestionNotImplemented
        | IngestionError::FileTreeIngestionNotImplemented
        | IngestionError::NormalizationFailed { .. }) => Err(other),
    }
}

/// Append rust-policy soft-failure entries to the `input_failures` vec.
fn append_rust_policy_input_failures(
    crawl: &G3WorkspaceCrawl,
    input_failures: &mut Vec<G3RsCargoInputFailure>,
) {
    match read_rust_policy_state(crawl) {
        G3RsCargoRustPolicyState::Missing | G3RsCargoRustPolicyState::Parsed { .. } => {}
        G3RsCargoRustPolicyState::Unreadable { rel_path, reason }
        | G3RsCargoRustPolicyState::ParseError { rel_path, reason } => {
            input_failures.push(crate::ingest::input_failure(
                rel_path,
                format!(
                    "Failed to parse root-local guardrail3-rs.toml for cargo Rust policy resolution: {reason}"
                ),
            ));
        }
    }
}

/// collect config members fn.
fn collect_config_members(
    crawl: &G3WorkspaceCrawl,
    raw_cargo: &toml::Value,
) -> IngestResult<Vec<G3RsCargoWorkspaceMember>> {
    let member_rels =
        crate::select::collect_declared_member_rels(crawl, raw_cargo).map_err(|reason| {
            IngestionError::NormalizationFailed {
                path: crawl.root_abs_path.join("Cargo.toml"),
                reason,
            }
        })?;

    let mut members = Vec::new();
    for member_rel in member_rels {
        let Some(member_entry) = crate::select::select_member_manifest(crawl, &member_rel) else {
            continue;
        };
        if !member_entry.readable {
            continue;
        }
        let cargo_member = match crate::parse::parse_member_cargo_toml(&member_entry.path.abs_path)
        {
            Ok(cargo_member) => cargo_member,
            Err(IngestionError::ParseFailed { .. } | IngestionError::Unreadable { .. }) => continue,
            Err(other) => return Err(other),
        };
        members.push(crate::ingest::build_member(
            member_rel,
            member_entry.path.rel_path.clone(),
            cargo_member,
        ));
    }
    members.sort_by(|left, right| left.member_rel.cmp(&right.member_rel));
    Ok(members)
}

/// Read the parsed rust-policy state for the workspace, if any.
fn read_rust_policy_state(crawl: &G3WorkspaceCrawl) -> G3RsCargoRustPolicyState {
    let Some(entry) = crate::select::select_root_rust_policy_toml(crawl) else {
        return G3RsCargoRustPolicyState::Missing;
    };
    if !entry.readable {
        return G3RsCargoRustPolicyState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "file is not readable".to_owned(),
        };
    }
    crate::parse::parse_rust_policy_state(&entry.path.rel_path, &entry.path.abs_path)
}

#[cfg(test)]
#[path = "run_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod run_tests;
