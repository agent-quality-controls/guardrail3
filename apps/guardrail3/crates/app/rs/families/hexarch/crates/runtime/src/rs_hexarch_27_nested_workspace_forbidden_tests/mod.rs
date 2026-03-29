#![allow(dead_code, unused_imports)]
mod ownership;
mod workspace_policy;

use std::path::{Path, PathBuf};

pub(super) use test_support::{walk, write_file};

const GOLDEN_REL: &str = "../../../../../../../tests/fixtures/r_arch_01/golden";

pub(super) fn run_family(root: &Path) -> Vec<guardrail3_domain_report::CheckResult> {
    super::results_for_test_root(root)
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(GOLDEN_REL)
}

pub(super) fn copy_fixture() -> tempfile::TempDir {
    let tmp = tempfile::tempdir().expect("create tempdir");
    copy_dir_recursive(&fixture_root(), tmp.path());
    tmp
}

fn copy_dir_recursive(src: &Path, dst: &Path) {
    for entry in std::fs::read_dir(src).expect("read fixture dir") {
        let entry = entry.expect("read entry");
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            std::fs::create_dir_all(&dst_path).expect("create dst dir");
            copy_dir_recursive(&src_path, &dst_path);
        } else {
            let _ = std::fs::copy(&src_path, &dst_path).expect("copy fixture file");
        }
    }
}
