use g3rs_cargo_types::G3RsCargoFileTreeChecksInput;
use guardrail3_check_types::G3CheckResult;

/// Run cargo file-tree checks against the input snapshot.
#[must_use]
pub fn check(input: &G3RsCargoFileTreeChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    for missing in &input.missing_members {
        crate::missing_member_cargo::check(missing, &mut results);
    }
    crate::missing_member_cargo::check_inventory(
        input.root.kind,
        &input.root.cargo_rel_path,
        input.root.members_parse_error,
        input.missing_members.is_empty(),
        &mut results,
    );

    for failure in &input.input_failures {
        crate::input_failures::check(failure, &mut results);
    }
    crate::input_failures::check_inventory(
        input.root.kind,
        &input.root.cargo_rel_path,
        input.root.rust_policy_rel_path.as_deref(),
        &input.input_failures,
        &mut results,
    );

    results
}

#[cfg(test)]
use g3rs_cargo_filetree_checks_assertions as _;

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
