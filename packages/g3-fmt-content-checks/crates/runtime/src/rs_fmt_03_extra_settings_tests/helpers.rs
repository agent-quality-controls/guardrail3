use cargo_toml_parser::parse as parse_cargo_toml;
use g3_fmt_content_checks_types::G3FmtContentChecksInput;
use guardrail3_check_types::GrdzCheckResult;
use rust_toolchain_toml_parser::parse as parse_toolchain_toml;
use rustfmt_toml_parser::parse as parse_rustfmt_toml;

use crate::rs_fmt_03_extra_settings::check;

pub(super) fn run_check(rustfmt_toml: &str) -> Vec<GrdzCheckResult> {
    let input = G3FmtContentChecksInput {
        rustfmt_rel_path: "rustfmt.toml".to_owned(),
        rustfmt: parse_rustfmt_toml(rustfmt_toml)
            .expect("rustfmt test fixture should parse"),
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo: parse_cargo_toml(
            r#"
[workspace.package]
edition = "2024"
"#,
        )
        .expect("cargo fixture should parse"),
        toolchain_rel_path: "rust-toolchain.toml".to_owned(),
        toolchain: parse_toolchain_toml(
            r#"
[toolchain]
channel = "stable"
components = ["clippy", "rustfmt"]
"#,
        )
        .expect("toolchain fixture should parse"),
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}
