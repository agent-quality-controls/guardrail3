use g3rs_garde_config_checks_types::G3RsGardeConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

/// Run extracted garde config checks.
///
/// Always runs the dependency-present check (Cargo.toml is required).
/// Runs clippy ban checks only when `clippy_rel_path` and `clippy` are
/// both present.
#[must_use]
pub fn check(input: &G3RsGardeConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    crate::rs_garde_config_01_dependency_present::check(
        &input.cargo_rel_path,
        &input.cargo,
        &mut results,
    );

    if let (Some(clippy_rel_path), Some(clippy)) = (&input.clippy_rel_path, &input.clippy) {
        crate::rs_garde_config_02_core_method_bans::check(clippy_rel_path, clippy, &mut results);
        crate::rs_garde_config_03_extractor_type_bans::check(
            clippy_rel_path,
            clippy,
            &mut results,
        );
        crate::rs_garde_config_04_reqwest_json_ban::check(clippy_rel_path, clippy, &mut results);
        crate::rs_garde_config_05_additional_method_bans::check(
            clippy_rel_path,
            clippy,
            &mut results,
        );
    }

    results
}
