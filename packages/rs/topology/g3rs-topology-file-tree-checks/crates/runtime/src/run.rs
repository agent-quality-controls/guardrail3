use g3rs_topology_types::G3RsTopologyFileTreeChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsTopologyFileTreeChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    for failure in &input.input_failures {
        crate::required_inputs_fail_closed::check(failure, &mut results);
    }

    for input in &input.nested_workspaces {
        crate::no_nested_workspaces::check(input, &mut results);
    }

    for input in &input.membership_issues {
        crate::declared_workspace_members_only::check(input, &mut results);
    }

    for input in &input.escaping_member_paths {
        crate::member_paths_must_not_escape_root::check(input, &mut results);
    }

    for file in &input.illegal_family_files {
        crate::workspace_local_file_placement::check(file, &mut results);
    }

    results
}

#[cfg(test)]
#[path = "run_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod run_tests;
