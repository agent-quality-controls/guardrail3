mod clippy_support;
mod facts;
mod inputs;
mod rs_clippy_01_coverage;
mod rs_clippy_02_max_struct_bools;
mod rs_clippy_03_max_fn_params_bools;
mod rs_clippy_04_missing_method_ban;
mod rs_clippy_05_missing_type_ban;
mod rs_clippy_06_extra_method_ban;
mod rs_clippy_07_extra_type_ban;
mod rs_clippy_08_reason_quality;
mod rs_clippy_09_too_many_lines_threshold;
mod rs_clippy_10_too_many_arguments_threshold;
mod rs_clippy_11_excessive_nesting_threshold;
mod rs_clippy_12_allowed_placement;
mod rs_clippy_13_local_policy_root_baseline;
mod rs_clippy_14_library_global_state;
mod rs_clippy_15_trivial_reason;
mod rs_clippy_16_avoid_breaking_exported_api;
mod rs_clippy_17_test_relaxations;
mod rs_clippy_18_duplicate_bans;
mod rs_clippy_19_unknown_keys;
mod rs_clippy_20_macro_bans;
mod rs_clippy_21_cognitive_complexity_threshold;
mod rs_clippy_22_type_complexity_threshold;

#[cfg(test)]
mod test_support;

use guardrail3_app_rs_family_mapper::RsClippyRoute;
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::CheckResult;

use self::facts::collect;
use self::inputs::{ConfigClippyInput, CoveredRustUnitInput, UncoveredRustUnitInput};

pub use self::clippy_support::{EXPECTED_METHOD_BANS, EXPECTED_TYPE_BANS};

pub fn check(tree: &ProjectTree, route: &RsClippyRoute) -> Vec<CheckResult> {
    let facts = collect(tree, route);
    let mut results = Vec::new();

    for covered in &facts.covered_units {
        rs_clippy_01_coverage::check_covered(&CoveredRustUnitInput::new(covered), &mut results);
    }

    for uncovered in &facts.uncovered_units {
        rs_clippy_01_coverage::check_uncovered(
            &UncoveredRustUnitInput::new(uncovered),
            &mut results,
        );
    }

    for forbidden in &facts.forbidden_configs {
        rs_clippy_12_allowed_placement::check(forbidden, &mut results);
    }

    for input in ConfigClippyInput::from_facts(&facts) {
        rs_clippy_02_max_struct_bools::check(&input, &mut results);
        rs_clippy_03_max_fn_params_bools::check(&input, &mut results);
        rs_clippy_04_missing_method_ban::check(&input, &mut results);
        rs_clippy_05_missing_type_ban::check(&input, &mut results);
        rs_clippy_06_extra_method_ban::check(&input, &mut results);
        rs_clippy_07_extra_type_ban::check(&input, &mut results);
        rs_clippy_08_reason_quality::check(&input, &mut results);
        rs_clippy_09_too_many_lines_threshold::check(&input, &mut results);
        rs_clippy_10_too_many_arguments_threshold::check(&input, &mut results);
        rs_clippy_11_excessive_nesting_threshold::check(&input, &mut results);
        rs_clippy_13_local_policy_root_baseline::check(&input, &mut results);
        rs_clippy_14_library_global_state::check(&input, &mut results);
        rs_clippy_15_trivial_reason::check(&input, &mut results);
        rs_clippy_16_avoid_breaking_exported_api::check(&input, &mut results);
        rs_clippy_17_test_relaxations::check(&input, &mut results);
        rs_clippy_18_duplicate_bans::check(&input, &mut results);
        rs_clippy_19_unknown_keys::check(&input, &mut results);
        rs_clippy_20_macro_bans::check(&input, &mut results);
        rs_clippy_21_cognitive_complexity_threshold::check(&input, &mut results);
        rs_clippy_22_type_complexity_threshold::check(&input, &mut results);
    }

    results
}
