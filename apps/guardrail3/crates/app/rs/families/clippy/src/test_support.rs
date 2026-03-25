use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use super::facts::{ClippyFacts, collect};
use super::inputs::ConfigClippyInput;
use guardrail3_adapters_outbound_fs::RealFileSystem;
use guardrail3_app_core::project_walker::walk_project;
use guardrail3_domain_modules::clippy::build_clippy_toml;
use guardrail3_domain_report::CheckResult;
use guardrail3_domain_project_tree::{DirEntry, ProjectTree};

const GOLDEN_REL: &str = "../../../../../tests/fixtures/r_arch_01/golden";

pub fn dir_entry(dirs: &[&str], files: &[&str]) -> DirEntry {
    DirEntry {
        dirs: dirs.iter().map(|dir| (*dir).to_owned()).collect(),
        files: files.iter().map(|file| (*file).to_owned()).collect(),
        symlink_dirs: Vec::new(),
        symlink_files: Vec::new(),
    }
}

pub fn project_tree(structure: Vec<(&str, DirEntry)>, content: Vec<(&str, String)>) -> ProjectTree {
    ProjectTree {
        root: PathBuf::from("/tmp/project"),
        structure: structure
            .into_iter()
            .map(|(rel, entry)| (rel.to_owned(), entry))
            .collect::<BTreeMap<_, _>>(),
        content: content
            .into_iter()
            .map(|(rel, content)| (rel.to_owned(), content))
            .collect::<BTreeMap<_, _>>(),
    }
}

pub fn collected_facts(tree: &ProjectTree) -> ClippyFacts {
    collect(tree)
}

pub fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(GOLDEN_REL)
}

pub fn copy_fixture() -> tempfile::TempDir {
    let tmp = tempfile::tempdir().expect("create tempdir");
    copy_dir_recursive(&fixture_root(), tmp.path());
    tmp
}

pub fn write_file(root: &Path, rel: &str, content: &str) {
    let path = root.join(rel);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create parent");
    }
    std::fs::write(path, content).expect("write file");
}

pub fn run_family(root: &Path) -> Vec<CheckResult> {
    let tree = walk_project(&RealFileSystem, root);
    super::check(&tree)
}

pub fn config_input<'a>(facts: &'a ClippyFacts, rel_path: &str) -> ConfigClippyInput<'a> {
    let config = facts
        .allowed_configs
        .iter()
        .find(|config| config.rel_path == rel_path)
        .expect("expected clippy config facts");
    ConfigClippyInput::new(config)
}

pub fn root_workspace_tree(clippy_toml: impl Into<String>) -> ProjectTree {
    project_tree(
        vec![("", dir_entry(&[], &["Cargo.toml", "clippy.toml"]))],
        vec![
            ("Cargo.toml", "[workspace]\nmembers = []".to_owned()),
            ("clippy.toml", clippy_toml.into()),
        ],
    )
}

pub fn root_workspace_tree_with_guardrail(
    clippy_toml: impl Into<String>,
    guardrail_toml: impl Into<String>,
) -> ProjectTree {
    project_tree(
        vec![(
            "",
            dir_entry(&[], &["Cargo.toml", "guardrail3.toml", "clippy.toml"]),
        )],
        vec![
            ("Cargo.toml", "[workspace]\nmembers = []".to_owned()),
            ("guardrail3.toml", guardrail_toml.into()),
            ("clippy.toml", clippy_toml.into()),
        ],
    )
}

