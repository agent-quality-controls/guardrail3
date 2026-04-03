use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::ToolchainPolicyRootInput;

const ID: &str = "RS-TOOLCHAIN-01";

pub fn check(input: &ToolchainPolicyRootInput<'_>, results: &mut Vec<CheckResult>) {
    match input.toolchain_toml_rel {
        Some(rel) => results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "rust-toolchain.toml exists".to_owned(),
                "Found rust-toolchain.toml at workspace root.".to_owned(),
                Some(rel.to_owned()),
                None,
                false,
            )
            .as_inventory(),
        ),
        None => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "rust-toolchain.toml missing".to_owned(),
            "Expected rust-toolchain.toml at workspace root. Create a `rust-toolchain.toml` with a `[toolchain]` section containing `channel` and `components`.".to_owned(),
            Some(expected_toolchain_rel(input.rel_dir)),
            None,
            false,
        )),
    }
}







fn expected_toolchain_rel(rel_dir: &str) -> String {
    if rel_dir.is_empty() {
        "rust-toolchain.toml".to_owned()
    } else {
        guardrail3_app_rs_family_view::FamilyView::join_rel(rel_dir, "rust-toolchain.toml")
    }
}

