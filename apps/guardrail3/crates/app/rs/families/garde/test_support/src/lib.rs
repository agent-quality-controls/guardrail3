use std::collections::BTreeMap;
use std::path::PathBuf;

use guardrail3_app_rs_family_mapper::{DirEntry, RsProjectSurface as ProjectTree};
use guardrail3_shared_fs::{create_dir_all, write_file};

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
    let root = std::env::temp_dir().join(unique);
    create_dir_all(&root).expect("create temp root");
    root
}

pub fn project_tree(
    structure: Vec<(&str, DirEntry)>,
    content: Vec<(&str, &str)>,
    root: PathBuf,
) -> ProjectTree {
    for (rel, entry) in &structure {
        let abs_dir = if rel.is_empty() {
            root.clone()
        } else {
            root.join(rel)
        };
        create_dir_all(&abs_dir).expect("create project dir");
        for dir in entry.dirs() {
            create_dir_all(&abs_dir.join(dir)).expect("create child dir");
        }
    }
    for (rel, body) in &content {
        let abs_path = root.join(rel);
        write_file(&abs_path, body).expect("write project file");
    }

    ProjectTree::new(
        root,
        structure
            .into_iter()
            .map(|(rel, entry)| (rel.to_owned(), entry))
            .collect::<BTreeMap<_, _>>(),
        content
            .into_iter()
            .map(|(rel, body)| (rel.to_owned(), body.to_owned()))
            .collect::<BTreeMap<_, _>>(),
    )
}

pub fn dir_entry(dirs: &[&str], files: &[&str]) -> DirEntry {
    DirEntry::new(
        dirs.iter().map(|value| (*value).to_owned()).collect(),
        files.iter().map(|value| (*value).to_owned()).collect(),
        Vec::new(),
        Vec::new(),
    )
}
