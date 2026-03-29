#![cfg(test)]

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use serde_yaml::Value as YamlValue;

use guardrail3_adapters_outbound_tool_runner::RealToolChecker;
use guardrail3_app_core::project_walker::walk_project;
use guardrail3_app_rs_family_mapper::{FamilyMapper, RsReleaseRoute};
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::CheckResult;
use guardrail3_outbound_traits::ToolChecker;
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};

use crate::facts::{PublishableCrateFacts, ReleaseEdgeFacts, RepoReleaseFacts, WorkflowFacts};
use crate::inputs::{PublishableCrateReleaseInput, ReleaseEdgeInput, RepoReleaseInput};
use crate::release_support::extract_workflow_analysis;

const GOLDEN_REL: &str = "../../../../../tests/fixtures/r_arch_01/golden";

pub(crate) fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(GOLDEN_REL)
}

pub(crate) fn copy_fixture() -> tempfile::TempDir {
    let tmp = tempfile::tempdir().expect("create tempdir");
    copy_dir_recursive(&fixture_root(), tmp.path());
    tmp
}

pub(crate) fn run_family(root: &Path, thorough: bool) -> Vec<CheckResult> {
    let tree = walk_project(&guardrail3_adapters_outbound_fs::RealFileSystem, root);
    run_tree(&tree, &RealToolChecker, thorough)
}

pub(crate) fn run_tree(
    tree: &ProjectTree,
    tc: &dyn ToolChecker,
    thorough: bool,
) -> Vec<CheckResult> {
    crate::check(tree, &family_route(tree), tc, thorough)
}

pub(crate) fn repo_facts() -> RepoReleaseFacts {
    RepoReleaseFacts {
        cargo_rel_path: "Cargo.toml".to_owned(),
        license_rel_path: None,
        release_plz_rel_path: "release-plz.toml".to_owned(),
        release_plz_exists: false,
        release_plz_parsed: None,
        release_plz_package_names: BTreeSet::new(),
        cliff_rel_path: "cliff.toml".to_owned(),
        cliff_exists: false,
        cliff_parsed: None,
        workflows: Vec::new(),
        publishable_crate_names: BTreeSet::new(),
        publishable_binary_crate_names: BTreeSet::new(),
        publishable_count: 0,
        non_publishable_count: 0,
        semver_checks_installed: false,
        publish_setting: None,
        release_profile_settings: Vec::new(),
    }
}

pub(crate) fn workflow_from_yaml(rel_path: &str, yaml: &str) -> WorkflowFacts {
    let parsed: YamlValue = serde_yaml::from_str(yaml).expect("valid workflow yaml");
    let analysis = extract_workflow_analysis(&parsed);
    WorkflowFacts {
        rel_path: rel_path.to_owned(),
        analysis,
    }
}

pub(crate) fn crate_facts(name: &str) -> PublishableCrateFacts {
    let mut binary_target_names = BTreeSet::new();
    let _ = binary_target_names.insert(name.to_owned());
    PublishableCrateFacts {
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

pub(crate) fn edge_facts() -> ReleaseEdgeFacts {
    ReleaseEdgeFacts {
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

pub(crate) fn repo_input(repo: &RepoReleaseFacts) -> RepoReleaseInput<'_> {
    RepoReleaseInput::new(repo)
}

pub(crate) fn crate_input(krate: &PublishableCrateFacts) -> PublishableCrateReleaseInput<'_> {
    PublishableCrateReleaseInput::new(krate)
}

pub(crate) fn edge_input(edge: &ReleaseEdgeFacts) -> ReleaseEdgeInput<'_> {
    ReleaseEdgeInput::new(edge)
}

pub(crate) fn family_route(tree: &ProjectTree) -> RsReleaseRoute {
    let scope = guardrail3_app_rs_placement::collect(tree);
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Release]));
    FamilyMapper::new(tree, &scope, None, &selected, None).map_rs_release()
}

fn copy_dir_recursive(src: &Path, dst: &Path) {
    for entry in std::fs::read_dir(src).expect("read fixture dir") {
        let entry = entry.expect("read entry");
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            std::fs::create_dir_all(&dst_path).expect("create dst dir");
            copy_dir_recursive(&src_path, &dst_path);
        } else {
            let _ = std::fs::copy(&src_path, &dst_path).expect("copy fixture file");
        }
    }
}
