use g3_toolchain_content_checks_types::G3ToolchainContentChecksInput;
use guardrail3_check_types::GrdzCheckResult;

pub fn check(input: &G3ToolchainContentChecksInput) -> Vec<GrdzCheckResult> {
    let mut results = Vec::new();
    crate::rs_toolchain_02_channel_and_components::check(input, &mut results);
    crate::rs_toolchain_03_msrv_consistency::check(input, &mut results);
    results
}
