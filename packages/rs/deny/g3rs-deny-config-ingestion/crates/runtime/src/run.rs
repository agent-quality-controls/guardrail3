/// Public ingestion entry point.
use g3rs_deny_types::{
    G3RsDenyAstChecksInput, G3RsDenyConfigChecksInput, G3RsDenyFileTreeChecksInput,
};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

/// Re-export of `G3RsDenyConfigIngestionError` so the facade can reach it.
pub use g3rs_deny_config_ingestion_types::G3RsDenyConfigIngestionError as IngestionError;

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
    Ok(crate::ingest::assemble(deny_rel_path, deny))
}

/// Stub AST ingestion entry point for the deny family.
pub fn ingest_for_ast_checks(_crawl: &G3RsWorkspaceCrawl) -> Result<G3RsDenyAstChecksInput, IngestionError> {
    Err(IngestionError::AstIngestionNotImplemented)
}

/// Stub file-tree ingestion entry point for the deny family.
pub fn ingest_for_file_tree_checks(
    _crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsDenyFileTreeChecksInput, IngestionError> {
    Err(IngestionError::FileTreeIngestionNotImplemented)
}
