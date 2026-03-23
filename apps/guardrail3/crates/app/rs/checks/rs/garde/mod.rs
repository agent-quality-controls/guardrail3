mod discover;
mod facts;
mod garde_support;
mod inputs;
mod parse;
mod rs_garde_01_dependency_present;
mod rs_garde_02_core_method_bans;
mod rs_garde_03_extractor_type_bans;
mod rs_garde_04_reqwest_json_ban;
mod rs_garde_05_struct_derive_validate;
mod rs_garde_06_additional_method_bans;
mod rs_garde_07_manual_deserialize_impl;
mod rs_garde_08_enum_derive_validate;
mod rs_garde_09_query_as_inventory;
mod rs_garde_10_input_failures;

#[cfg(test)]
mod test_support;

use crate::domain::project_tree::ProjectTree;
use crate::domain::report::CheckResult;

use self::facts::collect;
use self::inputs::{
    DerivedBoundaryTypeInput, GardeInputFailureInput, GardeRootInput, ManualDeserializeImplInput,
    QueryAsMacroInput,
};

pub fn check(tree: &ProjectTree) -> Vec<CheckResult> {
    let facts = collect(tree);
    let mut results = Vec::new();

    for failure in &facts.input_failures {
        rs_garde_10_input_failures::check(&GardeInputFailureInput::new(failure), &mut results);
    }

    for root in &facts.roots {
        let input = GardeRootInput::new(root);
        rs_garde_01_dependency_present::check(&input, &mut results);

        if !root.garde_dependency_present {
            continue;
        }

        rs_garde_02_core_method_bans::check(&input, &mut results);
        rs_garde_03_extractor_type_bans::check(&input, &mut results);
        rs_garde_04_reqwest_json_ban::check(&input, &mut results);
        rs_garde_06_additional_method_bans::check(&input, &mut results);
    }

    for target in &facts.struct_targets {
        rs_garde_05_struct_derive_validate::check(
            &DerivedBoundaryTypeInput::new(target),
            &mut results,
        );
    }

    for target in &facts.manual_deserialize_impls {
        rs_garde_07_manual_deserialize_impl::check(
            &ManualDeserializeImplInput::new(target),
            &mut results,
        );
    }

    for target in &facts.enum_targets {
        rs_garde_08_enum_derive_validate::check(
            &DerivedBoundaryTypeInput::new(target),
            &mut results,
        );
    }

    for macro_use in &facts.query_as_macros {
        rs_garde_09_query_as_inventory::check(&QueryAsMacroInput::new(macro_use), &mut results);
    }

    results
}
