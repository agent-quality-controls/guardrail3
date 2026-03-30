mod inventory_contract;

use std::path::{Path, PathBuf};

use guardrail3_domain_report::CheckResult;

pub(super) use test_support::write_file;

const GOLDEN_REL: &str = "../../../../../../../tests/fixtures/r_arch_01/golden";

pub(super) fn run_family(root: &Path) -> Vec<CheckResult> {
    super::check_test_tree(&test_support::walk(root))
}

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