pub fn nested_workspace_member_shadow_tree(file_name: &str) -> ProjectTree {
    project_tree(
        vec![
            ("", dir_entry(&["workspace"], &[])),
            (
                "workspace",
                dir_entry(&["crates"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("workspace/crates", dir_entry(&["core"], &[])),
            (
                "workspace/crates/core",
                dir_entry(&[], &["Cargo.toml", file_name]),
            ),
        ],
        vec![
            (
                "workspace/Cargo.toml",
                "[workspace]\nmembers = [\"crates/*\"]".to_owned(),
            ),
            (
                "workspace/crates/core/Cargo.toml",
                "[package]\nname = \"core\"".to_owned(),
            ),
            ("workspace/clippy.toml", canonical_clippy_toml().to_owned()),
            (
                &format!("workspace/crates/core/{file_name}"),
                canonical_clippy_toml().to_owned(),
            ),
        ],
    )
}

pub fn same_root_dual_config_tree() -> ProjectTree {
    project_tree(
        vec![(
            "",
            dir_entry(&[], &["Cargo.toml", "clippy.toml", ".clippy.toml"]),
        )],
        vec![
            ("Cargo.toml", "[workspace]\nmembers = []".to_owned()),
            ("clippy.toml", "max-struct-bools = 3".to_owned()),
            (".clippy.toml", "max-struct-bools = 4".to_owned()),
        ],
    )
}

pub fn incomplete_workspace_policy_root_tree() -> ProjectTree {
    project_tree(
        vec![
            ("", dir_entry(&["workspace"], &["clippy.toml"])),
            (
                "workspace",
                dir_entry(&["crates"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("workspace/crates", dir_entry(&["core"], &[])),
            ("workspace/crates/core", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "workspace/Cargo.toml",
                "[workspace]\nmembers = [\"crates/*\"]".to_owned(),
            ),
            (
                "workspace/crates/core/Cargo.toml",
                "[package]\nname = \"core\"".to_owned(),
            ),
            ("clippy.toml", canonical_clippy_toml().to_owned()),
            (
                "workspace/clippy.toml",
                r#"
too-many-lines-threshold = 75
disallowed-methods = []
disallowed-types = []
disallowed-macros = []
"#
                .to_owned(),
            ),
        ],
    )
}

pub fn library_workspace_root_tree(local_clippy_toml: impl Into<String>) -> ProjectTree {
    project_tree(
        vec![
            (
                "",
                dir_entry(&["apps"], &["guardrail3.toml", "clippy.toml"]),
            ),
            ("apps", dir_entry(&["libsite"], &[])),
            (
                "apps/libsite",
                dir_entry(&["crates"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("apps/libsite/crates", dir_entry(&["core"], &[])),
            ("apps/libsite/crates/core", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "guardrail3.toml",
                "[profile]\nname = \"service\"\n[rust.apps.libsite]\ntype = \"library\"".to_owned(),
            ),
            (
                "apps/libsite/Cargo.toml",
                "[workspace]\nmembers = [\"crates/*\"]".to_owned(),
            ),
            (
                "apps/libsite/crates/core/Cargo.toml",
                "[package]\nname = \"core\"".to_owned(),
            ),
            ("clippy.toml", canonical_clippy_toml().to_owned()),
            ("apps/libsite/clippy.toml", local_clippy_toml.into()),
        ],
    )
}

pub fn published_library_package_root_tree(local_clippy_toml: impl Into<String>) -> ProjectTree {
    project_tree(
        vec![(
            "",
            dir_entry(&[], &["Cargo.toml", "guardrail3.toml", "clippy.toml"]),
        )],
        vec![
            (
                "Cargo.toml",
                "[package]\nname = \"libcrate\"\npublish = true\n".to_owned(),
            ),
            (
                "guardrail3.toml",
                "[profile]\nname = \"library\"\n".to_owned(),
            ),
            ("clippy.toml", local_clippy_toml.into()),
        ],
    )
}

pub fn garde_disabled_root_tree(clippy_toml: impl Into<String>) -> ProjectTree {
    root_workspace_tree_with_guardrail(
        clippy_toml,
        "[profile]\nname = \"service\"\n[rust.checks]\ngarde = false",
    )
}

pub fn canonical_clippy_toml() -> String {
    build_clippy_toml("service", false, true, "", "")
}

pub fn remove_ban_path(clippy_toml: &str, key: &str, path: &str) -> String {
    let mut parsed = toml::from_str::<toml::Value>(clippy_toml).expect("valid clippy TOML");
    let entries = parsed
        .get_mut(key)
        .and_then(toml::Value::as_array_mut)
        .expect("expected ban array");
    entries.retain(|entry| {
        entry
            .get("path")
            .and_then(toml::Value::as_str)
            .or_else(|| entry.as_str())
            != Some(path)
    });
    toml::to_string(&parsed).expect("serialize clippy TOML")
}

pub fn prepend_ban_path(clippy_toml: &str, key: &str, path: &str, reason: &str) -> String {
    let mut parsed = toml::from_str::<toml::Value>(clippy_toml).expect("valid clippy TOML");
    let entries = parsed
        .get_mut(key)
        .and_then(toml::Value::as_array_mut)
        .expect("expected ban array");
    let mut entry = toml::map::Map::new();
    let _ = entry.insert("path".to_owned(), toml::Value::String(path.to_owned()));
    let _ = entry.insert("reason".to_owned(), toml::Value::String(reason.to_owned()));
    entries.insert(0, toml::Value::Table(entry));
    toml::to_string(&parsed).expect("serialize clippy TOML")
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

#[cfg(test)]
#[path = "clippy_test_support_tests.rs"]
mod tests;
