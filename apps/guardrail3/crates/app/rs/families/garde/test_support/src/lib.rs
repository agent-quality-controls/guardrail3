use std::collections::BTreeMap;
use std::path::PathBuf;

use guardrail3_domain_project_tree::{DirEntry, ProjectTree};

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
    std::fs::create_dir_all(&root).expect("create temp root");
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
        std::fs::create_dir_all(&abs_dir).expect("create project dir");
        for dir in &entry.dirs {
            std::fs::create_dir_all(abs_dir.join(dir)).expect("create child dir");
        }
    }
    for (rel, body) in &content {
        let abs_path = root.join(rel);
        if let Some(parent) = abs_path.parent() {
            std::fs::create_dir_all(parent).expect("create file parent");
        }
        std::fs::write(&abs_path, body).expect("write project file");
    }

    ProjectTree {
        root,
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

pub fn dir_entry(dirs: &[&str], files: &[&str]) -> DirEntry {
    DirEntry {
        dirs: dirs.iter().map(|value| (*value).to_owned()).collect(),
        files: files.iter().map(|value| (*value).to_owned()).collect(),
        symlink_dirs: Vec::new(),
        symlink_files: Vec::new(),
    }
}
