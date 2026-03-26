use std::collections::BTreeMap;
use std::path::PathBuf;

use guardrail3_domain_project_tree::{DirEntry, ProjectTree};

pub fn entry(dirs: &[&str], files: &[&str]) -> DirEntry {
    DirEntry {
        dirs: dirs.iter().map(|value| (*value).to_owned()).collect(),
        files: files.iter().map(|value| (*value).to_owned()).collect(),
        symlink_dirs: Vec::new(),
        symlink_files: Vec::new(),
    }
}

pub fn tree(structure: &[(&str, DirEntry)], content: &[(&str, &str)]) -> ProjectTree {
    tree_at("/tmp/arch", structure, content)
}

pub fn tree_at(
    root: &str,
    structure: &[(&str, DirEntry)],
    content: &[(&str, &str)],
) -> ProjectTree {
    ProjectTree {
        root: PathBuf::from(root),
        structure: structure
            .iter()
            .map(|(path, dir_entry)| ((*path).to_owned(), dir_entry.clone()))
            .collect::<BTreeMap<_, _>>(),
        content: content
            .iter()
            .map(|(path, body)| ((*path).to_owned(), (*body).to_owned()))
            .collect::<BTreeMap<_, _>>(),
    }
}
