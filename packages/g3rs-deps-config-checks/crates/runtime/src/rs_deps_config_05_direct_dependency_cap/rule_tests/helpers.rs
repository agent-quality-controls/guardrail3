use cargo_toml_parser::parse as parse_cargo_toml;
use g3rs_deps_config_checks_types::{G3RsDepsConfigDirectDependencyCapInput, G3RsDepsConfigLocalPathCargoManifest};
use guardrail3_check_types::G3CheckResult;

use crate::rs_deps_config_05_direct_dependency_cap::rule::check;

pub(super) fn run_check(workspace_cargo_toml: &str, crate_cargo_toml: &str) -> Vec<G3CheckResult> {
    run_check_with_local_paths(workspace_cargo_toml, crate_cargo_toml, &[], &[])
}

pub(super) fn run_check_with_local_paths(
    workspace_cargo_toml: &str,
    crate_cargo_toml: &str,
    local_path_cargo_rel_paths: &[&str],
    local_path_cargo_manifests: &[(&str, &str)],
) -> Vec<G3CheckResult> {
    let input = G3RsDepsConfigDirectDependencyCapInput {
        workspace_cargo_rel_path: "Cargo.toml".to_owned(),
        workspace_cargo: parse_cargo_toml(workspace_cargo_toml)
            .expect("workspace Cargo.toml fixture should parse"),
        crate_cargo_rel_path: "apps/api/Cargo.toml".to_owned(),
        crate_cargo: parse_cargo_toml(crate_cargo_toml).expect("crate Cargo.toml fixture should parse"),
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
