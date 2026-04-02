use std::collections::BTreeMap;
use std::path::PathBuf;

use guardrail3_app_rs_family_view::{DirEntry, FamilyView as ProjectTree};

pub fn entry(dirs: &[&str], files: &[&str]) -> DirEntry {
    DirEntry::new(
        dirs.iter().map(|value| (*value).to_owned()).collect(),
        files.iter().map(|value| (*value).to_owned()).collect(),
        Vec::new(),
        Vec::new(),
    )
}

pub fn tree(structure: &[(&str, DirEntry)], content: &[(&str, &str)]) -> ProjectTree {
    let full_structure: BTreeMap<_, _> = structure
        .iter()
        .map(|(path, entry)| ((*path).to_owned(), entry.clone()))
        .collect();
    let full_content: BTreeMap<_, _> = content
        .iter()
        .map(|(path, body)| ((*path).to_owned(), (*body).to_owned()))
        .collect();
    ProjectTree::build(
        PathBuf::from("/tmp/project"),
        &full_structure,
        &full_content,
        &["".to_owned()],
        &[],
        &[],
        None,
    )
}
