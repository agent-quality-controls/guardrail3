use g3rs_fmt_config_checks_types::{
    G3RsFmtCargoState, G3RsFmtConfigChecksInput, G3RsFmtRustPolicyState,
    G3RsFmtRustfmtConfigState, G3RsFmtToolchainState, G3RsFmtWaiver,
};
use guardrail3_check_types::G3CheckResult;
use rustfmt_toml_parser::parse as parse_rustfmt_toml;

use crate::rs_fmt_config_07_ignore_escape_hatch::check;

pub(super) fn run_check(
    rustfmt_toml: &str,
    waivers: Vec<G3RsFmtWaiver>,
) -> Vec<G3CheckResult> {
    let input = G3RsFmtConfigChecksInput {
        rustfmt_rel_path: "rustfmt.toml".to_owned(),
        rustfmt_state: G3RsFmtRustfmtConfigState::Parsed(
            parse_rustfmt_toml(rustfmt_toml).expect("rustfmt fixture should parse"),
        ),
        rustfmt_explicit_keys: Vec::new(),
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo_state: G3RsFmtCargoState::Missing,
        toolchain_rel_path: "rust-toolchain.toml".to_owned(),
        toolchain_state: G3RsFmtToolchainState::Missing,
        rust_policy: G3RsFmtRustPolicyState::Parsed {
            rel_path: "guardrail3-rs.toml".to_owned(),
            waivers,
        },
    };
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
