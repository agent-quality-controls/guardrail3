use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::UnownedToolchainInput;

const ID: &str = "RS-TOOLCHAIN-07";

pub fn check(input: &UnownedToolchainInput<'_>, results: &mut Vec<CheckResult>) {
    let title = if input.is_legacy {
        "toolchain file outside workspace root"
    } else {
        "toolchain file outside workspace root"
    };
    let file_name = if input.is_legacy {
        "`rust-toolchain`"
    } else {
        "`rust-toolchain.toml`"
    };

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        title.to_owned(),
        format!(
            "{file_name} at `{}` is not at a governed workspace root. Toolchain files are only allowed at workspace roots.",
            input.rel_path,
        ),
        Some(input.rel_path.to_owned()),
        None,
        false,
    ));
}

#[cfg(test)]
pub(crate) fn test_input(rel_path: &'static str, is_legacy: bool) -> UnownedToolchainInput<'static> {
    UnownedToolchainInput {
        rel_path,
        is_legacy,
    }
}

#[cfg(test)]
#[path = "rs_toolchain_07_illegal_placement_tests/mod.rs"]
mod rs_toolchain_07_illegal_placement_tests;
