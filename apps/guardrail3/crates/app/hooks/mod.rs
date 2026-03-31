use std::path::Path;

use guardrail3_app_rs_family_mapper::RsProjectSurface;
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::CheckResult;
use guardrail3_outbound_traits::{FileSystem, ToolChecker};

pub fn check(
    fs: &dyn FileSystem,
    root: &Path,
    tree: &ProjectTree,
    tc: &dyn ToolChecker,
) -> Vec<CheckResult> {
    guardrail3_app_rs_family_hooks_shared::check(fs, root, &RsProjectSurface::from_tree(tree), tc)
}

mod deploy_checks;
mod hook_checks;
mod hook_script_checks;
mod tool_checks;
pub mod validate;
