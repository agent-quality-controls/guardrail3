mod full_tree_policy;

use std::path::{Path, PathBuf};

use guardrail3_adapters_outbound_fs::RealFileSystem;
use guardrail3_domain_config::types::GuardrailConfig;
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};

const GOLDEN_REL: &str = "../../../../../../../tests/fixtures/full_golden";

pub(super) fn run_family(root: &Path) -> Vec<guardrail3_domain_report::CheckResult> {
    let walked = guardrail3_app_core::project_walker::walk_project(&RealFileSystem, root);
    let tree = guardrail3_app_rs_family_view::FamilyView::build(
        walked.root().clone(),
        walked.structure(),
        walked.content(),
        &["".to_owned()],
        &[],
        &[],
        None,
        &[],
    );
    super::check_test_tree(&tree)
}

pub(super) fn route_family(root: &Path) -> guardrail3_app_rs_family_mapper::RsToolchainRoute {
    let walked = guardrail3_app_core::project_walker::walk_project(&RealFileSystem, root);
    let tree = guardrail3_app_rs_family_view::FamilyView::build(
        walked.root().clone(),
        walked.structure(),
        walked.content(),
        &["".to_owned()],
        &[],
        &[],
        None,
        &[],
    );
    let structure = guardrail3_app_rs_structure::collect(walked, &[]);
    let legality = guardrail3_app_rs_legality::collect(structure);
    let config = tree
        .file_content("guardrail3.toml")
        .and_then(|content| toml::from_str::<GuardrailConfig>(content).ok());
    let selected = RustFamilySelection::new(std::collections::BTreeSet::from([
        RustValidateFamily::Toolchain,
    ]));
    guardrail3_app_rs_family_mapper::FamilyMapper::from_legality(
        &legality,
        config.as_ref(),
        &selected,
        None,
    )
    .map_rs_toolchain()
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(GOLDEN_REL)
}

pub(super) fn copy_fixture() -> tempfile::TempDir {
    let tmp =
        tempfile::tempdir().expect("failed to create temporary directory for toolchain fixture");
    copy_dir_recursive(&fixture_root(), tmp.path());
    tmp
}

fn copy_dir_recursive(src: &Path, dst: &Path) {
    for entry in std::fs::read_dir(src).expect("failed to read source toolchain fixture") {
        let entry = entry.expect("failed to read entry from source toolchain fixture");
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            std::fs::create_dir_all(&dst_path)
                .expect("failed to create destination directory in toolchain fixture");
            copy_dir_recursive(&src_path, &dst_path);
        } else {
            let _ = std::fs::copy(&src_path, &dst_path)
                .expect("failed to copy file into temporary toolchain fixture");
        }
    }
}

pub(super) fn write_file(root: &Path, rel_path: &str, content: &str) {
    let abs_path = root.join(rel_path);
    if let Some(parent) = abs_path.parent() {
        guardrail3_shared_fs::create_dir_all(parent)
            .expect("failed to create parent directory for toolchain fixture mutation");
    }
    guardrail3_shared_fs::write_file(&abs_path, content)
        .expect("failed to write toolchain fixture mutation");
}
