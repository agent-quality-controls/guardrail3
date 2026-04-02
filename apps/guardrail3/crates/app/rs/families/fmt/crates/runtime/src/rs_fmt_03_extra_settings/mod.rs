mod rule;
pub use rule::{check};

#[cfg(test)]
pub(crate) fn run_check(parsed: toml::Value) -> Vec<CheckResult> {
    let input = RustfmtRootInput {
        config_rel: Some("rustfmt.toml".to_owned()),
        parsed: Some(parsed),
        escape_hatches: Vec::new(),
        cargo_edition: super::facts::CargoEditionState::Present("2024".to_owned()),
        toolchain_channel: super::facts::ToolchainChannelState::Present("stable".to_owned()),
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}
#[cfg(test)]

mod tests;
