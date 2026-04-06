use g3rs_garde_config_checks_types::{
    G3RsGardeConfigClippyBanChecksInput, G3RsGardeConfigDependencyCheckInput,
};
use guardrail3_check_types::G3CheckResult;

pub fn check_dependency_present(input: &G3RsGardeConfigDependencyCheckInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::rs_garde_config_01_dependency_present::check(&input.cargo_rel_path, &input.cargo, &mut results);
    results
}

pub fn check_clippy_bans(input: &G3RsGardeConfigClippyBanChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::rs_garde_config_02_core_method_bans::check(&input.clippy_rel_path, &input.clippy, &mut results);
    crate::rs_garde_config_03_extractor_type_bans::check(
        &input.clippy_rel_path,
        &input.clippy,
        &mut results,
    );
    crate::rs_garde_config_04_reqwest_json_ban::check(&input.clippy_rel_path, &input.clippy, &mut results);
    crate::rs_garde_config_05_additional_method_bans::check(
        &input.clippy_rel_path,
        &input.clippy,
        &mut results,
    );
    results
}
