use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::ToolchainRootInput;

const ID: &str = "RS-TOOLCHAIN-04";

pub fn check(input: &ToolchainRootInput<'_>, results: &mut Vec<CheckResult>) {
    if let Some(legacy_rel) = input.legacy_toolchain_rel {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "legacy rust-toolchain file present".to_owned(),
            "Migrate `rust-toolchain` to `rust-toolchain.toml` so components can be declared explicitly."
                .to_owned(),
            Some(legacy_rel.to_owned()),
            None,
            false,
        ));
    }

    if let (Some(legacy_rel), Some(_modern_rel)) =
        (input.legacy_toolchain_rel, input.toolchain_toml_rel)
    {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "both rust-toolchain files present".to_owned(),
            "Remove the legacy `rust-toolchain` file. rustup prefers it over `rust-toolchain.toml`, so the modern contract is shadowed.".to_owned(),
            Some(legacy_rel.to_owned()),
            None,
            false,
        ));
    }
}

