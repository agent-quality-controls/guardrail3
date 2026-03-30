use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ToolchainPolicyRootInput;

#[cfg(test)]
use super::facts::PolicyRootKind;

const ID: &str = "RS-TOOLCHAIN-01";

pub fn check(input: &ToolchainPolicyRootInput<'_>, results: &mut Vec<CheckResult>) {
    match input.toolchain_toml_rel {
        Some(rel) => results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "rust-toolchain.toml exists".to_owned(),
                format!("Found rust-toolchain.toml at {}.", input.kind.label()),
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
            format!("Expected rust-toolchain.toml at {}.", input.kind.label()),
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
        PolicyRootKind::WorkspaceRoot,
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
    kind: PolicyRootKind,
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
        kind,
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
pub(crate) fn standalone_package_input(
    toolchain_toml_rel: Option<&'static str>,
) -> ToolchainPolicyRootInput<'static> {
    test_input_for_root(
        PolicyRootKind::StandalonePackageRoot,
        "packages/lib",
        "packages/lib/Cargo.toml",
        toolchain_toml_rel,
        None,
        None,
        None,
        Some("1.85"),
        None,
    )
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
        DirEntry {
            dirs: Vec::new(),
            files: root_files.iter().map(|file| (*file).to_owned()).collect(),
            symlink_dirs: Vec::new(),
            symlink_files: Vec::new(),
        },
    )]);
    let content = content
        .iter()
        .map(|(path, file_content)| ((*path).to_owned(), (*file_content).to_owned()))
        .collect();

    ProjectTree {
        root: PathBuf::from("/tmp/toolchain-family-tests"),
        structure,
        content,
    }
}

#[cfg(test)]
pub(crate) fn nested_workspace_root_tree() -> guardrail3_domain_project_tree::ProjectTree {
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    use guardrail3_domain_project_tree::{DirEntry, ProjectTree};

    let structure = BTreeMap::from([
        (
            String::new(),
            DirEntry {
                dirs: vec!["apps".to_owned()],
                files: vec!["rust-toolchain.toml".to_owned()],
                symlink_dirs: Vec::new(),
                symlink_files: Vec::new(),
            },
        ),
        (
            "apps".to_owned(),
            DirEntry {
                dirs: vec!["guardrail3".to_owned()],
                files: Vec::new(),
                symlink_dirs: Vec::new(),
                symlink_files: Vec::new(),
            },
        ),
        (
            "apps/guardrail3".to_owned(),
            DirEntry {
                dirs: Vec::new(),
                files: vec!["Cargo.toml".to_owned()],
                symlink_dirs: Vec::new(),
                symlink_files: Vec::new(),
            },
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
    ProjectTree {
        root: PathBuf::from("/tmp/toolchain-family-nested-root"),
        structure,
        content,
    }
}

#[cfg(test)]
pub(crate) fn test_route(
    tree: &guardrail3_domain_project_tree::ProjectTree,
) -> guardrail3_app_rs_family_mapper::RsToolchainRoute {
    let mut roots = Vec::new();

    if tree.file_exists("Cargo.toml") {
        roots.push(guardrail3_app_rs_family_mapper::RsRootView::new(
            String::new(),
            "Cargo.toml".to_owned(),
        ));
    }

    roots.extend(
        tree.dirs_with_file("Cargo.toml")
            .into_iter()
            .map(|rel_dir| {
                guardrail3_app_rs_family_mapper::RsRootView::new(
                    rel_dir.clone(),
                    guardrail3_domain_project_tree::ProjectTree::join_rel(&rel_dir, "Cargo.toml"),
                )
            }),
    );
    roots.sort_by(|left, right| left.cargo_rel_path().cmp(right.cargo_rel_path()));

    guardrail3_app_rs_family_mapper::RsToolchainRoute::new(roots)
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
