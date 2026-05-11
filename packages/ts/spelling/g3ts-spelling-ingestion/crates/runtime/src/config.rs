use g3_workspace_crawl::G3WorkspaceCrawl;
use g3ts_spelling_types::G3TsSpellingConfigSurfaceState;

/// Read and parse the cspell config under `app_root_rel_path` from `crawl`,
/// returning a surface-state describing what was found.
pub(crate) fn ingest_cspell_config(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> G3TsSpellingConfigSurfaceState {
    let Some(entry) = crawl.entries.iter().find(|entry| {
        entry.kind == g3_workspace_crawl::G3WorkspaceEntryKind::File
            && entry.ignore_state == g3_workspace_crawl::G3WorkspaceIgnoreState::Included
            && crate::roots::cspell_config_name(&entry.path.rel_path).is_some()
            && parent_rel_path(&entry.path.rel_path) == app_root_rel_path
    }) else {
        return G3TsSpellingConfigSurfaceState::Missing {
            rel_path: crate::roots::scoped_rel_path(app_root_rel_path, "cspell.config.*"),
        };
    };

    if !entry.readable {
        return G3TsSpellingConfigSurfaceState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the cspell config unreadable".to_owned(),
        };
    }

    if crate::roots::cspell_json_config_name(&entry.path.rel_path).is_some() {
        let document = match cspell_config_parser::from_path_document(&entry.path.abs_path) {
            Ok(document) => document,
            Err(error) => {
                return G3TsSpellingConfigSurfaceState::ParseError {
                    rel_path: entry.path.rel_path.clone(),
                    reason: error.to_string(),
                };
            }
        };
        if let Some(reason) = cspell_config_parser::parse_error_reason(&document) {
            return G3TsSpellingConfigSurfaceState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: reason.to_owned(),
            };
        }
    }

    G3TsSpellingConfigSurfaceState::Parsed {
        rel_path: entry.path.rel_path.clone(),
    }
}

/// Return the parent directory of `rel_path` as a workspace-relative path,
/// using `.` for the workspace root.
fn parent_rel_path(rel_path: &str) -> String {
    std::path::Path::new(rel_path)
        .parent()
        .and_then(std::path::Path::to_str)
        .filter(|parent| !parent.is_empty())
        .map_or_else(|| ".".to_owned(), str::to_owned)
}
