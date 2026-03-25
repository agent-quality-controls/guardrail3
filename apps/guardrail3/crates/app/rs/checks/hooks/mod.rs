pub use guardrail3_app_rs_family_hooks_rs as rs;
pub use guardrail3_app_rs_family_hooks_shared as shared;
pub(crate) mod shell;

use std::path::Path;

use crate::domain::report::CheckResult;
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_outbound_traits::FileSystem;
use guardrail3_outbound_traits::ToolChecker;

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
