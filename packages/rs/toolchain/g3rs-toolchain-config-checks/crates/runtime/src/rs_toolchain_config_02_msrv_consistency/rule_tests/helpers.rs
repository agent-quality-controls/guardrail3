use cargo_toml_parser::parse as parse_cargo_toml;
use guardrail3_check_types::G3CheckResult;
use rust_toolchain_toml_parser::parse as parse_toolchain_toml;

use crate::rs_toolchain_config_02_msrv_consistency::rule;

pub(super) fn run_check(toolchain_toml: &str, cargo_toml: &str) -> Vec<G3CheckResult> {
    let toolchain_parsed =
        parse_toolchain_toml(toolchain_toml).expect("toolchain test fixture should parse");
    let cargo_parsed = parse_cargo_toml(cargo_toml).expect("cargo test fixture should parse");
    let mut results = Vec::new();
    rule::check(
        "rust-toolchain.toml",
        &toolchain_parsed,
        "Cargo.toml",
        &cargo_parsed,
        &mut results,
    );
    results
}
