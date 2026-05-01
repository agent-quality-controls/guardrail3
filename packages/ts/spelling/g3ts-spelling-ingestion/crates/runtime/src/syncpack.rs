use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_spelling_types::{
    G3TsSpellingSyncpackSnapshot, G3TsSpellingSyncpackSurfaceState,
    G3TsSpellingSyncpackVersionGroupSnapshot,
};

pub(crate) fn ingest_syncpack_config(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> G3TsSpellingSyncpackSurfaceState {
    let rel_path = crate::roots::scoped_rel_path(app_root_rel_path, ".syncpackrc");
    let Some(entry) = crawl
        .entries
        .iter()
        .find(|entry| included_file(entry) && entry.path.rel_path == rel_path)
    else {
        return G3TsSpellingSyncpackSurfaceState::Missing { rel_path };
    };
    if !entry.readable {
        return G3TsSpellingSyncpackSurfaceState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the Syncpack config unreadable".to_owned(),
        };
    }
    let document = match syncpack_config_parser::from_path_document(&entry.path.abs_path) {
        Ok(document) => document,
        Err(error) => {
            return G3TsSpellingSyncpackSurfaceState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: error.to_string(),
            };
        }
    };
    if let Some(reason) = syncpack_config_parser::parse_error_reason(&document) {
        return G3TsSpellingSyncpackSurfaceState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: reason.to_owned(),
        };
    }
    let typed = syncpack_config_parser::typed(&document)
        .expect("parsed Syncpack config document should stay typed");
    G3TsSpellingSyncpackSurfaceState::Parsed {
        snapshot: G3TsSpellingSyncpackSnapshot {
            rel_path: entry.path.rel_path.clone(),
            source: typed.source.clone(),
            version_groups: typed
                .version_groups
                .iter()
                .cloned()
                .map(syncpack_version_group)
                .collect(),
        },
    }
}

fn included_file(entry: &g3_workspace_crawl::G3RsWorkspaceEntry) -> bool {
    entry.kind == g3_workspace_crawl::G3RsWorkspaceEntryKind::File
        && entry.ignore_state == g3_workspace_crawl::G3RsWorkspaceIgnoreState::Included
}

fn syncpack_version_group(
    group: syncpack_config_parser::types::SyncpackVersionGroup,
) -> G3TsSpellingSyncpackVersionGroupSnapshot {
    G3TsSpellingSyncpackVersionGroupSnapshot {
        dependencies: group.dependencies,
        dependency_types: group.dependency_types,
        packages: group.packages,
        specifier_types: group.specifier_types,
        is_ignored: group.is_ignored,
        is_banned: group.is_banned,
        pin_version: group.pin_version,
    }
}
