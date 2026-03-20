use std::path::Path;

use guardrail3::adapters::outbound::fs::RealFileSystem;
use guardrail3::app::rs::validate::arch::rs_arch_01::check_hex_arch_structure;
use guardrail3::domain::report::{CheckResult, Severity};

pub const GOLDEN: &str = "tests/fixtures/r_arch_01/golden";

pub fn copy_golden() -> tempfile::TempDir {
    let tmp = tempfile::tempdir().expect("create tempdir");
    copy_dir_recursive(Path::new(GOLDEN), tmp.path());
    tmp
}

fn copy_dir_recursive(src: &Path, dst: &Path) {
    for entry in std::fs::read_dir(src).expect("read golden dir") {
        let entry = entry.expect("read entry");
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            std::fs::create_dir_all(&dst_path).expect("create dir");
            copy_dir_recursive(&src_path, &dst_path);
        } else {
            std::fs::copy(&src_path, &dst_path).expect("copy file");
        }
    }
}

pub fn run_check(root: &Path) -> Vec<CheckResult> {
    let fs = RealFileSystem;
    let mut results = Vec::new();
    check_hex_arch_structure(&fs, root, &mut results);
    results
}

pub fn arch_01_errors(results: &[CheckResult]) -> Vec<&CheckResult> {
    results
        .iter()
        .filter(|r| r.id == "R-ARCH-01" && r.severity == Severity::Error)
        .collect()
}

pub fn remove_dir(root: &Path, rel: &str) {
    std::fs::remove_dir_all(root.join(rel)).expect("remove dir");
}

pub fn remove_file(root: &Path, rel: &str) {
    std::fs::remove_file(root.join(rel)).expect("remove file");
}

pub fn write_file(root: &Path, rel: &str, content: &str) {
    let path = root.join(rel);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create parent");
    }
    std::fs::write(&path, content).expect("write file");
}

pub fn assert_single_error(errors: &[&CheckResult], expected_title_fragment: &str) {
    assert_eq!(errors.len(), 1, "expected exactly 1 error, got {}: {errors:#?}", errors.len());
    assert!(
        errors[0].title.contains(expected_title_fragment),
        "expected title containing '{expected_title_fragment}', got: '{}'",
        errors[0].title
    );
}
