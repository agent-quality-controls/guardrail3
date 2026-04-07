/// Public ingestion entry points.
use std::collections::BTreeSet;

use g3rs_deps_types::{
    G3RsDepsAstChecksInput, G3RsDepsConfigChecksInput, G3RsDepsFileTreeChecksInput,
};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

/// Re-export of `G3RsDepsConfigIngestionError` so the facade can reach it.
pub use g3rs_deps_config_ingestion_types::G3RsDepsConfigIngestionError as IngestionError;

/// Ingest workspace deps config from a workspace crawl into per-crate checks inputs.
pub fn ingest_config(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<Vec<G3RsDepsConfigChecksInput>, IngestionError> {
    let workspace_cargo_entry = crate::select::select_workspace_cargo_toml(crawl)
        .ok_or(IngestionError::CargoTomlNotFound)?;
    if !workspace_cargo_entry.readable {
        return Err(IngestionError::Unreadable {
            path: workspace_cargo_entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    let guardrail_entry = crate::select::select_workspace_guardrail3_rs_toml(crawl)
        .ok_or(IngestionError::Guardrail3RsTomlNotFound)?;
    if !guardrail_entry.readable {
        return Err(IngestionError::Unreadable {
            path: guardrail_entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    let workspace_cargo = crate::parse::parse_cargo_toml(&workspace_cargo_entry.path.abs_path)?;
    let guardrail = crate::parse::parse_guardrail3_rs_toml(&guardrail_entry.path.abs_path)?;

    let member_entries = crate::select::select_member_cargo_tomls(crawl, &workspace_cargo)
        .map_err(|reason| IngestionError::NormalizationFailed {
            path: workspace_cargo_entry.path.abs_path.clone(),
            reason,
        })?;
    let workspace_member_dirs = member_entries
        .iter()
        .filter_map(|entry| entry.path.rel_path.strip_suffix("/Cargo.toml"))
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();

    let mut inputs = Vec::new();
    for member_entry in member_entries {
        if !member_entry.readable {
            return Err(IngestionError::Unreadable {
                path: member_entry.path.abs_path.clone(),
                reason: "file is not readable".to_owned(),
            });
        }

        let member_cargo = crate::parse::parse_cargo_toml(&member_entry.path.abs_path)?;
        let input = crate::ingest::assemble(
            member_entry.path.rel_path.clone(),
            &member_cargo,
            &workspace_cargo,
            &guardrail.config,
            guardrail.allowlist_present,
            &workspace_member_dirs,
        )
        .map_err(|reason| IngestionError::NormalizationFailed {
            path: member_entry.path.abs_path.clone(),
            reason,
        })?;
        inputs.push(input);
    }

    Ok(inputs)
}

/// Stub AST ingestion entry point for the deps family.
pub fn ingest_ast(_crawl: &G3RsWorkspaceCrawl) -> Result<G3RsDepsAstChecksInput, IngestionError> {
    Err(IngestionError::AstIngestionNotImplemented)
}

/// Stub file-tree ingestion entry point for the deps family.
pub fn ingest_file_tree(
    _crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsDepsFileTreeChecksInput, IngestionError> {
    Err(IngestionError::FileTreeIngestionNotImplemented)
}
