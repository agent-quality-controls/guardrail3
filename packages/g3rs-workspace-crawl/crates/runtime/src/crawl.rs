use std::path::Path;

use g3rs_workspace_crawl_types::{G3RsWorkspaceCrawl, G3RsWorkspaceEntry};
use walkdir::WalkDir;

use crate::run::G3RsWorkspaceCrawlError;

pub(crate) fn crawl_workspace(
    workspace_root: &Path,
) -> Result<G3RsWorkspaceCrawl, G3RsWorkspaceCrawlError> {
    if !workspace_root.is_dir() {
        return Err(G3RsWorkspaceCrawlError::InvalidRoot(
            workspace_root.to_path_buf(),
        ));
    }

    let root_abs_path = workspace_root.to_path_buf();
    let ignore_matcher = crate::ignore::build_ignore_matcher(workspace_root);
    let mut entries = Vec::<G3RsWorkspaceEntry>::new();

    for entry in WalkDir::new(workspace_root)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if path == workspace_root {
            continue;
        }
        if crate::support::is_inside_git_dir(workspace_root, path) {
            continue;
        }
        let Some(kind) = crate::support::entry_kind(&entry) else {
            continue;
        };

        entries.push(crate::support::build_entry(
            workspace_root,
            path,
            kind,
            &ignore_matcher,
        ));
    }

    entries.sort_by(|left, right| left.path.rel_path.cmp(&right.path.rel_path));

    Ok(G3RsWorkspaceCrawl {
        root_abs_path,
        entries,
    })
}
