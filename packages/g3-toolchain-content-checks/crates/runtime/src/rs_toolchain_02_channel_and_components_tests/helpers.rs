use g3_toolchain_content_checks_types::{G3CargoRustVersion, G3ToolchainContentChecksInput};
use guardrail3_check_types::GrdzCheckResult;

use crate::rs_toolchain_02_channel_and_components::check;

pub(super) fn run_check(toolchain_toml: &str) -> Vec<GrdzCheckResult> {
    let parsed = toml::from_str(toolchain_toml).expect("toolchain test fixture should parse");
    let input = G3ToolchainContentChecksInput {
        toolchain_rel_path: "rust-toolchain.toml".to_owned(),
        toolchain_toml: parsed,
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo_rust_version: G3CargoRustVersion::Version("1.85".to_owned()),
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}
