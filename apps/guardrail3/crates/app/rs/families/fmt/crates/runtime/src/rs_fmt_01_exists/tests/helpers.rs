use super::super::{check};
use crate::inputs::RustfmtRootInput;
use guardrail3_domain_report::CheckResult;

pub(super) fn run_check(config_rel: Option<&str>) -> Vec<CheckResult> {
    let input = RustfmtRootInput {
        config_rel: config_rel.map(str::to_owned),
        parsed: None,
        parse_error: None,
        escape_hatches: Vec::new(),
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
pub(super) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}
