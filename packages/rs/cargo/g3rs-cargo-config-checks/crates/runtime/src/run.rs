use g3rs_cargo_config_checks_types::G3RsCargoConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsCargoConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::rs_cargo_config_01_workspace_lints::check(
        &input.root.cargo_rel_path,
        &input.root.cargo,
        &mut results,
    );
    crate::rs_cargo_config_02_lint_levels::check(
        &input.root.cargo_rel_path,
        &input.root.cargo,
        &mut results,
    );
    crate::rs_cargo_config_03_workspace_metadata::check(
        &input.root.cargo_rel_path,
        &input.root.cargo,
        &mut results,
    );
    crate::rs_cargo_config_04_priority_order::check(
        &input.root.cargo_rel_path,
        &input.root.cargo,
        &mut results,
    );
    crate::rs_cargo_config_05_resolver::check(
        &input.root.cargo_rel_path,
        &input.root.cargo,
        &mut results,
    );
    crate::rs_cargo_config_06_disallowed_macros_deny::check(
        &input.root.cargo_rel_path,
        &input.root.cargo,
        &mut results,
    );
    crate::rs_cargo_config_07_approved_allow_inventory::check(&input.root, &mut results);
    crate::rs_cargo_config_11_unapproved_allow_entries::check(&input.root, &mut results);
    crate::rs_cargo_config_13_rust_version_policy::check(&input.root, &mut results);

    for member in &input.workspace_members {
        crate::rs_cargo_config_08_workspace_lints_inherited::check(member, &mut results);
        crate::rs_cargo_config_09_no_weakened_overrides::check(&input.root, member, &mut results);
        crate::rs_cargo_config_10_member_edition_drift::check(&input.root, member, &mut results);
        crate::rs_cargo_config_12_member_local_allows_forbidden::check(
            &input.root,
            member,
            &mut results,
        );
    }
    results
}
