mod rule;
pub use rule::{check};
#[cfg(test)]
use guardrail3_domain_report::CheckResult;
#[cfg(test)]
use crate::inputs::RustfmtRootInput;

#[cfg(test)]
pub(crate) fn run_check(parsed: toml::Value) -> Vec<CheckResult> {
    run_check_with_escape_hatches(parsed, Vec::new())
}
#[cfg(test)]
pub(crate) fn run_check_with_escape_hatches(
    parsed: toml::Value,
    escape_hatches: Vec<guardrail3_domain_config::types::EscapeHatchConfig>,
) -> Vec<CheckResult> {
    let input = RustfmtRootInput {
        config_rel: Some("rustfmt.toml".to_owned()),
        parsed: Some(parsed),
        escape_hatches,
        cargo_edition: super::facts::CargoEditionState::Present("2024".to_owned()),
        toolchain_channel: super::facts::ToolchainChannelState::Present("stable".to_owned()),
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}
#[cfg(test)]

mod tests;
