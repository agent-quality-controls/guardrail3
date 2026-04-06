use guardrail3_check_types::G3CheckResult;

use g3rs_toolchain_config_checks_types::{
    G3RsToolchainConfigChannelComponentsInput, G3RsToolchainConfigMsrvConsistencyInput,
};

pub fn check_channel_and_components(
    input: &G3RsToolchainConfigChannelComponentsInput,
) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::rs_toolchain_config_01_channel_and_components::check(
        &input.toolchain_rel_path,
        &input.toolchain_toml,
        &mut results,
    );
    results
}

pub fn check_msrv_consistency(input: &G3RsToolchainConfigMsrvConsistencyInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::rs_toolchain_config_02_msrv_consistency::check(
        &input.toolchain_rel_path,
        &input.toolchain_toml,
        &input.cargo_rel_path,
        &input.cargo_toml,
        &mut results,
    );
    results
}
