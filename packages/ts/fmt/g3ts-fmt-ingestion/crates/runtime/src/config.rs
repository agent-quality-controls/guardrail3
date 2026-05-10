use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_fmt_types::G3TsFmtConfigSurfaceState;

/// Ingests the Prettier config under `app_root_rel_path` into a surface state.
pub(crate) fn ingest_prettier_config(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> G3TsFmtConfigSurfaceState {
    let Some(entry) = crawl.entries.iter().find(|entry| {
        entry.kind == g3_workspace_crawl::G3RsWorkspaceEntryKind::File
            && entry.ignore_state == g3_workspace_crawl::G3RsWorkspaceIgnoreState::Included
            && crate::roots::prettier_config_name(&entry.path.rel_path).is_some()
            && parent_rel_path(&entry.path.rel_path) == app_root_rel_path
    }) else {
        return G3TsFmtConfigSurfaceState::Missing {
            rel_path: crate::roots::scoped_rel_path(app_root_rel_path, "prettier.config.*"),
        };
    };

    if !entry.readable {
        return G3TsFmtConfigSurfaceState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the Prettier config unreadable".to_owned(),
        };
    }

    G3TsFmtConfigSurfaceState::Parsed {
        rel_path: entry.path.rel_path.clone(),
    }
}

/// Returns the parent directory of `rel_path` as a relative path, or `.` for top-level files.
fn parent_rel_path(rel_path: &str) -> String {
    std::path::Path::new(rel_path)
        .parent()
        .and_then(std::path::Path::to_str)
        .filter(|parent| !parent.is_empty())
        .map_or_else(|| ".".to_owned(), str::to_owned)
}
