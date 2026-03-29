mod facts;
mod inputs;
mod rs_deps_01_cargo_deny_installed;
mod rs_deps_02_cargo_machete_installed;
mod rs_deps_03_cargo_dupes_installed;
mod rs_deps_04_gitleaks_installed;
mod rs_deps_05_dependencies_allowlisted;
mod rs_deps_06_build_dependencies_allowlisted;
mod rs_deps_07_dev_dependencies_allowlisted;
mod rs_deps_08_library_allowlist_present;
mod rs_deps_09_cargo_lock_present;
mod rs_deps_10_gitignore_not_ignoring_cargo_lock;
mod rs_deps_11_input_failures;

use guardrail3_app_rs_family_mapper::RsDepsRoute;
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::CheckResult;
use guardrail3_outbound_traits::ToolChecker;

use self::facts::{DepsFacts, collect};
use self::inputs::{
    AllowlistCoverageDepsInput, DependencyEntryDepsInput, InputFailureDepsInput, LockfileDepsInput,
    ToolDepsInput,
};

pub fn check(tree: &ProjectTree, route: &RsDepsRoute, tc: &dyn ToolChecker) -> Vec<CheckResult> {
    let facts = collect(tree, route, tc);
    run_with_facts(&facts)
}

pub fn run_with_facts(facts: &DepsFacts) -> Vec<CheckResult> {
    let mut results = Vec::new();

    for tool in &facts.tools {
        let input = ToolDepsInput::new(tool);
        rs_deps_01_cargo_deny_installed::check(&input, &mut results);
        rs_deps_02_cargo_machete_installed::check(&input, &mut results);
        rs_deps_03_cargo_dupes_installed::check(&input, &mut results);
        rs_deps_04_gitleaks_installed::check(&input, &mut results);
    }

    for entry in &facts.dependency_entries {
        let input = DependencyEntryDepsInput::new(entry);
        rs_deps_05_dependencies_allowlisted::check(&input, &mut results);
        rs_deps_06_build_dependencies_allowlisted::check(&input, &mut results);
        rs_deps_07_dev_dependencies_allowlisted::check(&input, &mut results);
    }

    for coverage in &facts.allowlist_coverage {
        let input = AllowlistCoverageDepsInput::new(coverage);
        rs_deps_08_library_allowlist_present::check(&input, &mut results);
    }

    for lockfile in &facts.lockfiles {
        let input = LockfileDepsInput::new(lockfile);
        rs_deps_09_cargo_lock_present::check(&input, &mut results);
        rs_deps_10_gitignore_not_ignoring_cargo_lock::check(&input, &mut results);
    }

    for failure in &facts.input_failures {
        let input = InputFailureDepsInput::new(failure);
        rs_deps_11_input_failures::check(&input, &mut results);
    }

    results
}
