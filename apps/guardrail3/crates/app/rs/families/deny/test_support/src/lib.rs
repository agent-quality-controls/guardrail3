use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use guardrail3_adapters_outbound_fs::RealFileSystem;
use guardrail3_app_core::project_walker::walk_project;
use guardrail3_domain_modules::deny::build_deny_toml;
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

#[must_use]
pub fn copy_fixture(rel_from_manifest: &str) -> TempDir {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let stripped = rel_from_manifest.trim_start_matches("../");

    let candidate = manifest_dir
        .ancestors()
        .flat_map(|base| {
            [
                base.join(rel_from_manifest),
                base.join(stripped),
                base.join("apps/guardrail3").join(stripped),
            ]
        })
        .find(|candidate| candidate.exists())
        .expect("fixture not found from manifest directory");
    copy_tree(&candidate)
}

pub fn dir_entry(dirs: &[&str], files: &[&str]) -> DirEntry {
    DirEntry::new(
        dirs.iter().map(|dir| (*dir).to_owned()).collect(),
        files.iter().map(|file| (*file).to_owned()).collect(),
        Vec::new(),
        Vec::new(),
    )
}

pub fn project_tree(structure: Vec<(&str, DirEntry)>, content: Vec<(&str, String)>) -> ProjectTree {
    ProjectTree::new(
        PathBuf::from("/tmp/project"),
        structure
            .into_iter()
            .map(|(rel, entry)| (rel.to_owned(), entry))
            .collect::<BTreeMap<_, _>>(),
        content
            .into_iter()
            .map(|(rel, content)| (rel.to_owned(), content))
            .collect::<BTreeMap<_, _>>(),
    )
}

#[must_use]
pub fn build_fixture_deny_toml(profile_name: &str) -> String {
    build_deny_toml(profile_name, "", "", "")
}

pub fn nested_member_shadow_tree(file_name: &str) -> ProjectTree {
    let (core_dirs, core_files, nested_rel_path) =
        if let Some((dir, nested_file)) = file_name.split_once('/') {
            (
                vec![dir],
                vec!["Cargo.toml"],
                format!("workspace/crates/core/{dir}/{nested_file}"),
            )
        } else {
            (
                Vec::new(),
                vec!["Cargo.toml", file_name],
                format!("workspace/crates/core/{file_name}"),
            )
        };

    project_tree(
        vec![
            ("", dir_entry(&["workspace"], &[])),
            (
                "workspace",
                dir_entry(&["crates"], &["Cargo.toml", "deny.toml"]),
            ),
            ("workspace/crates", dir_entry(&["core"], &[])),
            ("workspace/crates/core", dir_entry(&core_dirs, &core_files)),
            (
                "workspace/crates/core/.cargo",
                dir_entry(&[], &["deny.toml"]),
            ),
        ],
        vec![
            (
                "workspace/Cargo.toml",
                "[workspace]\nmembers=[\"crates/*\"]".to_owned(),
            ),
            (
                "workspace/crates/core/Cargo.toml",
                "[package]\nname=\"core\"".to_owned(),
            ),
            ("workspace/deny.toml", build_fixture_deny_toml("service")),
            (&nested_rel_path, build_fixture_deny_toml("service")),
        ],
    )
}

pub fn same_root_conflict_tree() -> ProjectTree {
    project_tree(
        vec![
            (
                "",
                dir_entry(&[".cargo"], &["Cargo.toml", "deny.toml", ".deny.toml"]),
            ),
            (".cargo", dir_entry(&[], &["deny.toml"])),
        ],
        vec![
            (
                "Cargo.toml",
                "[workspace]\nmembers=[]\n[package]\nname=\"crate\"\n".to_owned(),
            ),
            ("deny.toml", build_fixture_deny_toml("service")),
            (".deny.toml", build_fixture_deny_toml("service")),
            (".cargo/deny.toml", build_fixture_deny_toml("service")),
        ],
    )
}

#[must_use]
pub fn walk(root: &Path) -> ProjectTree {
    walk_project(&RealFileSystem, root)
}

