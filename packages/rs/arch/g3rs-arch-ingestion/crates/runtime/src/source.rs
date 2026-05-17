//! Source-tier ingestion: builds the facade-surface and path-attribute inputs
//! consumed by the architecture source checks.

use g3_workspace_crawl::G3WorkspaceCrawl;
use g3rs_arch_types::types as arch_types;

use crate::error::G3RsArchIngestionError;
use crate::source_facade::collect_facade_surfaces;
use crate::source_path_attr::collect_path_attr_sites;
use crate::view::CrawlView;
use crate::workspace::collect_crate_nodes;

/// Result alias for source-tier ingestion routines.
type IngestResult<T> = Result<T, G3RsArchIngestionError>;

/// Source-tier inputs produced for the architecture source checks.
type SourceChecksInputs = Vec<arch_types::G3RsArchSourceChecksInput>;

/// Builds the source-tier ingestion inputs for the architecture checks.
pub(crate) fn ingest_for_source_checks(
    crawl: &G3WorkspaceCrawl,
) -> IngestResult<SourceChecksInputs> {
    let view = CrawlView::new(crawl);
    let crate_nodes = collect_crate_nodes(&view)?;
    let facade_surfaces = collect_facade_surfaces(&view, &crate_nodes);
    let path_attr_sites = collect_path_attr_sites(&view, &crate_nodes)?;

    Ok(vec![arch_types::G3RsArchSourceChecksInput {
        lib_facade_checks: collect_lib_facade_checks(&crate_nodes, &facade_surfaces),
        mod_facade_surfaces: facade_surfaces
            .into_iter()
            .filter(|surface| surface.is_mod_rs)
            .collect(),
        path_attr_sites,
    }])
}

/// Projects crate nodes into the lightweight source-tier crate descriptor.
fn collect_source_crates(
    crate_nodes: &[arch_types::G3RsArchCrateNode],
) -> Vec<arch_types::G3RsArchSourceCrate> {
    crate_nodes
        .iter()
        .map(|node| arch_types::G3RsArchSourceCrate {
            rel_dir: node.rel_dir.clone(),
            lib_rs_rel: node.lib_rs_rel.clone(),
        })
        .collect()
}

/// Pairs each crate with its `lib.rs` facade surface (if any) for the lib-facade checks.
fn collect_lib_facade_checks(
    crate_nodes: &[arch_types::G3RsArchCrateNode],
    facade_surfaces: &[arch_types::G3RsArchFacadeSurface],
) -> Vec<arch_types::G3RsArchLibFacadeChecksInput> {
    let facade_map = facade_surfaces
        .iter()
        .map(|surface| (surface.rel_path.as_str(), surface))
        .collect::<std::collections::BTreeMap<_, _>>();

    collect_source_crates(crate_nodes)
        .into_iter()
        .map(|krate| arch_types::G3RsArchLibFacadeChecksInput {
            lib_surface: krate
                .lib_rs_rel
                .as_deref()
                .and_then(|rel_path| facade_map.get(rel_path).copied().cloned()),
            krate,
        })
        .collect()
}
