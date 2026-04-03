use super::super::{check};
use guardrail3_domain_report::CheckResult;
use crate::inputs::RustfmtRootInput;
pub(super) fn run_check(parsed: toml::Value) -> Vec<CheckResult> {
    let input = RustfmtRootInput {
        config_rel: Some("rustfmt.toml".to_owned()),
        parsed: Some(parsed),
        escape_hatches: Vec::new(),
        cargo_edition: crate::facts::CargoEditionState::Present("2024".to_owned()),
        toolchain_channel: crate::facts::ToolchainChannelState::Present("stable".to_owned()),
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}
