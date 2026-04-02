use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use guardrail3_adapters_outbound_fs::RealFileSystem;
use guardrail3_app_core::project_walker::walk_project;
use guardrail3_app_rs_family_view::{DirEntry, FamilyView as ProjectTree};
use guardrail3_shared_fs::{create_dir_all, metadata, remove_dir_all, write_file as write_fs_file};

pub fn write_file(root: &Path, rel: &str, content: &str) {
    let path = root.join(rel);
    write_fs_file(&path, content).expect("write file");
}

pub fn remove_dir(root: &Path, rel: &str) {
    remove_dir_all(&root.join(rel)).expect("remove dir");
}

pub fn create_dir(root: &Path, rel: &str) {
    create_dir_all(&root.join(rel)).expect("create dir");
}

pub fn empty_dir(root: &Path, rel: &str) {
    let dir = root.join(rel);
    if metadata(&dir).is_none() {
        return;
    }
    remove_dir_all(&dir).expect("remove dir contents");
    create_dir_all(&dir).expect("recreate dir");
}

pub fn walk(root: &Path) -> ProjectTree {
    let walked = walk_project(&RealFileSystem, root);
    ProjectTree::build(
        walked.root().clone(),
        walked.structure(),
        walked.content(),
        &["".to_owned()],
        &[],
        &[],
        None,
    )
}

pub fn dir_entry(dirs: &[&str], files: &[&str]) -> DirEntry {
    DirEntry::new(
        dirs.iter().map(|dir| (*dir).to_owned()).collect(),
        files.iter().map(|file| (*file).to_owned()).collect(),
        Vec::new(),
        Vec::new(),
    )
}

pub fn project_tree(structure: Vec<(&str, DirEntry)>, content: Vec<(&str, &str)>) -> ProjectTree {
    let full_structure: BTreeMap<_, _> = structure
        .into_iter()
        .map(|(rel, entry)| (rel.to_owned(), entry))
        .collect();
    let full_content: BTreeMap<_, _> = content
        .into_iter()
        .map(|(rel, body)| (rel.to_owned(), body.to_owned()))
        .collect();
    ProjectTree::build(
        PathBuf::from("/tmp/hexarch"),
        &full_structure,
        &full_content,
        &["".to_owned()],
        &[],
        &[],
        None,
    )
}
