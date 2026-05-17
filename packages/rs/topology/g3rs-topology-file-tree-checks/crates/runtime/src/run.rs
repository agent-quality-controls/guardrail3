use g3rs_topology_types::G3RsTopologyFileTreeChecksInput;
use guardrail3_check_types::G3CheckResult;

/// Runs all topology file-tree rules against `input` and returns aggregated results.
#[must_use]
pub fn check(input: &G3RsTopologyFileTreeChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    for failure in &input.input_failures {
        crate::required_inputs_fail_closed::check(failure, &mut results);
    }

    for nested in &input.nested_workspaces {
        crate::no_nested_workspaces::check(nested, &mut results);
    }

    for nested in &input.nested_guardrail3_rs_tomls {
        crate::no_nested_guardrail3_rs_toml::check(nested, &mut results);
    }

    for issue in &input.membership_issues {
        crate::declared_workspace_members_only::check(issue, &mut results);
    }

    for escaping in &input.escaping_member_paths {
        crate::member_paths_must_not_escape_root::check(escaping, &mut results);
    }

    for file in &input.illegal_family_files {
        crate::workspace_local_file_placement::check(file, &mut results);
    }

    results
}
