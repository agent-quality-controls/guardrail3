use g3rs_code_ast_checks_types::G3RsCodeAstChecksInput;
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

/// Re-export of `G3RsCodeAstIngestionError` so the facade can reach it.
pub use g3rs_code_ast_ingestion_types::G3RsCodeAstIngestionError as IngestionError;

/// Ingest `code` AST checks input from a workspace crawl.
pub fn ingest_for_ast_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<Vec<G3RsCodeAstChecksInput>, IngestionError> {
    crate::select::select_source_files(crawl)
        .into_iter()
        .map(|selected| {
            if !selected.entry.readable {
                return Err(IngestionError::Unreadable {
                    path: selected.entry.path.abs_path.clone(),
                    reason: "file is not readable".to_owned(),
                });
            }

            let content = crate::fs::read_to_string(&selected.entry.path.abs_path).map_err(|err| {
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
            ))
        })
        .collect()
}
