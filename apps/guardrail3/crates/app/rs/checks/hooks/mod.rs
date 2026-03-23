pub mod rs;
pub mod shared;
pub(crate) mod shell;

use std::path::Path;

use crate::domain::project_tree::ProjectTree;
use crate::domain::report::CheckResult;
use crate::ports::outbound::FileSystem;
use crate::ports::outbound::ToolChecker;

pub fn check(
    fs: &dyn FileSystem,
    root: &Path,
    tree: &ProjectTree,
    tc: &dyn ToolChecker,
) -> Vec<CheckResult> {
    let mut results = shared::check(fs, root, tree, tc);
    results.extend(rs::check(tree, tc));
    results
}
