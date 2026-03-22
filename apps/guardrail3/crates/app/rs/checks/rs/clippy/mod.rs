mod clippy_support;
mod facts;
mod inputs;
mod rs_clippy_01_exists;
mod rs_clippy_04_missing_method_ban;
mod rs_clippy_05_missing_type_ban;
mod rs_clippy_06_extra_method_ban;
mod rs_clippy_07_extra_type_ban;
mod rs_clippy_08_reason_quality;
mod rs_clippy_12_per_crate_inheritance;
mod rs_clippy_13_per_crate_root_bans;
mod rs_clippy_14_library_global_state;
mod rs_clippy_15_trivial_reason;
mod rs_clippy_16_avoid_breaking_exported_api;
mod rs_clippy_17_test_relaxations;
mod rs_clippy_18_duplicate_bans;
mod rs_clippy_19_unknown_keys;
mod rs_clippy_20_macro_bans;
mod rs_clippy_thresholds;

use crate::domain::project_tree::ProjectTree;
use crate::domain::report::CheckResult;

use self::facts::collect;
use self::inputs::{ConfigClippyInput, CoveredRustUnitInput, UncoveredRustUnitInput};

pub fn check(tree: &ProjectTree) -> Vec<CheckResult> {
    let facts = collect(tree);
    let mut results = Vec::new();

    for covered in &facts.covered_units {
        rs_clippy_01_exists::check_covered(&CoveredRustUnitInput::new(covered), &mut results);
    }

    for uncovered in &facts.uncovered_units {
        rs_clippy_01_exists::check_uncovered(&UncoveredRustUnitInput::new(uncovered), &mut results);
    }

    for forbidden in &facts.forbidden_configs {
        rs_clippy_12_per_crate_inheritance::check(forbidden, &mut results);
    }

    for input in ConfigClippyInput::from_facts(&facts) {
        rs_clippy_thresholds::check(&input, &mut results);
        rs_clippy_04_missing_method_ban::check(&input, &mut results);
        rs_clippy_05_missing_type_ban::check(&input, &mut results);
        rs_clippy_06_extra_method_ban::check(&input, &mut results);
        rs_clippy_07_extra_type_ban::check(&input, &mut results);
        rs_clippy_08_reason_quality::check(&input, &mut results);
        rs_clippy_13_per_crate_root_bans::check(&input, &mut results);
        rs_clippy_14_library_global_state::check(&input, &mut results);
        rs_clippy_15_trivial_reason::check(&input, &mut results);
        rs_clippy_16_avoid_breaking_exported_api::check(&input, &mut results);
        rs_clippy_17_test_relaxations::check(&input, &mut results);
        rs_clippy_18_duplicate_bans::check(&input, &mut results);
        rs_clippy_19_unknown_keys::check(&input, &mut results);
        rs_clippy_20_macro_bans::check(&input, &mut results);
    }

    results
}

#[cfg(test)]
#[path = "clippy_tests.rs"]
mod tests;
