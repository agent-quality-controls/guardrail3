use super::super::{check};
use crate::facts::CargoEditionState;
use guardrail3_domain_report::CheckResult;
use crate::inputs::RustfmtRootInput;
pub(super) fn run_check(parsed: Option<toml::Value>) -> Vec<CheckResult> {
    let input = RustfmtRootInput {
        config_rel: Some("rustfmt.toml".to_owned()),
        parsed,
        escape_hatches: Vec::new(),
        cargo_edition: CargoEditionState::Present("2024".to_owned()),
        toolchain_channel: crate::facts::ToolchainChannelState::Present("stable".to_owned()),
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}
pub(super) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}
