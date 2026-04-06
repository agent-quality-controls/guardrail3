use g3rs_garde_ast_checks_types::G3RsGardeAstChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsGardeAstChecksInput) -> Vec<G3CheckResult> {
    let analysis = crate::support::analyze_input(input);
    let mut results = Vec::new();

    for target in &analysis.struct_targets {
        crate::rs_garde_ast_01_struct_derive_validate::check(target, &mut results);
    }
    for target in &analysis.manual_deserialize_impls {
        crate::rs_garde_ast_02_manual_deserialize_impl::check(target, &mut results);
    }
    for target in &analysis.enum_targets {
        crate::rs_garde_ast_03_enum_derive_validate::check(target, &mut results);
    }
    for macro_use in &analysis.query_as_macros {
        crate::rs_garde_ast_04_query_as_inventory::check(macro_use, &mut results);
    }
    crate::rs_garde_ast_04_query_as_inventory::check_count(&analysis.query_as_macros, &mut results);
    for field in &analysis.boundary_fields {
        crate::rs_garde_ast_05_field_level_constraints::check(field, &mut results);
        crate::rs_garde_ast_06_nested_validation_dive::check(field, &mut results);
        crate::rs_garde_ast_07_context_validation_surface::check(field, &mut results);
    }
    for site in &analysis.guardrail_config_validation_sites {
        crate::rs_garde_ast_08_guardrail_config_validate_call::check(site, &mut results);
    }

    results
}
