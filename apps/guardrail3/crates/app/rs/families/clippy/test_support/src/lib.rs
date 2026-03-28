use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use guardrail3_domain_modules::clippy::build_clippy_toml;
use guardrail3_domain_project_tree::{DirEntry, ProjectTree};

static TEMP_ROOT_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
pub struct TempDir {
    path: PathBuf,
}

impl TempDir {
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }
}

fn temp_root(slug: &str) -> PathBuf {
    let unique_counter = TEMP_ROOT_COUNTER.fetch_add(1, Ordering::Relaxed);
    let unique = format!(
        "{}-{}-{}-{}",
        slug,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |duration| duration.as_nanos()),
        unique_counter,
    );
    let path = std::env::temp_dir().join(unique);
    create_dir_all(&path);
    path
}

#[must_use]
pub fn create_temp_dir(slug: &str) -> TempDir {
    TempDir {
        path: temp_root(slug),
    }
}

#[must_use]
pub fn copy_tree(src: &Path) -> TempDir {
    let temp_dir = create_temp_dir("copy-tree");
    copy_dir_recursive(src, temp_dir.path());
    temp_dir
}

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

#[must_use]
pub fn read_path(path: &Path) -> String {
    guardrail3_shared_fs::read_file_err(path).unwrap_or_default()
}

#[must_use]
pub fn read_file(root: &Path, rel: &str) -> String {
    read_path(&root.join(rel))
}

pub fn write_path(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        create_dir_all(parent);
    }
    assert!(
        guardrail3_shared_fs::write_file(path, content).is_ok(),
        "write file"
    );
}

pub fn write_file(root: &Path, rel: &str, content: &str) {
    write_path(&root.join(rel), content);
}

pub fn create_dir_all(path: &Path) {
    assert!(
        guardrail3_shared_fs::create_dir_all(path).is_ok(),
        "create dir"
    );
}

pub fn build_fixture_clippy_toml(
    profile_name: &str,
    is_pure_layer: bool,
    garde_enabled: bool,
    extra_methods: &str,
    extra_types: &str,
) -> String {
    build_clippy_toml(
        profile_name,
        is_pure_layer,
        garde_enabled,
        extra_methods,
        extra_types,
    )
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
            (
                "workspace/clippy.toml",
                build_fixture_clippy_toml("service", false, true, "", ""),
            ),
            (
                &format!("workspace/crates/core/{file_name}"),
                build_fixture_clippy_toml("service", false, true, "", ""),
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
            (
                "clippy.toml",
                build_fixture_clippy_toml("service", false, true, "", ""),
            ),
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
            (
                "clippy.toml",
                build_fixture_clippy_toml("service", false, true, "", ""),
            ),
            ("apps/libsite/clippy.toml", local_clippy_toml.into()),
        ],
    )
}

pub fn published_library_workspace_root_tree(local_clippy_toml: impl Into<String>) -> ProjectTree {
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
                "[package]\nname = \"core\"\npublish = true\n".to_owned(),
            ),
            (
                "clippy.toml",
                build_fixture_clippy_toml("service", false, true, "", ""),
            ),
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

