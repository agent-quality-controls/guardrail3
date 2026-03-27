use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use guardrail3_adapters_outbound_fs::RealFileSystem;
use guardrail3_app_core::project_walker::walk_project;
use guardrail3_app_rs_family_mapper::{FamilyMapper, RsCodeRoute};
use guardrail3_domain_config::types::GuardrailConfig;
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::CheckResult;
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};

const GOLDEN_REL: &str = "../../../../../../../tests/fixtures/r_arch_01/golden";

pub fn temp_root(slug: &str) -> PathBuf {
    let unique = format!(
        "{}-{}-{}",
        slug,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    );
    std::env::temp_dir().join(unique)
}

pub fn files_for_rule(results: &[CheckResult], rule_id: &str) -> BTreeSet<String> {
    results
        .iter()
        .filter(|result| result.id == rule_id)
        .filter_map(|result| result.file.clone())
        .collect()
}

pub fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(GOLDEN_REL)
}

pub fn copy_fixture() -> tempfile::TempDir {
    let tmp = tempfile::tempdir().expect("create tempdir");
    copy_dir_recursive(&fixture_root(), tmp.path());
    tmp
}

pub fn write_file(root: &Path, rel: &str, content: &str) {
    let path = root.join(rel);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create parent");
    }
    std::fs::write(path, content).expect("write file");
}

pub fn run_family(root: &Path) -> Vec<CheckResult> {
    let tree = walk_project(&RealFileSystem, root);
    super::check(&tree, &family_route(&tree, None))
}

pub fn run_tree(tree: &ProjectTree) -> Vec<CheckResult> {
    super::check(tree, &family_route(tree, None))
}

pub fn family_route(tree: &ProjectTree, scoped_files: Option<&BTreeSet<String>>) -> RsCodeRoute {
    let scope = guardrail3_app_rs_placement::collect(tree);
    let config = parse_guardrail_config(tree);
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Code]));
    FamilyMapper::new(tree, &scope, config.as_ref(), &selected, scoped_files).map_rs_code()
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

fn parse_guardrail_config(tree: &ProjectTree) -> Option<GuardrailConfig> {
    tree.file_content("guardrail3.toml")
        .and_then(|content| toml::from_str::<GuardrailConfig>(content).ok())
}
