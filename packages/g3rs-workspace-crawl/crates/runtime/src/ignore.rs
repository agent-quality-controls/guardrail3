use std::path::Path;

use g3rs_workspace_crawl_types::G3RsWorkspaceIgnoreState;
use ignore::gitignore::{Gitignore, GitignoreBuilder};

pub(crate) fn build_ignore_matcher(workspace_root: &Path) -> Gitignore {
    let mut builder = GitignoreBuilder::new(workspace_root);
    let _ = builder.add(workspace_root.join(".gitignore"));
    builder.build().unwrap_or_else(|_| Gitignore::empty())
}

pub(crate) fn ignore_state(
    ignore_matcher: &Gitignore,
    path: &Path,
    is_dir: bool,
) -> G3RsWorkspaceIgnoreState {
    if ignore_matcher.matched(path, is_dir).is_ignore() {
        G3RsWorkspaceIgnoreState::Ignored
    } else {
        G3RsWorkspaceIgnoreState::Included
    }
}
