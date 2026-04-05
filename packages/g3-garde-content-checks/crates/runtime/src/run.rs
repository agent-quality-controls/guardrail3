use g3_garde_content_checks_types::{
    G3GardeClippyBanChecksInput, G3GardeDependencyCheckInput,
};
use guardrail3_check_types::G3CheckResult;

pub fn check_dependency_present(input: &G3GardeDependencyCheckInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::rs_garde_01_dependency_present::check(&input.cargo_rel_path, &input.cargo, &mut results);
    results
}

pub fn check_clippy_bans(input: &G3GardeClippyBanChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::rs_garde_02_core_method_bans::check(&input.clippy_rel_path, &input.clippy, &mut results);
    crate::rs_garde_03_extractor_type_bans::check(
        &input.clippy_rel_path,
        &input.clippy,
        &mut results,
    );
    crate::rs_garde_04_reqwest_json_ban::check(&input.clippy_rel_path, &input.clippy, &mut results);
    crate::rs_garde_06_additional_method_bans::check(
        &input.clippy_rel_path,
        &input.clippy,
        &mut results,
    );
    results
}
