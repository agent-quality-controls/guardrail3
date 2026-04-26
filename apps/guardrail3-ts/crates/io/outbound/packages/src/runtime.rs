use std::path::Path;

use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use guardrail3_ts_app_types::{WorkspaceCrawlError, WorkspaceCrawler};

#[derive(Debug, Default)]
pub struct PackageRuntime;

impl WorkspaceCrawler for PackageRuntime {
    fn crawl(&self, root: &Path) -> Result<G3WorkspaceCrawl, WorkspaceCrawlError> {
        let root = absolute_root(root).map_err(|error| WorkspaceCrawlError { message: error })?;

        g3_workspace_crawl::crawl(&root).map_err(|error| WorkspaceCrawlError {
            message: format!("{error:?}"),
        })
    }
}

fn absolute_root(root: &Path) -> Result<std::path::PathBuf, String> {
    if root.is_absolute() {
        return Ok(root.to_path_buf());
    }

    std::env::current_dir()
        .map_err(|error| format!("could not resolve current directory: {error}"))
        .map(|cwd| cwd.join(root))
}
