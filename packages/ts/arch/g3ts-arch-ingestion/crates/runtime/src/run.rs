use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_arch_types::{
    G3TsArchConfigChecksInput, G3TsArchFileTreeChecksInput, G3TsArchSourceChecksInput,
};

/// Error type for ingestion failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsArchIngestionError {
    /// Human-readable failure description.
    pub message: String,
}

impl std::fmt::Display for G3TsArchIngestionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for G3TsArchIngestionError {}

/// Ingest config-checks input from a workspace crawl.
#[must_use]
pub fn ingest_for_config_checks(crawl: &G3WorkspaceCrawl) -> G3TsArchConfigChecksInput {
    G3TsArchConfigChecksInput {
        manifest: crate::manifest::ingest_manifest_state(crawl),
    }
}

/// Ingest file-tree-checks input from a workspace crawl.
#[must_use]
pub fn ingest_for_file_tree_checks(crawl: &G3WorkspaceCrawl) -> G3TsArchFileTreeChecksInput {
    let manifest = crate::manifest::ingest_manifest_state(crawl);
    G3TsArchFileTreeChecksInput {
        existing_entrypoints: crate::file_tree::existing_entrypoints(crawl, &manifest),
        manifest,
    }
}

/// Ingest source-checks inputs (one per facade) from a workspace crawl.
#[must_use]
pub fn ingest_for_source_checks(crawl: &G3WorkspaceCrawl) -> Vec<G3TsArchSourceChecksInput> {
    let manifest = crate::manifest::ingest_manifest_state(crawl);
    vec![G3TsArchSourceChecksInput {
        facades: crate::source::facade_states(crawl, &manifest),
    }]
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