#[must_use]
pub fn read_path(path: &Path) -> String {
    guardrail3_shared_fs::read_file_err(path).unwrap_or_default()
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

pub fn remove_deny_ban(deny_toml: &str, name: &str) -> String {
    let mut parsed = toml::from_str::<toml::Value>(deny_toml).expect("valid deny TOML");
    let entries = parsed
        .get_mut("bans")
        .and_then(|bans| bans.get_mut("deny"))
        .and_then(toml::Value::as_array_mut)
        .expect("expected [bans].deny array");
    entries.retain(|entry| {
        entry
            .get("name")
            .or_else(|| entry.get("crate"))
            .and_then(toml::Value::as_str)
            .map(|value| value.split('@').next().unwrap_or(value) != name)
            .unwrap_or(true)
    });
    toml::to_string(&parsed).expect("serialize deny TOML")
}

pub fn set_deny_ban_wrappers(deny_toml: &str, name: &str, wrappers: &[&str]) -> String {
    let mut parsed = toml::from_str::<toml::Value>(deny_toml).expect("valid deny TOML");
    let entries = parsed
        .get_mut("bans")
        .and_then(|bans| bans.get_mut("deny"))
        .and_then(toml::Value::as_array_mut)
        .expect("expected [bans].deny array");
    let entry = entries
        .iter_mut()
        .find(|entry| {
            entry
                .get("name")
                .or_else(|| entry.get("crate"))
                .and_then(toml::Value::as_str)
                .map(|value| value.split('@').next().unwrap_or(value) == name)
                .unwrap_or(false)
        })
        .expect("expected deny ban entry");
    let table = entry.as_table_mut().expect("expected table ban entry");
    let _ = table.insert(
        "wrappers".to_owned(),
        toml::Value::Array(
            wrappers
                .iter()
                .map(|wrapper| toml::Value::String((*wrapper).to_owned()))
                .collect(),
        ),
    );
    toml::to_string(&parsed).expect("serialize deny TOML")
}

pub fn remove_deny_ban_reason(deny_toml: &str, name: &str) -> String {
    let mut parsed = toml::from_str::<toml::Value>(deny_toml).expect("valid deny TOML");
    let entries = parsed
        .get_mut("bans")
        .and_then(|bans| bans.get_mut("deny"))
        .and_then(toml::Value::as_array_mut)
        .expect("expected [bans].deny array");
    let entry = entries
        .iter_mut()
        .find(|entry| {
            entry
                .get("name")
                .or_else(|| entry.get("crate"))
                .and_then(toml::Value::as_str)
                .map(|value| value.split('@').next().unwrap_or(value) == name)
                .unwrap_or(false)
        })
        .expect("expected deny ban entry");
    let table = entry.as_table_mut().expect("expected table ban entry");
    let _ = table.remove("reason");
    toml::to_string(&parsed).expect("serialize deny TOML")
}

pub fn remove_allowed_license(deny_toml: &str, license: &str) -> String {
    let mut parsed = toml::from_str::<toml::Value>(deny_toml).expect("valid deny TOML");
    let entries = parsed
        .get_mut("licenses")
        .and_then(|licenses| licenses.get_mut("allow"))
        .and_then(toml::Value::as_array_mut)
        .expect("expected [licenses].allow array");
    entries.retain(|entry| entry.as_str() != Some(license));
    toml::to_string(&parsed).expect("serialize deny TOML")
}

pub fn add_allowed_license(deny_toml: &str, license: &str) -> String {
    let mut parsed = toml::from_str::<toml::Value>(deny_toml).expect("valid deny TOML");
    let entries = parsed
        .get_mut("licenses")
        .and_then(|licenses| licenses.get_mut("allow"))
        .and_then(toml::Value::as_array_mut)
        .expect("expected [licenses].allow array");
    entries.push(toml::Value::String(license.to_owned()));
    toml::to_string(&parsed).expect("serialize deny TOML")
}

pub fn set_license_confidence_threshold(deny_toml: &str, value: toml::Value) -> String {
    let mut parsed = toml::from_str::<toml::Value>(deny_toml).expect("valid deny TOML");
    let licenses = parsed
        .get_mut("licenses")
        .and_then(toml::Value::as_table_mut)
        .expect("expected [licenses] table");
    let _ = licenses.insert("confidence-threshold".to_owned(), value);
    toml::to_string(&parsed).expect("serialize deny TOML")
}

pub fn set_license_exceptions(deny_toml: &str, entries: Vec<toml::Value>) -> String {
    let mut parsed = toml::from_str::<toml::Value>(deny_toml).expect("valid deny TOML");
    let licenses = parsed
        .get_mut("licenses")
        .and_then(toml::Value::as_table_mut)
        .expect("expected [licenses] table");
    let _ = licenses.insert("exceptions".to_owned(), toml::Value::Array(entries));
    toml::to_string(&parsed).expect("serialize deny TOML")
}

pub fn set_bans_allow_entries(deny_toml: &str, entries: Vec<toml::Value>) -> String {
    let mut parsed = toml::from_str::<toml::Value>(deny_toml).expect("valid deny TOML");
    let bans = parsed
        .get_mut("bans")
        .and_then(toml::Value::as_table_mut)
        .expect("expected [bans] table");
    let _ = bans.insert("allow".to_owned(), toml::Value::Array(entries));
    toml::to_string(&parsed).expect("serialize deny TOML")
}

pub fn set_advisory_ignores(deny_toml: &str, entries: Vec<toml::Value>) -> String {
    let mut parsed = toml::from_str::<toml::Value>(deny_toml).expect("valid deny TOML");
    let advisories = parsed
        .get_mut("advisories")
        .and_then(toml::Value::as_table_mut)
        .expect("expected [advisories] table");
    let _ = advisories.insert("ignore".to_owned(), toml::Value::Array(entries));
    toml::to_string(&parsed).expect("serialize deny TOML")
}

pub fn add_deny_ban_entry(deny_toml: &str, entry: toml::Value) -> String {
    let mut parsed = toml::from_str::<toml::Value>(deny_toml).expect("valid deny TOML");
    let entries = parsed
        .get_mut("bans")
        .and_then(|bans| bans.get_mut("deny"))
        .and_then(toml::Value::as_array_mut)
        .expect("expected [bans].deny array");
    entries.push(entry);
    toml::to_string(&parsed).expect("serialize deny TOML")
}

pub fn add_skip_entry(deny_toml: &str, entry: toml::Value) -> String {
    let mut parsed = toml::from_str::<toml::Value>(deny_toml).expect("valid deny TOML");
    let entries = parsed
        .get_mut("bans")
        .and_then(|bans| bans.get_mut("skip"))
        .and_then(toml::Value::as_array_mut)
        .expect("expected [bans].skip array");
    entries.push(entry);
    toml::to_string(&parsed).expect("serialize deny TOML")
}

pub fn set_feature_entries(deny_toml: &str, entries: Vec<toml::Value>) -> String {
    let mut parsed = toml::from_str::<toml::Value>(deny_toml).expect("valid deny TOML");
    let bans = parsed
        .get_mut("bans")
        .and_then(toml::Value::as_table_mut)
        .expect("expected [bans] table");
    let _ = bans.insert("features".to_owned(), toml::Value::Array(entries));
    toml::to_string(&parsed).expect("serialize deny TOML")
}

pub fn set_private_ignore(deny_toml: &str, ignore: bool) -> String {
    let mut parsed = toml::from_str::<toml::Value>(deny_toml).expect("valid deny TOML");
    let licenses = parsed
        .get_mut("licenses")
        .and_then(toml::Value::as_table_mut)
        .expect("expected [licenses] table");
    let private = licenses
        .entry("private".to_owned())
        .or_insert_with(|| toml::Value::Table(toml::map::Map::new()))
        .as_table_mut()
        .expect("expected [licenses.private] table");
    let _ = private.insert("ignore".to_owned(), toml::Value::Boolean(ignore));
    toml::to_string(&parsed).expect("serialize deny TOML")
}

pub fn remove_section(deny_toml: &str, section: &str) -> String {
    let mut parsed = toml::from_str::<toml::Value>(deny_toml).expect("valid deny TOML");
    let root = parsed.as_table_mut().expect("expected root table");
    let _ = root.remove(section);
    toml::to_string(&parsed).expect("serialize deny TOML")
}

pub fn remove_section_key(deny_toml: &str, section: &str, key: &str) -> String {
    let mut parsed = toml::from_str::<toml::Value>(deny_toml).expect("valid deny TOML");
    let table = parsed
        .get_mut(section)
        .and_then(toml::Value::as_table_mut)
        .expect("expected section table");
    let _ = table.remove(key);
    toml::to_string(&parsed).expect("serialize deny TOML")
}

pub fn set_section_string(deny_toml: &str, section: &str, key: &str, value: &str) -> String {
    let mut parsed = toml::from_str::<toml::Value>(deny_toml).expect("valid deny TOML");
    let table = parsed
        .get_mut(section)
        .and_then(toml::Value::as_table_mut)
        .expect("expected section table");
    let _ = table.insert(key.to_owned(), toml::Value::String(value.to_owned()));
    toml::to_string(&parsed).expect("serialize deny TOML")
}

pub fn set_section_bool(deny_toml: &str, section: &str, key: &str, value: bool) -> String {
    let mut parsed = toml::from_str::<toml::Value>(deny_toml).expect("valid deny TOML");
    let table = parsed
        .get_mut(section)
        .and_then(toml::Value::as_table_mut)
        .expect("expected section table");
    let _ = table.insert(key.to_owned(), toml::Value::Boolean(value));
    toml::to_string(&parsed).expect("serialize deny TOML")
}

pub fn set_source_policy(deny_toml: &str, key: &str, value: &str) -> String {
    let mut parsed = toml::from_str::<toml::Value>(deny_toml).expect("valid deny TOML");
    let sources = parsed
        .get_mut("sources")
        .and_then(toml::Value::as_table_mut)
        .expect("expected [sources] table");
    let _ = sources.insert(key.to_owned(), toml::Value::String(value.to_owned()));
    toml::to_string(&parsed).expect("serialize deny TOML")
}

pub fn set_allow_git_sources(deny_toml: &str, entries: &[&str]) -> String {
    let mut parsed = toml::from_str::<toml::Value>(deny_toml).expect("valid deny TOML");
    let sources = parsed
        .get_mut("sources")
        .and_then(toml::Value::as_table_mut)
        .expect("expected [sources] table");
    let _ = sources.insert(
        "allow-git".to_owned(),
        toml::Value::Array(
            entries
                .iter()
                .map(|entry| toml::Value::String((*entry).to_owned()))
                .collect(),
        ),
    );
    toml::to_string(&parsed).expect("serialize deny TOML")
}

pub fn set_allow_registries(deny_toml: &str, entries: &[&str]) -> String {
    let mut parsed = toml::from_str::<toml::Value>(deny_toml).expect("valid deny TOML");
    let sources = parsed
        .get_mut("sources")
        .and_then(toml::Value::as_table_mut)
        .expect("expected [sources] table");
    let _ = sources.insert(
        "allow-registry".to_owned(),
        toml::Value::Array(
            entries
                .iter()
                .map(|entry| toml::Value::String((*entry).to_owned()))
                .collect(),
        ),
    );
    toml::to_string(&parsed).expect("serialize deny TOML")
}

fn copy_dir_recursive(src: &Path, dst: &Path) {
    for entry in guardrail3_shared_fs::list_dir(src) {
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            create_dir_all(&dst_path);
            copy_dir_recursive(&src_path, &dst_path);
        } else {
            let _ = guardrail3_shared_fs::copy_file(&src_path, &dst_path)
                .expect("copy deny fixture file");
        }
    }
}
