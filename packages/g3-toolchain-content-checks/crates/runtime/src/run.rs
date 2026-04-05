use guardrail3_check_types::GrdzCheckResult;

use g3_toolchain_content_checks_types::{
    G3ToolchainChannelAndComponentsInput, G3ToolchainMsrvConsistencyInput,
};

pub fn check_channel_and_components(
    input: &G3ToolchainChannelAndComponentsInput,
) -> Vec<GrdzCheckResult> {
    let mut results = Vec::new();
    crate::rs_toolchain_02_channel_and_components::check(
        &input.toolchain_rel_path,
        &input.toolchain_toml,
        &mut results,
    );
    results
}

pub fn check_msrv_consistency(input: &G3ToolchainMsrvConsistencyInput) -> Vec<GrdzCheckResult> {
    let mut results = Vec::new();
    crate::rs_toolchain_03_msrv_consistency::check(
        &input.toolchain_rel_path,
        &input.toolchain_toml,
        &input.cargo_rel_path,
        &input.cargo_toml,
        &mut results,
    );
    results
}
