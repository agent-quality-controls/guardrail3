use g3rs_toolchain_config_checks_types::G3RsToolchainConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

/// Run extracted rust-toolchain config checks.
///
/// Always runs the channel & components check. Runs the MSRV consistency
/// check only when `cargo_rel_path` and `cargo_toml` are both present.
#[must_use]
pub fn check(input: &G3RsToolchainConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    crate::rs_toolchain_config_01_channel_and_components::check(
        &input.toolchain_rel_path,
        &input.toolchain_toml,
        &mut results,
    );

    if let (Some(cargo_rel_path), Some(cargo_toml)) =
        (&input.cargo_rel_path, &input.cargo_toml)
    {
        crate::rs_toolchain_config_02_msrv_consistency::check(
            &input.toolchain_rel_path,
            &input.toolchain_toml,
            cargo_rel_path,
            cargo_toml,
            &mut results,
        );
    }

    results
}
