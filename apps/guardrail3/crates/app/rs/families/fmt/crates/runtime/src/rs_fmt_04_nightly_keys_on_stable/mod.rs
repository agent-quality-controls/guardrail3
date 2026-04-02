mod rule;
pub use rule::{check};

#[cfg(test)]
pub(crate) enum TestToolchainState {
    Stable,
    Other,
}
#[cfg(test)]
pub(crate) fn run_check(state: TestToolchainState) -> Vec<CheckResult> {
    let toolchain_channel = match state {
        TestToolchainState::Stable => ToolchainChannelState::Present("stable".to_owned()),
        TestToolchainState::Other => ToolchainChannelState::Present("1.85.0".to_owned()),
    };
    let input = RustfmtRootInput {
        config_rel: Some("rustfmt.toml".to_owned()),
        parsed: Some(
            toml::from_str::<toml::Value>(
                r#"
edition = "2024"
max_width = 100
tab_spaces = 4
use_field_init_shorthand = true
use_try_shorthand = true
reorder_imports = true
reorder_modules = true
group_imports = "StdExternalCrate"
"#,
            )
            .expect("RS-FMT-04 in-memory rustfmt TOML fixture should parse"),
        ),
        escape_hatches: Vec::new(),
        cargo_edition: super::facts::CargoEditionState::Present("2024".to_owned()),
        toolchain_channel,
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
