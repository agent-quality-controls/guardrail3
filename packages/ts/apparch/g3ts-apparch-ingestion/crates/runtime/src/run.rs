use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_apparch_types::{G3TsApparchConfigChecksInput, G3TsApparchSourceChecksInput};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsApparchIngestionError {
    pub message: String,
}

impl std::fmt::Display for G3TsApparchIngestionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for G3TsApparchIngestionError {}

pub fn ingest_for_config_checks(
    crawl: &G3WorkspaceCrawl,
) -> Result<G3TsApparchConfigChecksInput, G3TsApparchIngestionError> {
    let facts = crate::source::collect_app_facts(crawl)?;
    Ok(G3TsApparchConfigChecksInput {
        files: facts.files,
        internal_edges: facts.internal_edges,
        external_imports: facts.external_imports,
    })
}

pub fn ingest_for_source_checks(
    crawl: &G3WorkspaceCrawl,
) -> Result<G3TsApparchSourceChecksInput, G3TsApparchIngestionError> {
    let facts = crate::source::collect_app_facts(crawl)?;
    Ok(G3TsApparchSourceChecksInput {
        files: facts.files,
        public_items: facts.public_items,
    })
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
