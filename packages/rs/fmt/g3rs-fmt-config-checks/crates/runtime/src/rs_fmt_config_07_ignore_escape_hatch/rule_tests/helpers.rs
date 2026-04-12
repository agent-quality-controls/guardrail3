use g3rs_fmt_config_checks_types::{
    G3RsFmtCargoState, G3RsFmtConfigChecksInput, G3RsFmtEscapeHatch, G3RsFmtRustfmtConfigState,
    G3RsFmtToolchainState,
};
use guardrail3_check_types::G3CheckResult;
use rustfmt_toml_parser::parse as parse_rustfmt_toml;

use crate::rs_fmt_config_07_ignore_escape_hatch::check;

pub(super) fn run_check(
    rustfmt_toml: &str,
    escape_hatches: Vec<G3RsFmtEscapeHatch>,
) -> Vec<G3CheckResult> {
    let input = G3RsFmtConfigChecksInput {
        rustfmt_rel_path: "rustfmt.toml".to_owned(),
        rustfmt_state: G3RsFmtRustfmtConfigState::Parsed(
            parse_rustfmt_toml(rustfmt_toml).expect("rustfmt fixture should parse"),
        ),
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo_state: G3RsFmtCargoState::Missing,
        toolchain_rel_path: "rust-toolchain.toml".to_owned(),
        toolchain_state: G3RsFmtToolchainState::Missing,
        escape_hatches,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

pub(super) fn escape_hatch(reason: &str) -> G3RsFmtEscapeHatch {
    G3RsFmtEscapeHatch {
        family: "fmt".to_owned(),
        file: "rustfmt.toml".to_owned(),
        kind: "ignore".to_owned(),
        selector: "ignore".to_owned(),
        reason: reason.to_owned(),
    }
}
