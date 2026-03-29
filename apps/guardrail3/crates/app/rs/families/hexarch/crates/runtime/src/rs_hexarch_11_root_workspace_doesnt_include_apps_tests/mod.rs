#![allow(dead_code, unused_imports)]
mod fail_closed;
mod workspace_boundary;

use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_11_root_workspace_doesnt_include_apps::CheckResult;
use std::path::{Path, PathBuf};

#[allow(unused_imports)]
pub(super) use test_support::{
    create_dir, dir_entry, empty_dir, project_tree, remove_dir, walk, write_file,
};

const GOLDEN_REL: &str = "../../../../../../../tests/fixtures/r_arch_01/golden";
const RUST_APPS: &[&str] = &["devctl", "backend", "worker"];
const INNER_HEX_ROOT: &str = "apps/backend/crates/adapters/inbound/mcp/crates";

pub(super) fn run_family(root: &Path) -> Vec<CheckResult> {
    super::results_for_test_root(root)
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(GOLDEN_REL)
}

pub(super) struct HexarchFixture;

pub(super) const FIXTURE: HexarchFixture = HexarchFixture;

pub(super) fn hexarch_fixture() -> HexarchFixture {
    HexarchFixture
}

impl HexarchFixture {
    pub(super) fn apps(&self) -> &'static [&'static str] {
        RUST_APPS
    }

    pub(super) fn inner_hex_root(&self) -> &'static str {
        INNER_HEX_ROOT
    }

    pub(super) fn inner_hex(&self, suffix: &str) -> String {
        if suffix.is_empty() {
            INNER_HEX_ROOT.to_owned()
        } else {
            format!("{INNER_HEX_ROOT}/{suffix}")
        }
    }
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
