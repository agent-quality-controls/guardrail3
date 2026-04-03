use guardrail3_app_rs_family_mapper::RsGardeRoute;
use guardrail3_app_rs_family_view::FamilyView;
use guardrail3_domain_report::CheckResult;

use crate::facts::collect;

pub fn check(surface: &FamilyView, route: &RsGardeRoute) -> Vec<CheckResult> {
    let tree = surface;
    let facts = collect(tree, route);
    let mut results = Vec::new();

    for failure in &facts.input_failures {
        crate::root_policy::rs_garde_10_input_failures::check(
            &crate::inputs::GardeInputFailureInput::new(failure),
            &mut results,
        );
    }

    for root in &facts.roots {
        if !root.garde_applicable {
            continue;
        }
        let input = crate::inputs::GardeRootInput::new(root);
        crate::root_policy::rs_garde_01_dependency_present::check(&input, &mut results);

        if !root.garde_dependency_present {
            continue;
        }

        crate::root_policy::rs_garde_02_core_method_bans::check(&input, &mut results);
        crate::root_policy::rs_garde_03_extractor_type_bans::check(&input, &mut results);
        crate::root_policy::rs_garde_04_reqwest_json_ban::check(&input, &mut results);
        crate::root_policy::rs_garde_06_additional_method_bans::check(&input, &mut results);
    }

    for target in &facts.struct_targets {
        if route
            .scoped_files()
            .is_some_and(|files| !files.contains(&target.rel_path))
        {
            continue;
        }
        crate::derive_checks::rs_garde_05_struct_derive_validate::check(
            &crate::inputs::DerivedBoundaryTypeInput::new(target),
            &mut results,
        );
    }

    for target in &facts.manual_deserialize_impls {
        if route
            .scoped_files()
            .is_some_and(|files| !files.contains(&target.rel_path))
        {
            continue;
        }
        crate::derive_checks::rs_garde_07_manual_deserialize_impl::check(
            &crate::inputs::ManualDeserializeImplInput::new(target),
            &mut results,
        );
    }

    for target in &facts.enum_targets {
        if route
            .scoped_files()
            .is_some_and(|files| !files.contains(&target.rel_path))
        {
            continue;
        }
        crate::derive_checks::rs_garde_08_enum_derive_validate::check(
            &crate::inputs::DerivedBoundaryTypeInput::new(target),
            &mut results,
        );
    }

    for macro_use in &facts.query_as_macros {
        if route
            .scoped_files()
            .is_some_and(|files| !files.contains(&macro_use.rel_path))
        {
            continue;
        }
        crate::inventory::rs_garde_09_query_as_inventory::check(
            &crate::inputs::QueryAsMacroInput::new(macro_use),
            &mut results,
        );
    }
    crate::inventory::rs_garde_09_query_as_inventory::check_count(
        facts.query_as_macros.iter().filter(|macro_use| {
            route
                .scoped_files()
                .is_none_or(|files| files.contains(&macro_use.rel_path))
        }),
        &mut results,
    );

    for field in &facts.boundary_fields {
        if route
            .scoped_files()
            .is_some_and(|files| !files.contains(&field.rel_path))
        {
            continue;
        }
        let input = crate::inputs::BoundaryFieldInput::new(field);
        crate::derive_checks::rs_garde_11_field_level_constraints::check(&input, &mut results);
        crate::derive_checks::rs_garde_12_nested_validation_dive::check(&input, &mut results);
        crate::derive_checks::rs_garde_13_context_validation_surface::check(&input, &mut results);
    }

    for site in &facts.guardrail_config_validation_sites {
        if route
            .scoped_files()
            .is_some_and(|files| !files.contains(&site.rel_path))
        {
            continue;
        }
        crate::derive_checks::rs_garde_14_guardrail_config_validate_call::check(
            &crate::inputs::GuardrailConfigValidationInput::new(site),
            &mut results,
        );
    }

    results
}
