use std::path::{Path, PathBuf};

use guardrail3_adapters_outbound_fs::RealFileSystem;
use guardrail3_app_core::project_walker::walk_project;
use guardrail3_app_rs_family_mapper::{FamilyMapper, RsClippyRoute};
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::CheckResult;
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};

use super::facts::{ClippyFacts, collect};
use super::inputs::ConfigClippyInput;

pub use test_support::{
    build_fixture_clippy_toml, dir_entry, garde_disabled_root_tree,
    incomplete_workspace_policy_root_tree, library_workspace_root_tree,
    nested_workspace_member_shadow_tree, prepend_ban_path,
    project_tree, published_library_package_root_tree, remove_ban_path,
    root_workspace_tree, same_root_dual_config_tree, write_file,
};

const GOLDEN_REL: &str = "../../../../../../../tests/fixtures/r_arch_01/golden";

pub fn collected_facts(tree: &ProjectTree) -> ClippyFacts {
    collect(tree, &family_route_for_tests(tree))
}

pub fn config_input<'a>(facts: &'a ClippyFacts, rel_path: &str) -> ConfigClippyInput<'a> {
    let config = facts
        .allowed_configs
        .iter()
        .find(|config| config.rel_path == rel_path)
        .expect("expected clippy config facts");
    ConfigClippyInput::new(config)
}

pub fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(GOLDEN_REL)
}

pub fn copy_fixture() -> test_support::TempDir {
    test_support::copy_tree(&fixture_root())
}

pub fn run_family(root: &Path) -> Vec<CheckResult> {
    let tree = walk_project(&RealFileSystem, root);
    super::check(&tree, &family_route_for_tests(&tree))
}

pub(crate) fn family_route_for_tests(tree: &ProjectTree) -> RsClippyRoute {
    let scope = guardrail3_app_rs_placement::collect(tree);
    let selected = RustFamilySelection::new(std::collections::BTreeSet::from([
        RustValidateFamily::Clippy,
    ]));
    FamilyMapper::new(tree, &scope, None, &selected, None).map_rs_clippy()
}
