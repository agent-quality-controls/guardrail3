mod derive_checks;
mod discover;
mod facts;
mod garde_support;
mod inputs;
mod inventory;
mod parse;
mod root_policy;

use glob as _;
use guardrail3_app_core as _;
use guardrail3_app_rs_family_mapper::RsGardeRoute;
use guardrail3_app_rs_family_view::FamilyView;
use guardrail3_domain_config as _;
use guardrail3_domain_modules as _;
use guardrail3_domain_report::CheckResult;
use guardrail3_outbound_traits as _;
use semver as _;
use serde_yaml as _;

use self::facts::collect;

pub fn check(surface: &FamilyView, route: &RsGardeRoute) -> Vec<CheckResult> {
    let tree = surface;
    let facts = collect(tree, route);
    let mut results = Vec::new();

    for failure in &facts.input_failures {
        root_policy::rs_garde_10_input_failures::check(
            &inputs::GardeInputFailureInput::new(failure),
            &mut results,
        );
    }

    for root in &facts.roots {
        if !root.garde_applicable {
            continue;
        }
        let input = inputs::GardeRootInput::new(root);
        root_policy::rs_garde_01_dependency_present::check(&input, &mut results);

        if !root.garde_dependency_present {
            continue;
        }

        root_policy::rs_garde_02_core_method_bans::check(&input, &mut results);
        root_policy::rs_garde_03_extractor_type_bans::check(&input, &mut results);
        root_policy::rs_garde_04_reqwest_json_ban::check(&input, &mut results);
        root_policy::rs_garde_06_additional_method_bans::check(&input, &mut results);
    }

    for target in &facts.struct_targets {
        if route
            .scoped_files()
            .is_some_and(|files| !files.contains(&target.rel_path))
        {
            continue;
        }
        derive_checks::rs_garde_05_struct_derive_validate::check(
            &inputs::DerivedBoundaryTypeInput::new(target),
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
        derive_checks::rs_garde_07_manual_deserialize_impl::check(
            &inputs::ManualDeserializeImplInput::new(target),
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
        derive_checks::rs_garde_08_enum_derive_validate::check(
            &inputs::DerivedBoundaryTypeInput::new(target),
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
        inventory::rs_garde_09_query_as_inventory::check(
            &inputs::QueryAsMacroInput::new(macro_use),
            &mut results,
        );
    }
    inventory::rs_garde_09_query_as_inventory::check_count(
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
        let input = inputs::BoundaryFieldInput::new(field);
        derive_checks::rs_garde_11_field_level_constraints::check(&input, &mut results);
        derive_checks::rs_garde_12_nested_validation_dive::check(&input, &mut results);
        derive_checks::rs_garde_13_context_validation_surface::check(&input, &mut results);
    }

    for site in &facts.guardrail_config_validation_sites {
        if route
            .scoped_files()
            .is_some_and(|files| !files.contains(&site.rel_path))
        {
            continue;
        }
        derive_checks::rs_garde_14_guardrail_config_validate_call::check(
            &inputs::GuardrailConfigValidationInput::new(site),
            &mut results,
        );
    }

    results
}

#[cfg(test)]
pub(crate) fn check_test_tree(
    tree: &guardrail3_app_rs_family_view::FamilyView,
    route: &RsGardeRoute,
) -> Vec<CheckResult> {
    check(tree, route)
}