pub fn package_library_workspace_root_tree(local_clippy_toml: impl Into<String>) -> ProjectTree {
    project_tree(
        vec![
            (
                "",
                dir_entry(&["packages"], &["guardrail3.toml", "clippy.toml"]),
            ),
            ("packages", dir_entry(&["shared-types"], &[])),
            (
                "packages/shared-types",
                dir_entry(&["crates"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("packages/shared-types/crates", dir_entry(&["core"], &[])),
            (
                "packages/shared-types/crates/core",
                dir_entry(&[], &["Cargo.toml"]),
            ),
        ],
        vec![
            (
                "guardrail3.toml",
                "[profile]\nname = \"service\"\n[rust.packages]\ntype = \"library\"\n".to_owned(),
            ),
            (
                "packages/shared-types/Cargo.toml",
                "[workspace]\nmembers = [\"crates/*\"]\n".to_owned(),
            ),
            (
                "packages/shared-types/crates/core/Cargo.toml",
                "[package]\nname = \"core\"\n".to_owned(),
            ),
            (
                "clippy.toml",
                build_fixture_clippy_toml("service", false, true, "", ""),
            ),
            (
                "packages/shared-types/clippy.toml",
                local_clippy_toml.into(),
            ),
        ],
    )
}

pub fn garde_disabled_root_tree(clippy_toml: impl Into<String>) -> ProjectTree {
    root_workspace_tree_with_guardrail(
        clippy_toml,
        "[profile]\nname = \"service\"\n[rust.checks]\ngarde = false",
    )
}

pub fn root_workspace_tree_with_cargo_config(
    cargo_config_rel: &str,
    cargo_config: impl Into<String>,
) -> ProjectTree {
    let cargo_config = cargo_config.into();
    project_tree(
        vec![
            ("", dir_entry(&[".cargo"], &["Cargo.toml", "clippy.toml"])),
            (".cargo", dir_entry(&[], &[cargo_config_rel])),
        ],
        vec![
            ("Cargo.toml", "[workspace]\nmembers = []".to_owned()),
            (
                "clippy.toml",
                build_fixture_clippy_toml("service", false, true, "", ""),
            ),
            (&format!(".cargo/{cargo_config_rel}"), cargo_config),
        ],
    )
}

pub fn nested_workspace_root_with_cargo_config(
    cargo_config_rel: &str,
    cargo_config: impl Into<String>,
) -> ProjectTree {
    let cargo_config = cargo_config.into();
    project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["backend"], &[])),
            (
                "apps/backend",
                dir_entry(&[".cargo", "crates"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("apps/backend/.cargo", dir_entry(&[], &[cargo_config_rel])),
            ("apps/backend/crates", dir_entry(&["core"], &[])),
            ("apps/backend/crates/core", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "apps/backend/Cargo.toml",
                "[workspace]\nmembers = [\"crates/*\"]".to_owned(),
            ),
            (
                "apps/backend/clippy.toml",
                build_fixture_clippy_toml("service", false, true, "", ""),
            ),
            (
                "apps/backend/crates/core/Cargo.toml",
                "[package]\nname = \"core\"\n".to_owned(),
            ),
            (
                &format!("apps/backend/.cargo/{cargo_config_rel}"),
                cargo_config,
            ),
        ],
    )
}

pub fn nested_workspace_member_with_cargo_config(
    cargo_config_rel: &str,
    cargo_config: impl Into<String>,
) -> ProjectTree {
    let cargo_config = cargo_config.into();
    project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["backend"], &[])),
            (
                "apps/backend",
                dir_entry(&["crates"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("apps/backend/crates", dir_entry(&["core"], &[])),
            (
                "apps/backend/crates/core",
                dir_entry(&[".cargo"], &["Cargo.toml"]),
            ),
            (
                "apps/backend/crates/core/.cargo",
                dir_entry(&[], &[cargo_config_rel]),
            ),
        ],
        vec![
            (
                "apps/backend/Cargo.toml",
                "[workspace]\nmembers = [\"crates/*\"]".to_owned(),
            ),
            (
                "apps/backend/clippy.toml",
                build_fixture_clippy_toml("service", false, true, "", ""),
            ),
            (
                "apps/backend/crates/core/Cargo.toml",
                "[package]\nname = \"core\"\n".to_owned(),
            ),
            (
                &format!("apps/backend/crates/core/.cargo/{cargo_config_rel}"),
                cargo_config,
            ),
        ],
    )
}

pub fn unrelated_nested_cargo_config_tree(
    cargo_config_rel: &str,
    cargo_config: impl Into<String>,
) -> ProjectTree {
    let cargo_config = cargo_config.into();
    project_tree(
        vec![
            ("", dir_entry(&["apps", "docs"], &[])),
            ("apps", dir_entry(&["backend"], &[])),
            (
                "apps/backend",
                dir_entry(&["crates"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("apps/backend/crates", dir_entry(&["core"], &[])),
            ("apps/backend/crates/core", dir_entry(&[], &["Cargo.toml"])),
            ("docs", dir_entry(&["guide"], &[])),
            ("docs/guide", dir_entry(&[".cargo"], &[])),
            ("docs/guide/.cargo", dir_entry(&[], &[cargo_config_rel])),
        ],
        vec![
            (
                "apps/backend/Cargo.toml",
                "[workspace]\nmembers = [\"crates/*\"]".to_owned(),
            ),
            (
                "apps/backend/clippy.toml",
                build_fixture_clippy_toml("service", false, true, "", ""),
            ),
            (
                "apps/backend/crates/core/Cargo.toml",
                "[package]\nname = \"core\"\n".to_owned(),
            ),
            (
                &format!("docs/guide/.cargo/{cargo_config_rel}"),
                cargo_config,
            ),
        ],
    )
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
    for entry in guardrail3_shared_fs::list_dir(src) {
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if guardrail3_shared_fs::metadata(&src_path).is_some_and(|metadata| metadata.is_dir()) {
            create_dir_all(&dst_path);
            copy_dir_recursive(&src_path, &dst_path);
        } else {
            let content = read_path(&src_path);
            write_path(&dst_path, &content);
        }
    }
}

#[cfg(test)]
mod lib_tests;
