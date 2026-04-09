use g3rs_code_ingestion_types::{
    G3RsCodeAstChecksInput, G3RsCodeConfigChecksInput, G3RsCodeFileTreeChecksInput,
};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

/// Re-export of `G3RsCodeIngestionError` so the facade can reach it.
pub use g3rs_code_ingestion_types::G3RsCodeIngestionError as IngestionError;

pub fn ingest_for_config_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsCodeConfigChecksInput, IngestionError> {
    let exception_comments = crate::config_comments::collect_exception_comments(crawl)?;
    let unsafe_code_lints = crate::unsafe_code_lints::collect_unsafe_code_lints(crawl)?;
    Ok(crate::config::assemble(exception_comments, unsafe_code_lints))
}

/// Ingest `code` AST checks input from a workspace crawl.
pub fn ingest_for_ast_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<Vec<G3RsCodeAstChecksInput>, IngestionError> {
    crate::select::select_source_files(crawl)?
        .into_iter()
        .map(|selected| {
            if !selected.entry.readable {
                return Err(IngestionError::Unreadable {
                    path: selected.entry.path.abs_path.clone(),
                    reason: "file is not readable".to_owned(),
                });
            }

            let content =
                crate::fs::read_to_string(&selected.entry.path.abs_path).map_err(|err| {
                    IngestionError::Unreadable {
                        path: selected.entry.path.abs_path.clone(),
                        reason: err.to_string(),
                    }
                })?;

            Ok(crate::ingest::assemble(
                selected.entry.path.rel_path.clone(),
                content,
                selected.is_test,
                selected.profile_name,
                selected.is_library_root,
            ))
        })
        .collect()
}

/// Stub file-tree ingestion entry point for the code family.
pub fn ingest_for_file_tree_checks(
    _crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsCodeFileTreeChecksInput, IngestionError> {
    Err(IngestionError::FileTreeIngestionNotImplemented)
}
