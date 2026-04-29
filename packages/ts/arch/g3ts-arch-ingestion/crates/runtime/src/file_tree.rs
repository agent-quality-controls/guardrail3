use g3_workspace_crawl::{G3RsWorkspaceCrawl as G3WorkspaceCrawl, entry};
use g3ts_arch_types::G3TsArchManifestState;

pub(crate) fn existing_entrypoints(
    crawl: &G3WorkspaceCrawl,
    manifest: &G3TsArchManifestState,
) -> Vec<String> {
    let G3TsArchManifestState::Parsed { snapshot } = manifest else {
        return Vec::new();
    };

    snapshot
        .declared_entrypoints
        .iter()
        .filter(|entrypoint| {
            entry(crawl, &entrypoint.rel_path)
                .is_some_and(|workspace_entry| workspace_entry.readable)
        })
        .map(|entrypoint| entrypoint.rel_path.clone())
        .collect()
}
