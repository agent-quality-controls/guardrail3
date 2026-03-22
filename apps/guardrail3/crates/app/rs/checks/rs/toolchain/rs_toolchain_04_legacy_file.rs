use crate::domain::report::{CheckResult, Severity};

use super::inputs::ToolchainRootInput;

const ID: &str = "RS-TOOLCHAIN-04";

pub fn check(input: &ToolchainRootInput<'_>, results: &mut Vec<CheckResult>) {
    if let Some(legacy_rel) = input.legacy_toolchain_rel {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "legacy rust-toolchain file present".to_owned(),
            message: "Migrate `rust-toolchain` to `rust-toolchain.toml` so components can be declared explicitly."
                .to_owned(),
            file: Some(legacy_rel.to_owned()),
            line: None,
            inventory: false,
        });
    }

    if input.legacy_toolchain_rel.is_some() && input.toolchain_toml_rel.is_some() {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "both rust-toolchain files present".to_owned(),
            message: "Remove the legacy `rust-toolchain` file to avoid ambiguity.".to_owned(),
            file: Some("rust-toolchain".to_owned()),
            line: None,
            inventory: false,
        });
    }
}
