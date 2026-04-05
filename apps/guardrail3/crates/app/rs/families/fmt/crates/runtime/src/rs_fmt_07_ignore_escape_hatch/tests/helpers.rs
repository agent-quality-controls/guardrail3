use super::super::{check};
use crate::inputs::RustfmtRootInput;
use guardrail3_domain_report::CheckResult;
pub(super) fn run_check(parsed: toml::Value) -> Vec<CheckResult> {
    run_check_with_escape_hatches(parsed, Vec::new())
}

pub(super) fn run_check_with_escape_hatches(
    parsed: toml::Value,
    escape_hatches: Vec<guardrail3_domain_config::types::EscapeHatchConfig>,
) -> Vec<CheckResult> {
    let parsed = parsed
        .try_into()
        .expect("RS-FMT-07 in-memory rustfmt TOML fixture should deserialize");
    let input = RustfmtRootInput {
        config_rel: Some("rustfmt.toml".to_owned()),
        parsed: Some(parsed),
        parse_error: None,
        escape_hatches,
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo: None,
        cargo_parse_error: None,
        toolchain_rel_path: "rust-toolchain.toml".to_owned(),
        toolchain: None,
        toolchain_parse_error: None,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}
