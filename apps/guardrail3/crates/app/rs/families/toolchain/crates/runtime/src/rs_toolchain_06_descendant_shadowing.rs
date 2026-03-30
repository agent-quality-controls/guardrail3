use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ToolchainRootInput;

const ID: &str = "RS-TOOLCHAIN-06";

pub fn check(input: &ToolchainRootInput<'_>, results: &mut Vec<CheckResult>) {
    for descendant in &input.descendant_toolchains {
        let title = if descendant.is_legacy {
            "descendant legacy toolchain shadows workspace policy"
        } else {
            "descendant toolchain shadows workspace policy"
        };
        let file_name = if descendant.is_legacy {
            "`rust-toolchain`"
        } else {
            "`rust-toolchain.toml`"
        };

        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            title.to_owned(),
            format!(
                "Descendant {file_name} at `{}` can override the workspace-root toolchain contract. Keep toolchain policy at the workspace root only.",
                descendant.rel_path,
            ),
            Some(descendant.rel_path.to_owned()),
            None,
            false,
        ));
    }
}

#[cfg(test)]
pub(crate) fn test_input<'a>(
    descendant_toolchains: Vec<super::inputs::DescendantToolchainInput<'a>>,
) -> ToolchainRootInput<'a> {
    ToolchainRootInput {
        rel_dir: "",
        cargo_rel_path: "Cargo.toml",
        cargo_toml_rel: Some("Cargo.toml"),
        toolchain_toml_rel: Some("rust-toolchain.toml"),
        legacy_toolchain_rel: None,
        parsed: None,
        parse_error: None,
        cargo_rust_version: Some("1.85"),
        cargo_rust_version_invalid: false,
        cargo_parse_error: None,
        ancestor_toolchain: None,
        descendant_toolchains,
    }
}

#[cfg(test)]
pub(crate) fn descendant_modern(rel_path: &'static str) -> super::inputs::DescendantToolchainInput<'static> {
    super::inputs::DescendantToolchainInput {
        rel_path,
        is_legacy: false,
    }
}

#[cfg(test)]
pub(crate) fn descendant_legacy(rel_path: &'static str) -> super::inputs::DescendantToolchainInput<'static> {
    super::inputs::DescendantToolchainInput {
        rel_path,
        is_legacy: true,
    }
}

#[cfg(test)]
#[path = "rs_toolchain_06_descendant_shadowing_tests/mod.rs"]
mod rs_toolchain_06_descendant_shadowing_tests;
