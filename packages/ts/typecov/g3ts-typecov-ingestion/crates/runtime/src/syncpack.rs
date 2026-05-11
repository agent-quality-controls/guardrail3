use g3_workspace_crawl::G3WorkspaceCrawl;
use g3ts_typecov_types::{
    G3TsTypecovSyncpackSnapshot, G3TsTypecovSyncpackSurfaceState,
    G3TsTypecovSyncpackVersionGroupSnapshot,
};

/// `ingest_syncpack_config`: ingest syncpack config.
pub(crate) fn ingest_syncpack_config(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> G3TsTypecovSyncpackSurfaceState {
    let rel_path = crate::roots::scoped_rel_path(app_root_rel_path, ".syncpackrc");
    let Some(entry) = crawl
        .entries
        .iter()
        .find(|entry| included_file(entry) && entry.path.rel_path == rel_path)
    else {
        return G3TsTypecovSyncpackSurfaceState::Missing { rel_path };
    };
    if !entry.readable {
        return G3TsTypecovSyncpackSurfaceState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the Syncpack config unreadable".to_owned(),
        };
    }
    let document = match syncpack_config_parser::from_path_document(&entry.path.abs_path) {
        Ok(document) => document,
        Err(error) => {
            return G3TsTypecovSyncpackSurfaceState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: error.to_string(),
            };
        }
    };
    if let Some(reason) = syncpack_config_parser::parse_error_reason(&document) {
        return G3TsTypecovSyncpackSurfaceState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: reason.to_owned(),
        };
    }
    let Some(typed) = syncpack_config_parser::typed(&document) else {
        return G3TsTypecovSyncpackSurfaceState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: "parsed Syncpack config document was not typed".to_owned(),
        };
    };
    G3TsTypecovSyncpackSurfaceState::Parsed {
        snapshot: G3TsTypecovSyncpackSnapshot {
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

/// `included_file`: included file.
fn included_file(entry: &g3_workspace_crawl::G3WorkspaceEntry) -> bool {
    entry.kind == g3_workspace_crawl::G3WorkspaceEntryKind::File
        && entry.ignore_state == g3_workspace_crawl::G3WorkspaceIgnoreState::Included
}

/// `syncpack_version_group`: syncpack version group.
fn syncpack_version_group(
    group: syncpack_config_parser::types::SyncpackVersionGroup,
) -> G3TsTypecovSyncpackVersionGroupSnapshot {
    G3TsTypecovSyncpackVersionGroupSnapshot {
        dependencies: group.dependencies,
        dependency_types: group.dependency_types,
        packages: group.packages,
        specifier_types: group.specifier_types,
        is_ignored: group.is_ignored,
        is_banned: group.is_banned,
        pin_version: group.pin_version,
    }
}
