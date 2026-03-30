use std::collections::BTreeSet;
use std::path::{Component, Path, PathBuf};

use semver::{Version, VersionReq};

use guardrail3_domain_project_tree::ProjectTree;

pub fn package_table(parsed: &toml::Value) -> Option<&toml::Value> {
    parsed.get("package")
}

pub fn string_field_present(table: Option<&toml::Value>, field: &str) -> bool {
    table
        .and_then(|table| table.get(field))
        .and_then(toml::Value::as_str)
        .is_some_and(|value| !value.trim().is_empty())
}

fn bool_field_false(table: Option<&toml::Value>, field: &str) -> bool {
    table
        .and_then(|table| table.get(field))
        .and_then(toml::Value::as_bool)
        .is_some_and(|value| !value)
}

pub fn publish_setting_string(table: Option<&toml::Value>) -> Option<String> {
    let publish = table.and_then(|table| table.get("publish"))?;
    Some(match publish {
        toml::Value::Boolean(value) => value.to_string(),
        toml::Value::Array(values) => format!(
            "[{}]",
            values
                .iter()
                .filter_map(toml::Value::as_str)
                .map(|value| format!("\"{value}\""))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        _ => publish.to_string(),
    })
}

pub fn is_publishable(table: Option<&toml::Value>) -> bool {
    !bool_field_false(table, "publish")
        && !table
            .and_then(|table| table.get("publish"))
            .and_then(toml::Value::as_array)
            .is_some_and(|publish| publish.is_empty())
}

pub fn is_library_crate(tree: &ProjectTree, rel_dir: &str, parsed: &toml::Value) -> bool {
    parsed.get("lib").is_some() || tree.file_exists(&join_under_root(rel_dir, "src/lib.rs"))
}

pub fn is_binary_crate(tree: &ProjectTree, rel_dir: &str, parsed: &toml::Value) -> bool {
    if parsed.get("bin").and_then(toml::Value::as_array).is_some() {
        return true;
    }

    let autobins_disabled = package_table(parsed)
        .and_then(|package| package.get("autobins"))
        .and_then(toml::Value::as_bool)
        .is_some_and(|autobins| !autobins);

    !autobins_disabled
        && (tree.file_exists(&join_under_root(rel_dir, "src/main.rs"))
            || autodiscovered_bin_exists(tree, rel_dir))
}

pub fn binary_target_names(
    tree: &ProjectTree,
    rel_dir: &str,
    parsed: &toml::Value,
) -> BTreeSet<String> {
    let mut names = BTreeSet::new();

    if let Some(bins) = parsed.get("bin").and_then(toml::Value::as_array) {
        for bin in bins {
            if let Some(name) = bin.get("name").and_then(toml::Value::as_str) {
                let _ = names.insert(name.to_owned());
                continue;
            }
            if let Some(path) = bin.get("path").and_then(toml::Value::as_str)
                && let Some(name) = binary_name_from_path(path)
            {
                let _ = names.insert(name);
            }
        }
    }

    let autobins_disabled = package_table(parsed)
        .and_then(|package| package.get("autobins"))
        .and_then(toml::Value::as_bool)
        .is_some_and(|autobins| !autobins);
    if autobins_disabled {
        return names;
    }

    if tree.file_exists(&join_under_root(rel_dir, "src/main.rs"))
        && let Some(package_name) = package_table(parsed)
            .and_then(|package| package.get("name"))
            .and_then(toml::Value::as_str)
    {
        let _ = names.insert(package_name.to_owned());
    }

    let src_bin_rel = join_under_root(rel_dir, "src/bin");
    if let Some(src_bin) = tree.dir_contents(&src_bin_rel) {
        for file in src_bin.files() {
            if let Some(name) = binary_name_from_path(file) {
                let _ = names.insert(name);
            }
        }
        for dir in src_bin.dirs() {
            let nested = ProjectTree::join_rel(&src_bin_rel, dir);
            if tree.file_exists(&ProjectTree::join_rel(&nested, "main.rs")) {
                let _ = names.insert(dir.to_owned());
            }
        }
    }

    names
}

fn binary_name_from_path(path: &str) -> Option<String> {
    let path = Path::new(path);

    if path.file_name().and_then(|name| name.to_str()) == Some("main.rs") {
        return path
            .parent()
            .and_then(Path::file_name)
            .and_then(|name| name.to_str())
            .filter(|name| !name.is_empty())
            .map(str::to_owned);
    }

    path.file_stem()
        .and_then(|stem| stem.to_str())
        .filter(|stem| !stem.is_empty())
        .map(str::to_owned)
}

pub fn join_under_root(root_rel_dir: &str, child: &str) -> String {
    if root_rel_dir.is_empty() {
        child.to_owned()
    } else {
        ProjectTree::join_rel(root_rel_dir, child)
    }
}

pub fn resolve_manifest_relative_path(
    tree: &ProjectTree,
    manifest_rel_dir: &str,
    relative: &str,
) -> (String, PathBuf) {
    let abs = if manifest_rel_dir.is_empty() {
        tree.root().join(relative)
    } else {
        tree.root().join(manifest_rel_dir).join(relative)
    };
    let rel = abs
        .strip_prefix(tree.root())
        .map(normalize_relative_path)
        .unwrap_or_else(|_| relative.to_owned());
    (rel, abs)
}

fn normalize_relative_path(path: &Path) -> String {
    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                let _ = parts.pop();
            }
            Component::Normal(part) => parts.push(part.to_string_lossy().into_owned()),
            Component::RootDir | Component::Prefix(_) => {}
        }
    }
    parts.join("/")
}

pub fn readme_target_path(
    tree: &ProjectTree,
    manifest_rel_dir: &str,
    readme_field: Option<&str>,
) -> (String, PathBuf) {
    let readme_rel = readme_field.unwrap_or("README.md");
    resolve_manifest_relative_path(tree, manifest_rel_dir, readme_rel)
}

pub fn valid_semver(version: &str) -> bool {
    Version::parse(version).is_ok()
}

pub fn version_requirement_satisfied(actual: &str, req: &str) -> bool {
    let Ok(actual) = Version::parse(actual) else {
        return false;
    };
    let normalized = if req.trim_start().starts_with(['^', '~', '>', '<', '=']) {
        req.trim().to_owned()
    } else {
        format!("^{req}")
    };
    let Ok(req) = VersionReq::parse(&normalized) else {
        return false;
    };
    req.matches(&actual)
}

fn autodiscovered_bin_exists(tree: &ProjectTree, rel_dir: &str) -> bool {
    let src_bin_rel = join_under_root(rel_dir, "src/bin");
    let Some(src_bin) = tree.dir_contents(&src_bin_rel) else {
        return false;
    };

    if src_bin.files().iter().any(|file| file.ends_with(".rs")) {
        return true;
    }

    src_bin.dirs().iter().any(|dir| {
        let nested = ProjectTree::join_rel(&src_bin_rel, dir);
        tree.file_exists(&ProjectTree::join_rel(&nested, "main.rs"))
    })
}

pub fn path_file_exists(path: &Path) -> bool {
    guardrail3_shared_fs::metadata(path).is_some()
}
