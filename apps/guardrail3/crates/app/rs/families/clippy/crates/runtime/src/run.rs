use guardrail3_app_rs_family_mapper::RsClippyRoute;
use guardrail3_app_rs_family_view::FamilyView;
use guardrail3_domain_report::CheckResult;

use crate::facts::collect;
use crate::inputs::{
    CargoConfigOverrideInput, CargoRootFailureInput, ConfigClippyInput, CoveredRustUnitInput,
    PolicyContextFailureInput, UncoveredRustUnitInput,
};

pub fn check(surface: &FamilyView, route: &RsClippyRoute) -> Vec<CheckResult> {
    let tree = surface;
    let facts = collect(tree, route);
    let has_routed_roots = !route.roots().is_empty();
    let mut results = Vec::new();

    for failure in &facts.cargo_root_failures {
        crate::rs_clippy_01_coverage::check_root_failure(
            &CargoRootFailureInput::new(failure),
            &mut results,
        );
    }

    for covered in &facts.covered_units {
        crate::rs_clippy_01_coverage::check_covered(&CoveredRustUnitInput::new(covered), &mut results);
    }

    for allowed in &facts.allowed_configs {
        crate::rs_clippy_12_allowed_placement::check_allowed(allowed, &mut results);
    }

    for uncovered in &facts.uncovered_units {
        crate::rs_clippy_01_coverage::check_uncovered(
            &UncoveredRustUnitInput::new(uncovered),
            &mut results,
        );
    }

    for forbidden in &facts.forbidden_configs {
        crate::rs_clippy_12_allowed_placement::check(forbidden, &mut results);
    }

    if has_routed_roots {
        if let Some(parse_error) = facts.policy_context_parse_error.as_deref() {
            crate::rs_clippy_23_policy_context_parseable::check(
                &PolicyContextFailureInput::new(parse_error),
                &mut results,
            );
        } else if tree.file_exists("guardrail3.toml") {
            crate::rs_clippy_23_policy_context_parseable::check_parseable(&mut results);
        }
    }

    if has_routed_roots || !facts.cargo_config_overrides.is_empty() {
        if facts.cargo_config_overrides.is_empty() {
            crate::rs_clippy_24_forbid_clippy_conf_dir_override::check_clean(&mut results);
        } else {
            for override_facts in &facts.cargo_config_overrides {
                crate::rs_clippy_24_forbid_clippy_conf_dir_override::check(
                    &CargoConfigOverrideInput::new(override_facts),
                    &mut results,
                );
            }
        }
    }

    for input in ConfigClippyInput::from_facts(&facts) {
        crate::rs_clippy_25_config_parseable::check(&input, &mut results);
        crate::rs_clippy_02_max_struct_bools::check(&input, &mut results);
        crate::rs_clippy_03_max_fn_params_bools::check(&input, &mut results);
        crate::rs_clippy_04_missing_method_ban::check(&input, &mut results);
        crate::rs_clippy_05_missing_type_ban::check(&input, &mut results);
        crate::rs_clippy_06_extra_method_ban::check(&input, &mut results);
        crate::rs_clippy_07_extra_type_ban::check(&input, &mut results);
        crate::rs_clippy_08_reason_quality::check(&input, &mut results);
        crate::rs_clippy_09_too_many_lines_threshold::check(&input, &mut results);
        crate::rs_clippy_10_too_many_arguments_threshold::check(&input, &mut results);
        crate::rs_clippy_11_excessive_nesting_threshold::check(&input, &mut results);
        crate::rs_clippy_13_local_policy_root_baseline::check(&input, &mut results);
        crate::rs_clippy_14_library_global_state::check(&input, &mut results);
        crate::rs_clippy_15_trivial_reason::check(&input, &mut results);
        crate::rs_clippy_16_avoid_breaking_exported_api::check(&input, &mut results);
        crate::rs_clippy_17_test_relaxations::check(&input, &mut results);
        crate::rs_clippy_18_duplicate_bans::check(&input, &mut results);
        crate::rs_clippy_19_unknown_keys::check(&input, &mut results);
        crate::rs_clippy_20_macro_bans::check(&input, &mut results);
        crate::rs_clippy_21_cognitive_complexity_threshold::check(&input, &mut results);
        crate::rs_clippy_22_type_complexity_threshold::check(&input, &mut results);
    }

    results
}
