use cargo_toml_parser::parse as parse_cargo_toml;
use g3rs_deps_config_checks_types::G3RsDepsConfigChecksInput;
use guardrail3_check_types::G3CheckResult;
use guardrail3_domain_config::types::GuardrailConfig;

use crate::rs_deps_config_04_library_allowlist_present::rule::check;

pub(super) fn run_check(guardrail_toml: &str) -> Vec<G3CheckResult> {
    let input = G3RsDepsConfigChecksInput {
        workspace_cargo_rel_path: "Cargo.toml".to_owned(),
        workspace_cargo: parse_cargo_toml("[workspace]\nmembers = [\"packages/core\"]\n")
            .expect("workspace Cargo.toml fixture should parse"),
        crate_cargo_rel_path: "packages/core/Cargo.toml".to_owned(),
        crate_cargo: parse_cargo_toml("[package]\nname = \"core\"\n")
            .expect("crate Cargo.toml fixture should parse"),
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
