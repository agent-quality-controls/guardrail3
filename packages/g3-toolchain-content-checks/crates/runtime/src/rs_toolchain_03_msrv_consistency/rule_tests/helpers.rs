use cargo_toml_parser::parse as parse_cargo_toml;
use g3_toolchain_content_checks_types::G3ToolchainMsrvConsistencyInput;
use guardrail3_check_types::G3CheckResult;
use rust_toolchain_toml_parser::parse as parse_toolchain_toml;

use crate::rs_toolchain_03_msrv_consistency::check;

pub(super) fn run_check(toolchain_toml: &str, cargo_toml: &str) -> Vec<G3CheckResult> {
    let toolchain_toml = parse_toolchain_toml(toolchain_toml)
        .expect("toolchain test fixture should parse");
    let cargo_toml = parse_cargo_toml(cargo_toml).expect("cargo test fixture should parse");
    let input = G3ToolchainMsrvConsistencyInput {
        toolchain_rel_path: "rust-toolchain.toml".to_owned(),
        toolchain_toml,
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo_toml,
    };
    let mut results = Vec::new();
    check(
        &input.toolchain_rel_path,
        &input.toolchain_toml,
        &input.cargo_rel_path,
        &input.cargo_toml,
        &mut results,
    );
    results
}
