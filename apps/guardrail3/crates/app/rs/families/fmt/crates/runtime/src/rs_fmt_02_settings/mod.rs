mod rule;
pub use rule::{check};
#[cfg(test)]
use crate::facts::CargoEditionState;
#[cfg(test)]
use guardrail3_domain_report::CheckResult;
#[cfg(test)]
use crate::inputs::RustfmtRootInput;

#[cfg(test)]
pub(crate) fn run_check(parsed: Option<toml::Value>) -> Vec<CheckResult> {
    let input = RustfmtRootInput {
        config_rel: Some("rustfmt.toml".to_owned()),
        parsed,
        escape_hatches: Vec::new(),
        cargo_edition: CargoEditionState::Present("2024".to_owned()),
        toolchain_channel: super::facts::ToolchainChannelState::Present("stable".to_owned()),
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}
#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}
#[cfg(test)]

mod tests;
