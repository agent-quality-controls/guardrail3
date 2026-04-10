use std::path::{Path, PathBuf};

use g3rs_workspace_crawl::{crawl, G3RsWorkspaceCrawl};

mod pipeline;
mod reachability;
mod selection;
mod source_layout;

pub(super) fn workspace_root(root: &Path) -> PathBuf {
    if root.join("Cargo.toml").exists() {
        root.to_path_buf()
    } else {
        root.join("apps/demo")
    }
}

pub(super) fn crawl_workspace(root: &Path) -> G3RsWorkspaceCrawl {
    crawl(&workspace_root(root)).expect("crawl")
}
