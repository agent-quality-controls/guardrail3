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
    ProjectTree::new(
        PathBuf::from("/tmp/project"),
        structure
            .iter()
            .map(|(path, entry)| ((*path).to_owned(), entry.clone()))
            .collect::<BTreeMap<_, _>>(),
        content
            .iter()
            .map(|(path, body)| ((*path).to_owned(), (*body).to_owned()))
            .collect::<BTreeMap<_, _>>(),
    )
}
