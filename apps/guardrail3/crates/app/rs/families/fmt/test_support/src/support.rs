use std::path::Path;

use guardrail3_adapters_outbound_fs::RealFileSystem;
use guardrail3_app_core::project_walker::walk_project;
use guardrail3_app_rs_family_view::FamilyView as ProjectTree;
use guardrail3_shared_fs::write_file as write_fs_file;

pub fn tempdir() -> tempfile::TempDir {
    tempfile::tempdir().expect("fmt test support should create a temporary fixture directory")
}

pub fn write_file(root: &Path, rel_path: &str, content: &str) {
    let abs = root.join(rel_path);
    write_fs_file(&abs, content).expect("fmt test support should write the requested fixture file");
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
        &[],
    )
}
