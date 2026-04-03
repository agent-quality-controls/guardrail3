#![allow(dead_code)]
mod helpers;
pub(super) fn results_for_test_root(root: &std::path::Path) -> Vec<guardrail3_domain_report::CheckResult> {
    helpers::results_for_test_root(root)
}
#[allow(dead_code, unused_imports)]
mod fail_closed;

use std::path::{Path, PathBuf};

#[allow(unused_imports)]
pub(super) use test_support::{
    create_dir, dir_entry, empty_dir, project_tree, remove_dir, walk, write_file,
};

const GOLDEN_REL: &str = "../../../../../../../tests/fixtures/full_golden";

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(GOLDEN_REL)
}

pub(super) fn copy_fixture() -> tempfile::TempDir {
    let tmp =
        tempfile::tempdir().expect("failed to create temporary directory for hexarch fixture copy");
    copy_dir_recursive(&fixture_root(), tmp.path());
    tmp
}

fn copy_dir_recursive(src: &Path, dst: &Path) {
    for entry in std::fs::read_dir(src).expect("failed to read source hexarch fixture directory") {
        let entry = entry.expect("failed to read entry from source hexarch fixture directory");
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            std::fs::create_dir_all(&dst_path)
                .expect("failed to create destination directory in copied hexarch fixture");
            copy_dir_recursive(&src_path, &dst_path);
        } else {
            let _ = std::fs::copy(&src_path, &dst_path)
                .expect("failed to copy file into temporary hexarch fixture");
        }
    }
}
