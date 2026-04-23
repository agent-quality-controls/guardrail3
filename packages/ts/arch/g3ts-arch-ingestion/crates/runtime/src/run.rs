use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_arch_types::{
    G3TsArchConfigChecksInput, G3TsArchFileTreeChecksInput, G3TsArchSourceChecksInput,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsArchIngestionError {
    pub message: String,
}

impl std::fmt::Display for G3TsArchIngestionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for G3TsArchIngestionError {}

pub fn ingest_for_config_checks(
    crawl: &G3WorkspaceCrawl,
) -> Result<G3TsArchConfigChecksInput, G3TsArchIngestionError> {
    Ok(G3TsArchConfigChecksInput {
        manifest: crate::manifest::ingest_manifest_state(crawl),
    })
}

pub fn ingest_for_file_tree_checks(
    crawl: &G3WorkspaceCrawl,
) -> Result<G3TsArchFileTreeChecksInput, G3TsArchIngestionError> {
    let manifest = crate::manifest::ingest_manifest_state(crawl);
    Ok(G3TsArchFileTreeChecksInput {
        existing_entrypoints: crate::file_tree::existing_entrypoints(crawl, &manifest),
        manifest,
        source_tree: crate::file_tree::source_tree(crawl),
    })
}

pub fn ingest_for_source_checks(
    crawl: &G3WorkspaceCrawl,
) -> Result<Vec<G3TsArchSourceChecksInput>, G3TsArchIngestionError> {
    let manifest = crate::manifest::ingest_manifest_state(crawl);
    Ok(vec![G3TsArchSourceChecksInput {
        facades: crate::source::facade_states(crawl, &manifest),
    }])
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
