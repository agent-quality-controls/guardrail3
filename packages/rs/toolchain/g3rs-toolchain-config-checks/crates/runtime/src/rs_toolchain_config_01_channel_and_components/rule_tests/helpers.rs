use guardrail3_check_types::G3CheckResult;
use rust_toolchain_toml_parser::parse as parse_toolchain_toml;

use crate::rs_toolchain_config_01_channel_and_components::check;

pub(super) fn run_check(toolchain_toml: &str) -> Vec<G3CheckResult> {
    let parsed = parse_toolchain_toml(toolchain_toml)
        .expect("toolchain test fixture should parse");
    let mut results = Vec::new();
    check("rust-toolchain.toml", &parsed, &mut results);
    results
}
