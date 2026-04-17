use g3rs_topology_types::G3RsTopologyFileTreeChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsTopologyFileTreeChecksInput) -> Vec<G3CheckResult> {
    let facts = crate::support::collect_facts(input);
    let mut results = Vec::new();

    for failure in &facts.input_failures {
        crate::rs_topology_07_required_inputs_fail_closed::check(failure, &mut results);
    }

    for input in &facts.nested_workspaces {
        crate::rs_topology_11_no_nested_workspaces::check(input, &mut results);
    }

    for input in &facts.membership_issues {
        crate::rs_topology_12_declared_workspace_members_only::check(input, &mut results);
    }

    for input in &facts.escaping_member_paths {
        crate::rs_topology_13_member_paths_must_not_escape_root::check(input, &mut results);
    }

    for file in &facts.illegal_family_files {
        crate::rs_topology_16_workspace_local_file_placement::check(file, &mut results);
    }

    results
}
