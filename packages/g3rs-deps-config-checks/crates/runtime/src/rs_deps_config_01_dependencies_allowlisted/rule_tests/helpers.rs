use cargo_toml_parser::parse as parse_cargo_toml;
use g3rs_deps_config_checks_types::{G3RsDepsConfigLocalPathCargoManifest, G3RsDepsConfigPolicyChecksInput};
use guardrail3_check_types::G3CheckResult;
use guardrail3_domain_config::types::GuardrailConfig;

use crate::rs_deps_config_01_dependencies_allowlisted::rule::check;

pub(super) fn run_check(
    workspace_cargo_toml: &str,
    crate_cargo_rel_path: &str,
    crate_cargo_toml: &str,
    guardrail_toml: &str,
) -> Vec<G3CheckResult> {
    run_check_with_local_paths(
        workspace_cargo_toml,
        crate_cargo_rel_path,
        crate_cargo_toml,
        guardrail_toml,
        &[],
        &[],
    )
}

pub(super) fn run_check_with_local_paths(
    workspace_cargo_toml: &str,
    crate_cargo_rel_path: &str,
    crate_cargo_toml: &str,
    guardrail_toml: &str,
    local_path_cargo_rel_paths: &[&str],
    local_path_cargo_manifests: &[(&str, &str)],
) -> Vec<G3CheckResult> {
    let input = G3RsDepsConfigPolicyChecksInput {
        workspace_cargo_rel_path: "Cargo.toml".to_owned(),
        workspace_cargo: parse_cargo_toml(workspace_cargo_toml)
            .expect("workspace Cargo.toml fixture should parse"),
        crate_cargo_rel_path: crate_cargo_rel_path.to_owned(),
        crate_cargo: parse_cargo_toml(crate_cargo_toml).expect("crate Cargo.toml fixture should parse"),
        guardrail_rel_path: "guardrail3.toml".to_owned(),
        guardrail: toml::from_str::<GuardrailConfig>(guardrail_toml)
            .expect("guardrail3.toml fixture should parse"),
        local_path_cargo_rel_paths: local_path_cargo_rel_paths
            .iter()
            .map(|path| (*path).to_owned())
            .collect(),
        local_path_cargo_manifests: local_path_cargo_manifests
            .iter()
            .map(|(cargo_rel_path, cargo_toml)| G3RsDepsConfigLocalPathCargoManifest {
                cargo_rel_path: (*cargo_rel_path).to_owned(),
                cargo: parse_cargo_toml(cargo_toml)
                    .expect("local path Cargo.toml fixture should parse"),
            })
            .collect(),
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}
