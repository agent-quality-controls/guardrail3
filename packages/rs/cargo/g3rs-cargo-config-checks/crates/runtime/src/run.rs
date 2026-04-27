use g3rs_cargo_types::G3RsCargoConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsCargoConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    if let Some(cargo) = cargo_toml_parser::document::typed(&input.root.cargo) {
        crate::workspace_lints::check(&input.root.cargo_rel_path, cargo, &mut results);
        crate::lint_levels::check(&input.root.cargo_rel_path, cargo, &mut results);
        crate::workspace_metadata::check(&input.root.cargo_rel_path, cargo, &mut results);
        crate::priority_order::check(&input.root.cargo_rel_path, cargo, &mut results);
        crate::resolver::check(&input.root.cargo_rel_path, cargo, &mut results);
        crate::disallowed_macros_deny::check(&input.root.cargo_rel_path, cargo, &mut results);
    }
    crate::approved_allow_inventory::check(&input.root, &mut results);
    crate::unapproved_allow_entries::check(&input.root, &mut results);
    crate::rust_version_policy::check(&input.root, &mut results);

    for member in &input.workspace_members {
        crate::workspace_lints_inherited::check(member, &mut results);
        crate::no_weakened_overrides::check(&input.root, member, &mut results);
        crate::member_edition_drift::check(&input.root, member, &mut results);
        crate::member_local_allows_forbidden::check(&input.root, member, &mut results);
    }
    results
}
