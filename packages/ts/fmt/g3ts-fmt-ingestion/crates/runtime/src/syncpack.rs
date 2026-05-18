use g3_workspace_crawl::G3WorkspaceCrawl;
use g3ts_fmt_types::{G3TsFmtSyncpackSnapshot, G3TsFmtSyncpackSurfaceState};

/// Ingests the Syncpack config under `app_root_rel_path` into a surface state.
pub(crate) fn ingest_syncpack_config(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> G3TsFmtSyncpackSurfaceState {
    let rel_path = crate::roots::scoped_rel_path(app_root_rel_path, ".syncpackrc");
    let Some(entry) = crawl
        .entries
        .iter()
        .find(|entry| entry.path.rel_path == rel_path)
    else {
        return G3TsFmtSyncpackSurfaceState::Missing { rel_path };
    };
    if !entry.readable {
        return G3TsFmtSyncpackSurfaceState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the Syncpack config unreadable".to_owned(),
        };
    }
    let document = match syncpack_config_parser::from_path_document(&entry.path.abs_path) {
        Ok(document) => document,
        Err(error) => {
            return G3TsFmtSyncpackSurfaceState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: error.to_string(),
            };
        }
    };
    if let Some(reason) = syncpack_config_parser::parse_error_reason(&document) {
        return G3TsFmtSyncpackSurfaceState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: reason.to_owned(),
        };
    }
    let Some(typed) = syncpack_config_parser::typed(&document) else {
        return G3TsFmtSyncpackSurfaceState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: "Syncpack config parsed without typed data".to_owned(),
        };
    };
    G3TsFmtSyncpackSurfaceState::Parsed {
        snapshot: G3TsFmtSyncpackSnapshot {
            rel_path: entry.path.rel_path.clone(),
            source: typed.source.clone(),
            version_groups: typed.version_groups.clone(),
        },
    }
}
