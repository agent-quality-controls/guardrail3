use super::super::{check};
use crate::facts::CargoEditionState;
use guardrail3_domain_report::CheckResult;
use crate::inputs::RustfmtRootInput;
pub(super) enum TestCargoEditionState {
    Edition(&'static str),
}
pub(super) fn run_check(
    cargo_edition: TestCargoEditionState,
    rustfmt_edition: &str,
) -> Vec<CheckResult> {
    let cargo_edition = match cargo_edition {
        TestCargoEditionState::Edition(value) => CargoEditionState::Present(value.to_owned()),
    };
    let input = RustfmtRootInput {
        config_rel: Some("rustfmt.toml".to_owned()),
        parsed: Some(
            toml::from_str::<toml::Value>(&format!(
                "edition = \"{rustfmt_edition}\"\nmax_width = 100\ntab_spaces = 4\nuse_field_init_shorthand = true\nuse_try_shorthand = true\nreorder_imports = true\nreorder_modules = true\n"
            ))
            .expect("RS-FMT-06 in-memory rustfmt TOML fixture should parse"),
        ),
        escape_hatches: Vec::new(),
        cargo_edition,
        toolchain_channel: crate::facts::ToolchainChannelState::Present("stable".to_owned()),
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}
pub(super) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}
