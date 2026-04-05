use cargo_toml_parser::parse as parse_cargo_toml;
use g3_deps_content_checks_types::G3DepsPolicyContentChecksInput;
use guardrail3_check_types::G3CheckResult;
use guardrail3_domain_config::types::GuardrailConfig;

use crate::rs_deps_06_build_dependencies_allowlisted::rule::check;

pub(super) fn run_check(
    workspace_cargo_toml: &str,
    crate_cargo_toml: &str,
    guardrail_toml: &str,
) -> Vec<G3CheckResult> {
    let input = G3DepsPolicyContentChecksInput {
        workspace_cargo_rel_path: "Cargo.toml".to_owned(),
        workspace_cargo: parse_cargo_toml(workspace_cargo_toml)
            .expect("workspace Cargo.toml fixture should parse"),
        crate_cargo_rel_path: "packages/core/Cargo.toml".to_owned(),
        crate_cargo: parse_cargo_toml(crate_cargo_toml).expect("crate Cargo.toml fixture should parse"),
        guardrail_rel_path: "guardrail3.toml".to_owned(),
        guardrail: toml::from_str::<GuardrailConfig>(guardrail_toml)
            .expect("guardrail3.toml fixture should parse"),
        local_path_cargo_rel_paths: Vec::new(),
        local_path_cargo_manifests: Vec::new(),
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}
