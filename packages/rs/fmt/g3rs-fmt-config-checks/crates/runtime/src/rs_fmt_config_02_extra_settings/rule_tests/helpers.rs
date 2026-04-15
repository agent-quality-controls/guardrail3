use cargo_toml_parser::parse as parse_cargo_toml;
use test_support::{
    G3RsFmtCargoState, G3RsFmtConfigChecksInput, G3RsFmtRustPolicyState,
    G3RsFmtRustfmtConfigState, G3RsFmtToolchainState,
};
use guardrail3_check_types::G3CheckResult;
use rust_toolchain_toml_parser::parse as parse_toolchain_toml;
use rustfmt_toml_parser::parse as parse_rustfmt_toml;

use super::super::rule::check;

pub(super) fn run_check(rustfmt_toml: &str) -> Vec<G3CheckResult> {
    let input = G3RsFmtConfigChecksInput {
        rustfmt_rel_path: "rustfmt.toml".to_owned(),
        rustfmt_state: G3RsFmtRustfmtConfigState::Parsed(
            parse_rustfmt_toml(rustfmt_toml).expect("rustfmt test fixture should parse"),
        ),
        rustfmt_explicit_keys: explicit_keys(rustfmt_toml),
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo_state: G3RsFmtCargoState::Parsed(
            parse_cargo_toml(
                r#"
[workspace.package]
edition = "2024"
"#,
            )
            .expect("cargo fixture should parse"),
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

fn explicit_keys(rustfmt_toml: &str) -> Vec<String> {
    toml::from_str::<toml::Value>(rustfmt_toml)
        .expect("rustfmt test fixture should parse as toml")
        .as_table()
        .expect("rustfmt test fixture should be a toml table")
        .keys()
        .cloned()
        .collect()
}
