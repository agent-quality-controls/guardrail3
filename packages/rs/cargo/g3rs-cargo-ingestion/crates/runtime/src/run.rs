use g3rs_cargo_types::{
    G3RsCargoConfigChecksInput, G3RsCargoFileTreeChecksInput, G3RsCargoPolicyRootKind,
    G3RsCargoRustPolicyState, G3RsCargoSourceChecksInput,
};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

pub use g3rs_cargo_ingestion_types::G3RsCargoIngestionError as IngestionError;

pub fn ingest_for_config_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsCargoConfigChecksInput, IngestionError> {
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

pub fn ingest_for_source_checks(
    _crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsCargoSourceChecksInput, IngestionError> {
    Err(IngestionError::SourceIngestionNotImplemented)
}

pub fn ingest_for_file_tree_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsCargoFileTreeChecksInput, IngestionError> {
    let root_entry =
        crate::select::select_root_cargo_toml(crawl).ok_or(IngestionError::CargoTomlNotFound)?;
    let rust_policy_rel_path =
        crate::select::select_root_rust_policy_toml(crawl).map(|entry| entry.path.rel_path.clone());
    let mut input_failures = Vec::new();
    let mut missing_members = Vec::new();
    let mut kind = None;
    let mut members_parse_error = false;

    if !root_entry.readable {
        input_failures.push(crate::ingest::input_failure(
            root_entry.path.rel_path.clone(),
            "Failed to parse owned policy-root Cargo.toml for cargo lint policy checks: file is not readable.",
        ));
    } else {
        match crate::parse::parse_raw_toml(&root_entry.path.abs_path) {
            Ok(raw_cargo) => {
                kind = Some(crate::select::workspace_root_kind(&raw_cargo));
                match crate::parse::parse_root_cargo_toml(&root_entry.path.abs_path) {
                    Ok(_) => {}
                    Err(IngestionError::ParseFailed { reason, .. }) => {
                        input_failures.push(crate::ingest::input_failure(
                            root_entry.path.rel_path.clone(),
                            format!(
                                "Failed to parse owned policy-root Cargo.toml against cargo-toml-parser for cargo config checks: {reason}"
                            ),
                        ));
                    }
                    Err(IngestionError::Unreadable { reason, .. }) => {
                        input_failures.push(crate::ingest::input_failure(
                            root_entry.path.rel_path.clone(),
                            format!(
                                "Failed to parse owned policy-root Cargo.toml against cargo-toml-parser for cargo config checks: {reason}"
                            ),
                        ));
                    }
                    Err(other) => return Err(other),
                }

                if kind == Some(G3RsCargoPolicyRootKind::WorkspaceRoot) {
                    match crate::select::collect_declared_member_rels(crawl, &raw_cargo) {
                        Ok(member_rels) => {
                            for member_rel in member_rels {
                                match crate::select::select_member_manifest(crawl, &member_rel) {
                                    Some(member_entry) if !member_entry.readable => {
                                        input_failures.push(crate::ingest::input_failure(
                                            member_entry.path.rel_path.clone(),
                                            "Failed to parse workspace member Cargo.toml for cargo lint policy checks: file is not readable.",
                                        ));
                                    }
                                    Some(member_entry) => {
                                        if let Err(error) = crate::parse::parse_raw_toml(
                                            &member_entry.path.abs_path,
                                        ) {
                                            match error {
                                                IngestionError::ParseFailed { reason, .. } => {
                                                    input_failures.push(crate::ingest::input_failure(
                                                        member_entry.path.rel_path.clone(),
                                                        format!(
                                                            "Failed to parse workspace member Cargo.toml for cargo lint policy checks: {reason}"
                                                        ),
                                                    ));
                                                }
                                                IngestionError::Unreadable { reason, .. } => {
                                                    input_failures.push(crate::ingest::input_failure(
                                                        member_entry.path.rel_path.clone(),
                                                        format!(
                                                            "Failed to parse workspace member Cargo.toml for cargo lint policy checks: {reason}"
                                                        ),
                                                    ));
                                                }
                                                other => return Err(other),
                                            }
                                        }
                                    }
                                    None => {
                                        missing_members
                                            .push(crate::ingest::missing_member(member_rel));
                                    }
                                }
                            }
                        }
                        Err(reason) => {
                            members_parse_error = true;
                            input_failures.push(crate::ingest::input_failure(
                                root_entry.path.rel_path.clone(),
                                format!(
                                    "Failed to parse `[workspace].members` for cargo workspace membership checks: {reason}"
                                ),
                            ));
                        }
                    }
                }
            }
            Err(IngestionError::ParseFailed { reason, .. }) => {
                input_failures.push(crate::ingest::input_failure(
                    root_entry.path.rel_path.clone(),
                    format!(
                        "Failed to parse owned policy-root Cargo.toml for cargo lint policy checks: {reason}"
                    ),
                ));
            }
            Err(IngestionError::Unreadable { reason, .. }) => {
                input_failures.push(crate::ingest::input_failure(
                    root_entry.path.rel_path.clone(),
                    format!(
                        "Failed to parse owned policy-root Cargo.toml for cargo lint policy checks: {reason}"
                    ),
                ));
            }
            Err(other) => return Err(other),
        }
    }

    match read_rust_policy_state(crawl) {
        G3RsCargoRustPolicyState::Missing | G3RsCargoRustPolicyState::Parsed { .. } => {}
        G3RsCargoRustPolicyState::Unreadable { rel_path, reason } => {
            input_failures.push(crate::ingest::input_failure(
                rel_path,
                format!(
                    "Failed to parse root-local guardrail3-rs.toml for cargo Rust policy resolution: {reason}"
                ),
            ));
        }
        G3RsCargoRustPolicyState::ParseError { rel_path, reason } => {
            input_failures.push(crate::ingest::input_failure(
                rel_path,
                format!(
                    "Failed to parse root-local guardrail3-rs.toml for cargo Rust policy resolution: {reason}"
                ),
            ));
        }
    }

    Ok(G3RsCargoFileTreeChecksInput {
        root: crate::ingest::filetree_root(kind, rust_policy_rel_path, members_parse_error),
        missing_members,
        input_failures,
    })
}

fn collect_config_members(
    crawl: &G3RsWorkspaceCrawl,
    raw_cargo: &toml::Value,
) -> Result<Vec<g3rs_cargo_types::G3RsCargoWorkspaceMember>, IngestionError> {
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

fn read_rust_policy_state(crawl: &G3RsWorkspaceCrawl) -> G3RsCargoRustPolicyState {
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
