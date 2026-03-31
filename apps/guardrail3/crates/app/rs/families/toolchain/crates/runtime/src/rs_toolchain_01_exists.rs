use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ToolchainPolicyRootInput;

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
            "Expected rust-toolchain.toml at workspace root.".to_owned(),
            Some(expected_toolchain_rel(input.rel_dir)),
            None,
            false,
        )),
    }
}

#[cfg(test)]
pub(crate) fn test_input<'a>(
    toolchain_toml_rel: Option<&'a str>,
    legacy_toolchain_rel: Option<&'a str>,
    parsed: Option<&'a toml::Value>,
    parse_error: Option<&'a str>,
    cargo_rust_version: Option<&'a str>,
    cargo_parse_error: Option<&'a str>,
) -> ToolchainPolicyRootInput<'a> {
    test_input_for_root(
        "",
        "Cargo.toml",
        toolchain_toml_rel,
        legacy_toolchain_rel,
        parsed,
        parse_error,
        cargo_rust_version,
        cargo_parse_error,
    )
}

#[cfg(test)]
pub(crate) fn test_input_for_root<'a>(
    rel_dir: &'a str,
    cargo_rel_path: &'a str,
    toolchain_toml_rel: Option<&'a str>,
    legacy_toolchain_rel: Option<&'a str>,
    parsed: Option<&'a toml::Value>,
    parse_error: Option<&'a str>,
    cargo_rust_version: Option<&'a str>,
    cargo_parse_error: Option<&'a str>,
) -> ToolchainPolicyRootInput<'a> {
    ToolchainPolicyRootInput {
        rel_dir,
        cargo_rel_path,
        #[cfg(test)]
        cargo_toml_rel: Some(cargo_rel_path),
        toolchain_toml_rel,
        legacy_toolchain_rel,
        parsed,
        parse_error,
        cargo_rust_version,
        cargo_rust_version_invalid: false,
        cargo_parse_error,
    }
}

#[cfg(test)]
pub(crate) fn run_family_check(
    tree: &guardrail3_domain_project_tree::ProjectTree,
) -> Vec<CheckResult> {
    crate::check(tree, &test_route(tree))
}

#[cfg(test)]
pub(crate) fn test_tree(
    root_files: &[&str],
    content: &[(&str, &str)],
) -> guardrail3_domain_project_tree::ProjectTree {
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    use guardrail3_domain_project_tree::{DirEntry, ProjectTree};

    let structure = BTreeMap::from([(
        String::new(),
        DirEntry::new(
            Vec::new(),
            root_files.iter().map(|file| (*file).to_owned()).collect(),
            Vec::new(),
            Vec::new(),
        ),
    )]);
    let content = content
        .iter()
        .map(|(path, file_content)| ((*path).to_owned(), (*file_content).to_owned()))
        .collect();

    ProjectTree::new(
        PathBuf::from("/tmp/toolchain-family-tests"),
        structure,
        content,
    )
}

#[cfg(test)]
pub(crate) fn nested_workspace_root_tree() -> guardrail3_domain_project_tree::ProjectTree {
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    use guardrail3_domain_project_tree::{DirEntry, ProjectTree};

    let structure = BTreeMap::from([
        (
            String::new(),
            DirEntry::new(
                vec!["apps".to_owned()],
                vec!["rust-toolchain.toml".to_owned()],
                Vec::new(),
                Vec::new(),
            ),
        ),
        (
            "apps".to_owned(),
            DirEntry::new(
                vec!["guardrail3".to_owned()],
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ),
        ),
        (
            "apps/guardrail3".to_owned(),
            DirEntry::new(
                Vec::new(),
                vec!["Cargo.toml".to_owned()],
                Vec::new(),
                Vec::new(),
            ),
        ),
    ]);
    let content = BTreeMap::from([
        (
            "rust-toolchain.toml".to_owned(),
            "[toolchain]\nchannel = \"stable\"\ncomponents = [\"clippy\", \"rustfmt\"]".to_owned(),
        ),
        (
            "apps/guardrail3/Cargo.toml".to_owned(),
            "[workspace]\n".to_owned(),
        ),
    ]);
    ProjectTree::new(
        PathBuf::from("/tmp/toolchain-family-nested-root"),
        structure,
        content,
    )
}

#[cfg(test)]
pub(crate) fn test_route(
    tree: &guardrail3_domain_project_tree::ProjectTree,
) -> guardrail3_app_rs_family_mapper::RsToolchainRoute {
    let scope = guardrail3_app_rs_structure::collect(tree);
    let selected =
        guardrail3_validation_model::RustFamilySelection::new(std::collections::BTreeSet::from([
            guardrail3_validation_model::RustValidateFamily::Toolchain,
        ]));
    guardrail3_app_rs_family_mapper::FamilyMapper::new(tree, &scope, None, &selected, None)
        .map_rs_toolchain()
}

fn expected_toolchain_rel(rel_dir: &str) -> String {
    if rel_dir.is_empty() {
        "rust-toolchain.toml".to_owned()
    } else {
        guardrail3_domain_project_tree::ProjectTree::join_rel(rel_dir, "rust-toolchain.toml")
    }
}

#[cfg(test)]
#[path = "rs_toolchain_01_exists_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_toolchain_01_exists_tests;
