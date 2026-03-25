use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ToolchainRootInput;

const ID: &str = "RS-TOOLCHAIN-01";

pub fn check(input: &ToolchainRootInput<'_>, results: &mut Vec<CheckResult>) {
    match input.toolchain_toml_rel {
        Some(rel) => results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "rust-toolchain.toml exists".to_owned(),
                message: "Found rust-toolchain.toml at workspace root.".to_owned(),
                file: Some(rel.to_owned()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        ),
        None => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "rust-toolchain.toml missing".to_owned(),
            message: "Expected rust-toolchain.toml at workspace root.".to_owned(),
            file: Some("".to_owned()),
            line: None,
            inventory: false,
        }),
    }
}

#[cfg(test)]
#[path = "rs_toolchain_01_exists_tests.rs"]
mod tests;
