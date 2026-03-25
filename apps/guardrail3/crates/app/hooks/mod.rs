pub mod domain {
    pub use guardrail3_domain_report as report;
}

pub mod app {
    pub use guardrail3_app_core as core;

    pub mod rs {
        pub mod checks {
            pub mod hooks {
                use std::path::Path;

                use guardrail3_domain_project_tree::ProjectTree;
                use guardrail3_domain_report::CheckResult;
                use guardrail3_outbound_traits::{FileSystem, ToolChecker};

                pub fn check(
                    fs: &dyn FileSystem,
                    root: &Path,
                    tree: &ProjectTree,
                    tc: &dyn ToolChecker,
                ) -> Vec<CheckResult> {
                    let mut results =
                        guardrail3_app_rs_family_hooks_shared::check(fs, root, tree, tc);
                    results.extend(guardrail3_app_rs_family_hooks_rs::check(tree, tc));
                    results
                }
            }
        }
    }
}

mod deploy_checks;
mod hook_checks;
mod hook_script_checks;
mod tool_checks;
pub mod validate;
