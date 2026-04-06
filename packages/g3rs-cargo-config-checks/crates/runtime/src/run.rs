use g3rs_cargo_config_checks_types::G3RsCargoConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsCargoConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::rs_cargo_config_01_workspace_lints::check(&input.cargo_rel_path, &input.cargo, &mut results);
    crate::rs_cargo_config_02_lint_levels::check(&input.cargo_rel_path, &input.cargo, &mut results);
    crate::rs_cargo_config_03_workspace_metadata::check(&input.cargo_rel_path, &input.cargo, &mut results);
    crate::rs_cargo_config_04_priority_order::check(&input.cargo_rel_path, &input.cargo, &mut results);
    crate::rs_cargo_config_05_resolver::check(&input.cargo_rel_path, &input.cargo, &mut results);
    crate::rs_cargo_config_06_disallowed_macros_deny::check(
        &input.cargo_rel_path,
        &input.cargo,
        &mut results,
    );
    results
}
