use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use super::facts::{DenyConfigFacts, DenyFacts, collect};
use super::inputs::{ForbiddenDenyConfigInput, SameRootConflictInput};
use crate::adapters::outbound::fs::RealFileSystem;
use crate::app::core::project_walker::walk_project;
use crate::domain::modules::deny::build_deny_toml;
use crate::domain::project_tree::{DirEntry, ProjectTree};
use crate::domain::report::CheckResult;

const GOLDEN_REL: &str = "tests/fixtures/r_arch_01/golden";

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

pub fn canonical_deny_toml_service() -> String {
    build_deny_toml("service", "", "", "")
}

pub fn canonical_deny_toml_library() -> String {
    build_deny_toml("library", "", "", "")
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
            ("workspace/deny.toml", canonical_deny_toml_service()),
            (&nested_rel_path, canonical_deny_toml_service()),
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
            ("Cargo.toml", "[package]\nname=\"crate\"\n".to_owned()),
            ("deny.toml", canonical_deny_toml_service()),
            (".deny.toml", canonical_deny_toml_service()),
            (".cargo/deny.toml", canonical_deny_toml_service()),
        ],
    )
}

pub fn config_facts(deny: &str) -> DenyConfigFacts {
    let (parsed, parse_error) = match toml::from_str::<toml::Value>(deny) {
        Ok(parsed) => (Some(parsed), None),
        Err(err) => (None, Some(err.to_string())),
    };
    DenyConfigFacts {
        policy_root_rel: String::new(),
        rel_path: "deny.toml".to_owned(),
        file_kind: "deny.toml".to_owned(),
        parsed,
        parse_error,
        profile_name: Some("service".to_owned()),
    }
}

pub fn config_facts_with_profile(deny: &str, profile_name: &str) -> DenyConfigFacts {
    let mut facts = config_facts(deny);
    facts.profile_name = Some(profile_name.to_owned());
    facts
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

pub fn collected_facts(tree: &ProjectTree) -> DenyFacts {
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

pub fn forbidden_input<'a>(facts: &'a DenyFacts, rel_path: &str) -> ForbiddenDenyConfigInput<'a> {
    let forbidden = facts
        .forbidden_configs
        .iter()
        .find(|config| config.rel_path == rel_path)
        .expect("expected forbidden deny config facts");
    ForbiddenDenyConfigInput::new(forbidden)
}

pub fn same_root_conflict_input<'a>(
    facts: &'a DenyFacts,
    policy_root_rel: &str,
) -> SameRootConflictInput<'a> {
    let conflict = facts
        .same_root_conflicts
        .iter()
        .find(|conflict| conflict.policy_root_rel == policy_root_rel)
        .expect("expected same-root deny conflict facts");
    SameRootConflictInput::new(conflict)
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
#[path = "deny_test_support_tests.rs"]
mod tests;
