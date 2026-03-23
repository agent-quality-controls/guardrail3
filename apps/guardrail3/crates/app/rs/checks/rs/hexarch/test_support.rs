use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::adapters::outbound::fs::RealFileSystem;
use crate::app::core::project_walker::walk_project;
use crate::domain::project_tree::{DirEntry, ProjectTree};
use crate::domain::report::{CheckResult, Severity};

use super::dependency_facts::{self, DependencyFamilyFacts, EdgeKind};

const GOLDEN_REL: &str = "tests/fixtures/r_arch_01/golden";
pub const RUST_APPS: &[&str] = &["devctl", "backend", "worker"];
pub const INNER_HEX: &str = "apps/backend/crates/adapters/inbound/mcp/crates";

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

pub fn remove_dir(root: &Path, rel: &str) {
    std::fs::remove_dir_all(root.join(rel)).expect("remove dir");
}

pub fn create_dir(root: &Path, rel: &str) {
    std::fs::create_dir_all(root.join(rel)).expect("create dir");
}

pub fn empty_dir(root: &Path, rel: &str) {
    let dir = root.join(rel);
    if !dir.exists() {
        return;
    }
    for entry in std::fs::read_dir(dir).expect("read dir") {
        let entry = entry.expect("dir entry");
        let file_type = entry.file_type().expect("file type");
        if file_type.is_dir() {
            std::fs::remove_dir_all(entry.path()).expect("remove nested dir");
        } else {
            std::fs::remove_file(entry.path()).expect("remove nested file");
        }
    }
}

pub fn run_family(root: &Path) -> Vec<CheckResult> {
    let tree = walk_project(&RealFileSystem, root);
    super::check(&tree)
}

pub fn dir_entry(dirs: &[&str], files: &[&str]) -> DirEntry {
    DirEntry {
        dirs: dirs.iter().map(|dir| (*dir).to_owned()).collect(),
        files: files.iter().map(|file| (*file).to_owned()).collect(),
    }
}

pub fn project_tree(
    structure: Vec<(&str, DirEntry)>,
    content: Vec<(&str, &str)>,
) -> ProjectTree {
    ProjectTree {
        root: PathBuf::from("/tmp/hexarch"),
        structure: structure
            .into_iter()
            .map(|(rel, entry)| (rel.to_owned(), entry))
            .collect::<BTreeMap<_, _>>(),
        content: content
            .into_iter()
            .map(|(rel, body)| (rel.to_owned(), body.to_owned()))
            .collect::<BTreeMap<_, _>>(),
    }
}

pub fn dependency_facts(tree: &ProjectTree) -> DependencyFamilyFacts {
    dependency_facts::collect(tree)
}

pub fn find_edge<'a>(
    facts: &'a DependencyFamilyFacts,
    source_rel_dir: &str,
    alias: &str,
    kind: EdgeKind,
) -> &'a super::dependency_facts::DependencyEdgeFacts {
    facts.edges
        .iter()
        .find(|edge| edge.source_rel_dir == source_rel_dir && edge.dep_alias == alias && edge.kind == kind)
        .expect("expected dependency edge")
}

pub fn errors_by_id<'a>(results: &'a [CheckResult], id: &str) -> Vec<&'a CheckResult> {
    results
        .iter()
        .filter(|result| result.id == id && result.severity == Severity::Error)
        .collect()
}

pub fn assert_no_error(results: &[CheckResult], id: &str) {
    let errors = errors_by_id(results, id);
    assert!(errors.is_empty(), "expected no {id} errors, got: {errors:#?}");
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
