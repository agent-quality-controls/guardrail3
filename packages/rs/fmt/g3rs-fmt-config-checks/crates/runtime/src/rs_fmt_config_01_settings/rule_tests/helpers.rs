use cargo_toml_parser::parse as parse_cargo_toml;
use guardrail3_check_types::G3CheckResult;
use rust_toolchain_toml_parser::parse as parse_toolchain_toml;
use rustfmt_toml_parser::parse as parse_rustfmt_toml;
use test_support::{
    G3RsFmtCargoState, G3RsFmtConfigChecksInput, G3RsFmtRustPolicyState, G3RsFmtRustfmtConfigState,
    G3RsFmtToolchainState,
};

use super::super::check;

pub(super) fn run_check(
    rustfmt_state: G3RsFmtRustfmtConfigState,
    cargo_toml: &str,
) -> Vec<G3CheckResult> {
    let input = G3RsFmtConfigChecksInput {
        rustfmt_rel_path: "rustfmt.toml".to_owned(),
        rustfmt_state,
        rustfmt_explicit_keys: Vec::new(),
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo_state: G3RsFmtCargoState::Parsed(
            parse_cargo_toml(cargo_toml).expect("cargo test fixture should parse"),
        ),
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

pub(super) fn parsed_rustfmt(rustfmt_toml: &str) -> G3RsFmtRustfmtConfigState {
    G3RsFmtRustfmtConfigState::Parsed(
        parse_rustfmt_toml(rustfmt_toml).expect("rustfmt test fixture should parse"),
    )
}
