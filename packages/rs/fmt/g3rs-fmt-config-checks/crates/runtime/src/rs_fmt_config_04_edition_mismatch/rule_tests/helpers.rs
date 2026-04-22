use guardrail3_check_types::G3CheckResult;
use test_support::{
    G3RsFmtCargoState, G3RsFmtRustPolicyState, parsed_rustfmt, parsed_toolchain, rustfmt_input,
};

use super::super::check;

pub(super) fn run_check(rustfmt_toml: &str, cargo_state: G3RsFmtCargoState) -> Vec<G3CheckResult> {
    let input = rustfmt_input(
        parsed_rustfmt(rustfmt_toml),
        cargo_state,
        parsed_toolchain(
            r#"
[toolchain]
channel = "stable"
components = ["clippy", "rustfmt"]
"#,
        ),
        G3RsFmtRustPolicyState::Missing,
    );
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}
