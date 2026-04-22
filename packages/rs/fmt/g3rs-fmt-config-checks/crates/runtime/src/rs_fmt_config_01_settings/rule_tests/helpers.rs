use guardrail3_check_types::G3CheckResult;
use test_support::{
    G3RsFmtRustPolicyState, G3RsFmtRustfmtConfigState, parsed_cargo, parsed_toolchain,
    rustfmt_input,
};

use super::super::check;

pub(super) fn run_check(
    rustfmt_state: G3RsFmtRustfmtConfigState,
    cargo_toml: &str,
) -> Vec<G3CheckResult> {
    let input = rustfmt_input(
        rustfmt_state,
        parsed_cargo(cargo_toml),
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

pub(super) fn parsed_rustfmt(rustfmt_toml: &str) -> G3RsFmtRustfmtConfigState {
    test_support::parsed_rustfmt(rustfmt_toml)
}
