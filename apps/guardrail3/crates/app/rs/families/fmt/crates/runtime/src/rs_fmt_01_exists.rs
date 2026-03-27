use guardrail3_domain_report::CheckResult;

use super::inputs::RustfmtRootInput;

const ID: &str = "RS-FMT-01";

pub fn check(input: &RustfmtRootInput, results: &mut Vec<CheckResult>) {
    match input.config_rel.as_deref() {
        Some(_rel) => {}
        None => results.push(CheckResult {
            id: ID.to_owned(),
            severity: guardrail3_domain_report::Severity::Error,
            title: "rustfmt config missing".to_owned(),
            message: "Expected rustfmt.toml or .rustfmt.toml at workspace root".to_owned(),
            file: Some("".to_owned()),
            line: None,
            inventory: false,
        }),
    }
}

#[cfg(test)]
pub(crate) fn run_check(config_rel: Option<&str>) -> Vec<CheckResult> {
    let input = RustfmtRootInput {
        config_rel: config_rel.map(str::to_owned),
        parsed: None,
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
#[path = "rs_fmt_01_exists_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_fmt_01_exists_tests;
