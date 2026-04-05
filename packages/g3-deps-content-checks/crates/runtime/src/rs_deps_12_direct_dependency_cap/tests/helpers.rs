use cargo_toml_parser::parse as parse_cargo_toml;
use g3_deps_content_checks_types::G3DepsContentChecksInput;
use guardrail3_check_types::G3CheckResult;
use guardrail3_rs_toml_parser::parse as parse_guardrail_rs_toml;

use crate::rs_deps_12_direct_dependency_cap::rule::check;

pub(super) fn run_check(workspace_cargo_toml: &str, crate_cargo_toml: &str) -> Vec<G3CheckResult> {
    let input = G3DepsContentChecksInput {
        workspace_cargo_rel_path: "Cargo.toml".to_owned(),
        workspace_cargo: parse_cargo_toml(workspace_cargo_toml)
            .expect("workspace Cargo.toml fixture should parse"),
        crate_cargo_rel_path: "apps/api/Cargo.toml".to_owned(),
        crate_cargo: parse_cargo_toml(crate_cargo_toml).expect("crate Cargo.toml fixture should parse"),
        guardrail_rs_rel_path: "guardrail3-rs.toml".to_owned(),
        guardrail_rs: parse_guardrail_rs_toml("profile = \"service\"\n")
            .expect("guardrail3-rs.toml fixture should parse"),
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}
