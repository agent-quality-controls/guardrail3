#![cfg(test)]

const GOLDEN_REL: &str = "../../../../../tests/fixtures/full_golden";

pub(crate) fn fixture_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(GOLDEN_REL)
}

pub(crate) fn copy_fixture() -> tempfile::TempDir {
    let tmp = tempfile::tempdir().expect("failed to create release fixture tempdir");
    copy_dir_recursive(&fixture_root(), tmp.path());
    tmp
}

pub(crate) fn run_family(
    root: &std::path::Path,
    thorough: bool,
) -> Vec<guardrail3_domain_report::CheckResult> {
    let walked = guardrail3_app_core::project_walker::walk_project(
        &guardrail3_adapters_outbound_fs::RealFileSystem,
        root,
    );
    let tree = guardrail3_app_rs_family_view::FamilyView::build(
        walked.root().clone(),
        walked.structure(),
        walked.content(),
        &["".to_owned()],
        &[],
        &[],
        None,
    );
    run_tree(
        &tree,
        &guardrail3_adapters_outbound_tool_runner::RealToolChecker,
        thorough,
    )
}

pub(crate) fn run_tree(
    tree: &guardrail3_app_rs_family_view::FamilyView,
    tc: &dyn guardrail3_outbound_traits::ToolChecker,
    thorough: bool,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::check(
        tree,
        &family_route(tree),
        tc,
        thorough,
    )
}

pub(crate) fn run_tree_with_validation_scope(
    tree: &guardrail3_app_rs_family_view::FamilyView,
    tc: &dyn guardrail3_outbound_traits::ToolChecker,
    thorough: bool,
    validation_scope: &str,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::check(
        tree,
        &family_route_with_validation_scope(tree, validation_scope),
        tc,
        thorough,
    )
}

pub(crate) fn repo_facts() -> crate::facts::RepoReleaseFacts {
    crate::facts::RepoReleaseFacts {
        cargo_rel_path: "Cargo.toml".to_owned(),
        license_rel_path: None,
        release_plz_rel_path: "release-plz.toml".to_owned(),
        release_plz_exists: false,
        release_plz_parsed: None,
        release_plz_package_names: std::collections::BTreeSet::new(),
        cliff_rel_path: "cliff.toml".to_owned(),
        cliff_exists: false,
        cliff_parsed: None,
        workflows: Vec::new(),
        publishable_crate_names: std::collections::BTreeSet::new(),
        publishable_binary_crate_names: std::collections::BTreeSet::new(),
        publishable_count: 0,
        non_publishable_count: 0,
        semver_checks_installed: false,
        publish_setting: None,
        release_profile_settings: Vec::new(),
    }
}

pub(crate) fn workflow_from_yaml(rel_path: &str, yaml: &str) -> crate::facts::WorkflowFacts {
    let parsed: serde_yaml::Value =
        serde_yaml::from_str(yaml).expect("failed to parse release workflow fixture yaml");
    let analysis = crate::release_support::extract_workflow_analysis(&parsed);
    crate::facts::WorkflowFacts {
        rel_path: rel_path.to_owned(),
        analysis,
    }
}

pub(crate) fn crate_facts(name: &str) -> crate::facts::PublishableCrateFacts {
    let mut binary_target_names = std::collections::BTreeSet::new();
    let _ = binary_target_names.insert(name.to_owned());
    crate::facts::PublishableCrateFacts {
        name: name.to_owned(),
        cargo_rel_path: "crates/example/Cargo.toml".to_owned(),
        binary_target_names,
        publishable: true,
        is_binary: false,
        is_library: true,
        description_present: true,
        license_present: true,
        repository_present: true,
        readme_declared_false: false,
        readme_rel_path: "crates/example/README.md".to_owned(),
        readme_exists: true,
        readme_content: Some("# Example\n\n".to_owned() + &"x".repeat(240)),
        keywords_count: Some(3),
        categories_count: Some(1),
        version_string: Some("1.2.3".to_owned()),
        workspace_version: false,
        version_valid: true,
        docs_rs_present: true,
        include_exclude_present: true,
        has_binstall_metadata: false,
        dry_run: None,
    }
}

pub(crate) fn edge_facts() -> crate::facts::ReleaseEdgeFacts {
    crate::facts::ReleaseEdgeFacts {
        crate_name: "example".to_owned(),
        cargo_rel_path: "crates/example/Cargo.toml".to_owned(),
        dep_name: "dep".to_owned(),
        dep_package_name: "dep".to_owned(),
        section_label: "dependencies".to_owned(),
        target_label: None,
        has_path: true,
        dep_publishable: true,
        version_req: Some("1.0".to_owned()),
        actual_version: Some("1.2.3".to_owned()),
        version_satisfied: Some(true),
    }
}

pub(crate) fn repo_input(
    repo: &crate::facts::RepoReleaseFacts,
) -> crate::inputs::RepoReleaseInput<'_> {
    crate::inputs::RepoReleaseInput::new(repo)
}

pub(crate) fn crate_input(
    krate: &crate::facts::PublishableCrateFacts,
) -> crate::inputs::PublishableCrateReleaseInput<'_> {
    crate::inputs::PublishableCrateReleaseInput::new(krate)
}

pub(crate) fn edge_input(
    edge: &crate::facts::ReleaseEdgeFacts,
) -> crate::inputs::ReleaseEdgeInput<'_> {
    crate::inputs::ReleaseEdgeInput::new(edge)
}

pub(crate) fn family_route(
    tree: &guardrail3_app_rs_family_view::FamilyView,
) -> guardrail3_app_rs_family_mapper::RsReleaseRoute {
    let pt = guardrail3_domain_project_tree::ProjectTree::new(tree.root_path().to_path_buf(), tree.structure().clone(), tree.content().clone());
    let structure = guardrail3_app_rs_structure::collect(pt, &[]);
    let legality = guardrail3_app_rs_legality::collect(structure);
    let selected =
        guardrail3_validation_model::RustFamilySelection::new(std::collections::BTreeSet::from([
            guardrail3_validation_model::RustValidateFamily::Release,
        ]));
    guardrail3_app_rs_family_mapper::FamilyMapper::from_legality(&legality, None, &selected, None)
        .map_rs_release()
}

pub(crate) fn family_route_with_validation_scope(
    tree: &guardrail3_app_rs_family_view::FamilyView,
    validation_scope: &str,
) -> guardrail3_app_rs_family_mapper::RsReleaseRoute {
    let pt = guardrail3_domain_project_tree::ProjectTree::new(tree.root_path().to_path_buf(), tree.structure().clone(), tree.content().clone());
    let structure = guardrail3_app_rs_structure::collect(pt, &[]);
    let legality = guardrail3_app_rs_legality::collect(structure);
    let selected =
        guardrail3_validation_model::RustFamilySelection::new(std::collections::BTreeSet::from([
            guardrail3_validation_model::RustValidateFamily::Release,
        ]));
    guardrail3_app_rs_family_mapper::FamilyMapper::from_legality(&legality, None, &selected, None)
        .with_validation_scope(Some(validation_scope))
        .map_rs_release()
}

fn copy_dir_recursive(src: &std::path::Path, dst: &std::path::Path) {
    for entry in guardrail3_shared_fs::list_dir(src) {
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            guardrail3_shared_fs::create_dir_all(&dst_path)
                .expect("failed to create release fixture destination directory");
            copy_dir_recursive(&src_path, &dst_path);
        } else {
            let _ = guardrail3_shared_fs::copy_file(&src_path, &dst_path)
                .expect("failed to copy release fixture file");
        }
    }
}
