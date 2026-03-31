use std::collections::BTreeMap;
use std::path::PathBuf;

use guardrail3_app_rs_family_mapper::{DirEntry, RsProjectSurface as ProjectTree};

pub fn entry(dirs: &[&str], files: &[&str]) -> DirEntry {
    DirEntry::new(
        dirs.iter().map(|value| (*value).to_owned()).collect(),
        files.iter().map(|value| (*value).to_owned()).collect(),
        Vec::new(),
        Vec::new(),
    )
}

pub fn tree(structure: &[(&str, DirEntry)], content: &[(&str, &str)]) -> ProjectTree {
    tree_at("/tmp/arch", structure, content)
}

pub fn tree_at(
    root: &str,
    structure: &[(&str, DirEntry)],
    content: &[(&str, &str)],
) -> ProjectTree {
    ProjectTree::new(
        PathBuf::from(root),
        structure
            .iter()
            .map(|(path, dir_entry)| ((*path).to_owned(), dir_entry.clone()))
            .collect::<BTreeMap<_, _>>(),
        content
            .iter()
            .map(|(path, body)| ((*path).to_owned(), (*body).to_owned()))
            .collect::<BTreeMap<_, _>>(),
    )
}
