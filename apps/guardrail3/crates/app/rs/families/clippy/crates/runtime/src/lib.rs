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

use guardrail3_app_rs_family_mapper::RsClippyRoute;
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::CheckResult;

use self::facts::collect;
use self::inputs::{ConfigClippyInput, CoveredRustUnitInput, UncoveredRustUnitInput};

pub use self::clippy_support::{EXPECTED_METHOD_BANS, EXPECTED_TYPE_BANS};

#[cfg(test)]
use guardrail3_app_rs_family_clippy_assertions as _;

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

#[cfg(test)]
mod test_support {
    use std::path::{Path, PathBuf};

    use guardrail3_adapters_outbound_fs::RealFileSystem;
    use guardrail3_app_core::project_walker::walk_project;
    use guardrail3_app_rs_family_mapper::FamilyMapper;
    use guardrail3_domain_project_tree::ProjectTree;
    use guardrail3_domain_report::CheckResult;
    use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};

    pub use ::test_support::*;
    use super::facts::{ClippyFacts, collect};
    use super::inputs::ConfigClippyInput;

    const GOLDEN_REL: &str = "../../../../../../../tests/fixtures/r_arch_01/golden";

    pub fn collected_facts(tree: &ProjectTree) -> ClippyFacts {
        collect(tree, &family_route(tree))
    }

    pub fn fixture_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(GOLDEN_REL)
    }

    pub fn copy_fixture() -> ::test_support::TempDir {
        ::test_support::copy_tree(&fixture_root())
    }

    pub fn run_family(root: &Path) -> Vec<CheckResult> {
        let tree = walk_project(&RealFileSystem, root);
        super::check(&tree, &family_route(&tree))
    }

    pub fn config_input<'a>(facts: &'a ClippyFacts, rel_path: &str) -> ConfigClippyInput<'a> {
        let config = facts
            .allowed_configs
            .iter()
            .find(|config| config.rel_path == rel_path)
            .expect("expected clippy config facts");
        ConfigClippyInput::new(config)
    }

    fn family_route(tree: &ProjectTree) -> super::RsClippyRoute {
        let scope = guardrail3_app_rs_placement::collect(tree);
        let selected = RustFamilySelection::new(std::collections::BTreeSet::from([
            RustValidateFamily::Clippy,
        ]));
        FamilyMapper::new(tree, &scope, None, &selected, None).map_rs_clippy()
    }
}
