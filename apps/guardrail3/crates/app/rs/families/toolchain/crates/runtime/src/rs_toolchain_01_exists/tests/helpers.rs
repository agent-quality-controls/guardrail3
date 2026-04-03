use guardrail3_domain_report::CheckResult;
use crate::inputs::ToolchainPolicyRootInput;
pub(super) fn test_input<'a>(
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
pub(super) fn test_input_for_root<'a>(
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
pub(super) fn run_family_check(
    tree: &guardrail3_app_rs_family_view::FamilyView,
) -> Vec<CheckResult> {
    crate::check(
        tree,
        &test_route(tree),
    )
}
pub(super) fn test_tree(
    root_files: &[&str],
    content: &[(&str, &str)],
) -> guardrail3_app_rs_family_view::FamilyView {
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    use guardrail3_app_rs_family_view::{DirEntry, FamilyView as ProjectTree};

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

    ProjectTree::build(
        PathBuf::from("/tmp/toolchain-family-tests"),
        &structure,
        &content,
        &["".to_owned()],
        &[],
        &[],
        None,
        &[],
    )
}
pub(super) fn nested_workspace_root_tree() -> guardrail3_app_rs_family_view::FamilyView {
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    use guardrail3_app_rs_family_view::{DirEntry, FamilyView as ProjectTree};

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
    ProjectTree::build(
        PathBuf::from("/tmp/toolchain-family-nested-root"),
        &structure,
        &content,
        &["".to_owned()],
        &[],
        &[],
        None,
        &[],
    )
}
pub(super) fn test_route(
    tree: &guardrail3_app_rs_family_view::FamilyView,
) -> guardrail3_app_rs_family_mapper::RsToolchainRoute {
    let pt = guardrail3_domain_project_tree::ProjectTree::new(tree.root_path().to_path_buf(), tree.structure().clone(), tree.content().clone());
    let structure = guardrail3_app_rs_structure::collect(pt, &[]);
    let legality = guardrail3_app_rs_legality::collect(structure);
    let selected =
        guardrail3_validation_model::RustFamilySelection::new(std::collections::BTreeSet::from([
            guardrail3_validation_model::RustValidateFamily::Toolchain,
        ]));
    guardrail3_app_rs_family_mapper::FamilyMapper::from_legality(&legality, None, &selected, None)
        .map_rs_toolchain()
}
