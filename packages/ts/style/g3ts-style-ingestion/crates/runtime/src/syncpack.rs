use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_style_types::{
    G3TsStyleSyncpackSnapshot, G3TsStyleSyncpackSurfaceState,
    G3TsStyleSyncpackVersionGroupSnapshot,
};

const SYNCPACK_CONFIG_REL_PATH: &str = ".syncpackrc";

pub(crate) fn ingest_syncpack_config(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> G3TsStyleSyncpackSurfaceState {
    let Some(entry) = select_syncpack_config(crawl, app_root_rel_path) else {
        return G3TsStyleSyncpackSurfaceState::Missing {
            rel_path: missing_syncpack_config_rel_path(app_root_rel_path),
        };
    };

    if !entry.readable {
        return G3TsStyleSyncpackSurfaceState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the Syncpack config unreadable".to_owned(),
        };
    }

    let document = match syncpack_config_parser::from_path_document(&entry.path.abs_path) {
        Ok(document) => document,
        Err(error) => {
            return G3TsStyleSyncpackSurfaceState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: error.to_string(),
            };
        }
    };

    if let Some(reason) = syncpack_config_parser::parse_error_reason(&document) {
        return G3TsStyleSyncpackSurfaceState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: reason.to_owned(),
        };
    }

    let typed = syncpack_config_parser::typed(&document)
        .expect("parsed Syncpack config document should stay typed");
    G3TsStyleSyncpackSurfaceState::Parsed {
        snapshot: G3TsStyleSyncpackSnapshot {
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

fn syncpack_version_group(
    group: syncpack_config_parser::types::SyncpackVersionGroup,
) -> G3TsStyleSyncpackVersionGroupSnapshot {
    G3TsStyleSyncpackVersionGroupSnapshot {
        dependencies: group.dependencies,
        dependency_types: group.dependency_types,
        packages: group.packages,
        specifier_types: group.specifier_types,
        is_ignored: group.is_ignored,
        is_banned: group.is_banned,
        pin_version: group.pin_version,
    }
}

fn select_syncpack_config<'crawl>(
    crawl: &'crawl G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> Option<&'crawl g3_workspace_crawl::G3RsWorkspaceEntry> {
    let app_config = crate::roots::scoped_rel_path(app_root_rel_path, SYNCPACK_CONFIG_REL_PATH);
    crawl.entries.iter().find(|entry| {
        entry.kind == g3_workspace_crawl::G3RsWorkspaceEntryKind::File
            && entry.ignore_state == g3_workspace_crawl::G3RsWorkspaceIgnoreState::Included
            && entry.path.rel_path == app_config
    })
}

fn missing_syncpack_config_rel_path(app_root_rel_path: &str) -> String {
    crate::roots::scoped_rel_path(app_root_rel_path, SYNCPACK_CONFIG_REL_PATH)
}

#[cfg(test)]
#[path = "syncpack_tests/mod.rs"]
mod syncpack_tests;
