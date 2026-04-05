use g3_cargo_content_checks_types::G3CargoContentChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3CargoContentChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::rs_cargo_01_workspace_lints::check(&input.cargo_rel_path, &input.cargo, &mut results);
    crate::rs_cargo_02_lint_levels::check(&input.cargo_rel_path, &input.cargo, &mut results);
    crate::rs_cargo_05_workspace_metadata::check(&input.cargo_rel_path, &input.cargo, &mut results);
    crate::rs_cargo_07_priority_order::check(&input.cargo_rel_path, &input.cargo, &mut results);
    crate::rs_cargo_08_resolver::check(&input.cargo_rel_path, &input.cargo, &mut results);
    crate::rs_cargo_11_disallowed_macros_deny::check(
        &input.cargo_rel_path,
        &input.cargo,
        &mut results,
    );
    results
}
