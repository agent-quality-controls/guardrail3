use g3rs_garde_types::{G3RsGardeApplicability, G3RsGardeSourceChecksInput};
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3RsGardeSourceChecksInput) -> Vec<G3CheckResult> {
    if input.applicability == G3RsGardeApplicability::Inactive {
        return Vec::new();
    }

    let mut results = Vec::new();

    for failure in &input.input_failures {
        crate::input_failures::check(failure, &mut results);
    }
    for target in &input.struct_targets {
        crate::struct_derive_validate::check(target, &mut results);
    }
    for target in &input.manual_deserialize_impls {
        crate::manual_deserialize_impl::check(target, &mut results);
    }
    for target in &input.enum_targets {
        crate::enum_derive_validate::check(target, &mut results);
    }
    for macro_use in &input.query_as_macros {
        crate::query_as_inventory::check(macro_use, &mut results);
    }
    crate::query_as_inventory::check_count(&input.query_as_macros, &mut results);
    for field in &input.boundary_fields {
        crate::field_level_constraints::check(field, &mut results);
        crate::nested_validation_dive::check(field, &mut results);
        crate::context_validation_surface::check(field, &mut results);
    }

    results
}
