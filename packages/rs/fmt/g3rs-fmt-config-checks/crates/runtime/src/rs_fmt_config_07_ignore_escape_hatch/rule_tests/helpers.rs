use guardrail3_check_types::G3CheckResult;
use test_support::{
    G3RsFmtCargoState, G3RsFmtRustPolicyState, G3RsFmtToolchainState, G3RsFmtWaiver,
    parsed_rustfmt, rustfmt_input,
};

use super::super::check;

pub(super) fn run_check(rustfmt_toml: &str, waivers: Vec<G3RsFmtWaiver>) -> Vec<G3CheckResult> {
    let input = rustfmt_input(
        parsed_rustfmt(rustfmt_toml),
        G3RsFmtCargoState::Missing,
        G3RsFmtToolchainState::Missing,
        G3RsFmtRustPolicyState::Parsed {
            rel_path: "guardrail3-rs.toml".to_owned(),
            waivers,
        },
    );
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

pub(super) fn waiver(reason: &str) -> G3RsFmtWaiver {
    G3RsFmtWaiver {
        rule: "RS-FMT-CONFIG-07".to_owned(),
        file: "rustfmt.toml".to_owned(),
        selector: "ignore".to_owned(),
        reason: reason.to_owned(),
    }
}
