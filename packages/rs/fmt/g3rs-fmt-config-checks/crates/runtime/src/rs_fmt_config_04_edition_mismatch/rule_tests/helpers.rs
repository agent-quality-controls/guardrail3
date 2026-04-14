use g3rs_fmt_config_checks_types::{
    G3RsFmtCargoState, G3RsFmtConfigChecksInput, G3RsFmtRustPolicyState,
    G3RsFmtRustfmtConfigState, G3RsFmtToolchainState,
};
use guardrail3_check_types::G3CheckResult;
use rust_toolchain_toml_parser::parse as parse_toolchain_toml;
use rustfmt_toml_parser::parse as parse_rustfmt_toml;

use crate::rs_fmt_config_04_edition_mismatch::check;

pub(super) fn run_check(rustfmt_toml: &str, cargo_state: G3RsFmtCargoState) -> Vec<G3CheckResult> {
    let input = G3RsFmtConfigChecksInput {
        rustfmt_rel_path: "rustfmt.toml".to_owned(),
        rustfmt_state: G3RsFmtRustfmtConfigState::Parsed(
            parse_rustfmt_toml(rustfmt_toml).expect("rustfmt test fixture should parse"),
        ),
        rustfmt_explicit_keys: Vec::new(),
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo_state,
        toolchain_rel_path: "rust-toolchain.toml".to_owned(),
        toolchain_state: G3RsFmtToolchainState::Parsed(
            parse_toolchain_toml(
                r#"
[toolchain]
channel = "stable"
components = ["clippy", "rustfmt"]
"#,
            )
            .expect("toolchain fixture should parse"),
        ),
        rust_policy: G3RsFmtRustPolicyState::Missing,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}
