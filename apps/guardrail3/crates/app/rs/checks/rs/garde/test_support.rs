#![allow(dead_code)]

use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::domain::modules::clippy::build_clippy_toml;
use crate::domain::project_tree::{DirEntry, ProjectTree};
use crate::domain::report::CheckResult;

use super::facts::{
    DerivedBoundaryTypeFacts, GardeInputFailureFacts, GardeRootFacts, ManualDeserializeImplFacts,
    PolicyRootKind, QueryAsMacroFacts,
};
use super::parse::BoundaryKind;

pub fn temp_root(slug: &str) -> PathBuf {
    let unique = format!(
        "{}-{}-{}",
        slug,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    );
    std::env::temp_dir().join(unique)
}

pub fn project_tree(
    structure: Vec<(&str, DirEntry)>,
    content: Vec<(&str, &str)>,
    root: PathBuf,
) -> ProjectTree {
    ProjectTree {
        root,
        structure: structure
            .into_iter()
            .map(|(rel, entry)| (rel.to_owned(), entry))
            .collect::<BTreeMap<_, _>>(),
        content: content
            .into_iter()
            .map(|(rel, body)| (rel.to_owned(), body.to_owned()))
            .collect::<BTreeMap<_, _>>(),
    }
}

pub fn dir_entry(dirs: &[&str], files: &[&str]) -> DirEntry {
    DirEntry {
        dirs: dirs.iter().map(|value| (*value).to_owned()).collect(),
        files: files.iter().map(|value| (*value).to_owned()).collect(),
        symlink_dirs: Vec::new(),
        symlink_files: Vec::new(),
    }
}

pub fn root_facts(garde_dependency_present: bool) -> GardeRootFacts {
    GardeRootFacts {
        rel_dir: String::new(),
        cargo_rel_path: "Cargo.toml".to_owned(),
        kind: PolicyRootKind::WorkspaceRoot,
        garde_dependency_present,
        clippy_rel_path: Some("clippy.toml".to_owned()),
        clippy_parsed: None,
        clippy_parse_error: None,
    }
}

pub fn derived_target(boundary_kind: BoundaryKind, has_validate: bool) -> DerivedBoundaryTypeFacts {
    DerivedBoundaryTypeFacts {
        rel_path: "src/input.rs".to_owned(),
        line: 4,
        name: "Input".to_owned(),
        boundary_kind,
        boundary_macros: vec!["Deserialize".to_owned()],
        has_validate,
    }
}

pub fn manual_impl(needs_validate: bool, has_validate: bool) -> ManualDeserializeImplFacts {
    ManualDeserializeImplFacts {
        rel_path: "src/input.rs".to_owned(),
        line: 7,
        type_name: "Input".to_owned(),
        needs_validate,
        has_validate,
    }
}

pub fn query_as_macro() -> QueryAsMacroFacts {
    QueryAsMacroFacts {
        rel_path: "src/db.rs".to_owned(),
        line: 9,
        macro_name: "sqlx::query_as".to_owned(),
    }
}

pub fn input_failure(rel_path: &str, message: &str) -> GardeInputFailureFacts {
    GardeInputFailureFacts {
        rel_path: rel_path.to_owned(),
        message: message.to_owned(),
    }
}

pub fn remove_clippy_ban_path(clippy_toml: &str, key: &str, path: &str) -> String {
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

pub fn has_result<F>(results: &[CheckResult], id: &str, predicate: F) -> bool
where
    F: Fn(&CheckResult) -> bool,
{
    results
        .iter()
        .any(|result| result.id == id && predicate(result))
}

pub fn canonical_clippy_toml() -> String {
    build_clippy_toml("service", false, true, "", "")
}
