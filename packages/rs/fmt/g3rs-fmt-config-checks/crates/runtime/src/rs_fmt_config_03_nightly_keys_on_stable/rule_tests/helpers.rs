use guardrail3_check_types::G3CheckResult;
use test_support::{
    G3RsFmtRustPolicyState, G3RsFmtToolchainState, parsed_cargo, parsed_rustfmt, rustfmt_input,
};

use super::super::check;

pub(super) fn run_check(
    rustfmt_toml: &str,
    toolchain_state: G3RsFmtToolchainState,
) -> Vec<G3CheckResult> {
    let input = rustfmt_input(
        parsed_rustfmt(rustfmt_toml),
        parsed_cargo(
            r#"
[workspace.package]
edition = "2024"
"#,
        ),
        toolchain_state,
        G3RsFmtRustPolicyState::Missing,
    );
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

pub(super) fn parsed_toolchain(toolchain_toml: &str) -> G3RsFmtToolchainState {
    test_support::parsed_toolchain(toolchain_toml)
}
