use std::path::Path;

use guardrail3_adapters_outbound_fs::RealFileSystem;
use guardrail3_app_core::project_walker::walk_project;
use guardrail3_domain_project_tree::ProjectTree;

pub fn tempdir() -> tempfile::TempDir {
    tempfile::tempdir().expect("create tempdir")
}

pub fn write_file(root: &Path, rel_path: &str, content: &str) {
    let abs = root.join(rel_path);
    if let Some(parent) = abs.parent() {
        std::fs::create_dir_all(parent).expect("create parent");
    }
    std::fs::write(abs, content).expect("write file");
}

pub fn walk(root: &Path) -> ProjectTree {
    walk_project(&RealFileSystem, root)
}
