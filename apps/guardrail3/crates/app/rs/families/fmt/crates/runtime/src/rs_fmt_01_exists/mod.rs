mod rule;
pub use rule::{check};

#[cfg(test)]
pub(crate) fn run_check(config_rel: Option<&str>) -> Vec<CheckResult> {
    let input = RustfmtRootInput {
        config_rel: config_rel.map(str::to_owned),
        parsed: None,
        escape_hatches: Vec::new(),
        cargo_edition: super::facts::CargoEditionState::Present("2024".to_owned()),
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
