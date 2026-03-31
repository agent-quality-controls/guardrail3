use guardrail3_app_rs_family_mapper::RsProjectSurface as ProjectTree;
use syn::{Item, UseTree};

use super::{FacadeExportFacts, WorkspaceMemberFacts};

pub(super) fn parse_workspace_members(
    tree: &ProjectTree,
    package_rel_dir: &str,
    parsed: Option<&toml::Value>,
    cargo_parse_error: Option<&str>,
) -> (Vec<WorkspaceMemberFacts>, Option<String>) {
    if let Some(error) = cargo_parse_error {
        return (Vec::new(), Some(error.to_owned()));
    }
    let Some(parsed) = parsed else {
        return (Vec::new(), None);
    };
    let Some(workspace) = parsed.get("workspace") else {
        return (Vec::new(), None);
    };
    let Some(members_value) = workspace.get("members") else {
        return (Vec::new(), None);
    };
    let Some(members) = members_value.as_array() else {
        return (
            Vec::new(),
            Some("[workspace].members must be an array of strings".to_owned()),
        );
    };

    let mut outputs = Vec::new();
    for (index, member) in members.iter().enumerate() {
        let Some(raw) = member.as_str() else {
            return (
                Vec::new(),
                Some(format!(
                    "[workspace].members[{index}] must be a string, got {}",
                    toml_value_kind(member)
                )),
            );
        };
        let mut resolved_dirs = resolve_member_pattern(tree, package_rel_dir, raw);
        resolved_dirs.sort();
        resolved_dirs.dedup();
        outputs.push(WorkspaceMemberFacts { resolved_dirs });
    }

    (outputs, None)
}

pub(super) fn collect_facade_exports(
    tree: &ProjectTree,
    lib_rel_path: Option<&str>,
) -> Vec<FacadeExportFacts> {
    let Some(lib_rel_path) = lib_rel_path else {
        return Vec::new();
    };
    let Ok(content) = guardrail3_shared_fs::read_file_err(&tree.abs_path(lib_rel_path)) else {
        return Vec::new();
    };
    let Ok(file) = syn::parse_file(&content) else {
        return Vec::new();
    };

    let mut crate_names = Vec::new();
    for item in file.items {
        if let Item::Use(item_use) = item {
            if !matches!(item_use.vis, syn::Visibility::Public(_)) {
                continue;
            }
            collect_use_tree_crates(&item_use.tree, &mut crate_names);
        }
    }

    crate_names.sort();
    crate_names.dedup();
    crate_names
        .into_iter()
        .map(|crate_name| FacadeExportFacts { crate_name })
        .collect()
}

pub(super) fn facade_source_error(
    tree: &ProjectTree,
    lib_rel_path: Option<&str>,
) -> Option<String> {
    let Some(lib_rel_path) = lib_rel_path else {
        return None;
    };
    if !tree.file_exists(lib_rel_path) {
        return Some(format!("Facade source `{lib_rel_path}` does not exist."));
    }
    let content = guardrail3_shared_fs::read_file_err(&tree.abs_path(lib_rel_path))
        .map_err(|error| error.to_string())
        .ok()?;
    syn::parse_file(&content)
        .map(|_| None)
        .unwrap_or_else(|error| {
            Some(format!(
                "Facade source `{lib_rel_path}` could not be parsed as Rust: {error}"
            ))
        })
}

pub(super) fn package_name(value: &toml::Value) -> Option<String> {
    value
        .get("package")
        .and_then(|package| package.get("name"))
        .and_then(toml::Value::as_str)
        .map(str::to_owned)
}

pub(super) fn library_rel_path(base: &str, value: &toml::Value) -> Option<String> {
    let lib = value.get("lib")?;
    let path = lib.get("path")?.as_str()?;
    Some(normalize_path(base, path))
}

pub(super) fn library_crate_name(value: &toml::Value, package_name: &str) -> Option<String> {
    value
        .get("lib")
        .and_then(|lib| lib.get("name"))
        .and_then(toml::Value::as_str)
        .map(str::to_owned)
        .or_else(|| value.get("lib").map(|_| package_name.replace('-', "_")))
}

pub(super) fn normalize_path(base: &str, rel: &str) -> String {
    let mut parts = base
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    for segment in rel.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                let _ = parts.pop();
            }
            value => parts.push(value),
        }
    }
    parts.join("/")
}

pub(super) fn fallback_name(rel_dir: &str) -> String {
    rel_dir.rsplit('/').next().unwrap_or(rel_dir).to_owned()
}

fn resolve_member_pattern(
    tree: &ProjectTree,
    workspace_root_rel_dir: &str,
    member: &str,
) -> Vec<String> {
    let pattern = normalize_path(workspace_root_rel_dir, member);
    let mut matches = tree
        .matching_dir_rels(&pattern)
        .into_iter()
        .filter(|dir| tree.file_exists(&ProjectTree::join_rel(dir, "Cargo.toml")))
        .collect::<Vec<_>>();
    if matches.is_empty() && tree.file_exists(&ProjectTree::join_rel(&pattern, "Cargo.toml")) {
        matches.push(pattern);
    }
    matches
}

fn collect_use_tree_crates(tree: &UseTree, crate_names: &mut Vec<String>) {
    match tree {
        UseTree::Path(path) => {
            let ident = path.ident.to_string();
            if !matches!(ident.as_str(), "crate" | "self" | "super") {
                crate_names.push(ident);
            }
        }
        UseTree::Group(group) => {
            for item in &group.items {
                collect_use_tree_crates(item, crate_names);
            }
        }
        UseTree::Name(name) => crate_names.push(name.ident.to_string()),
        UseTree::Rename(rename) => crate_names.push(rename.ident.to_string()),
        UseTree::Glob(_) => {}
    }
}

fn toml_value_kind(value: &toml::Value) -> &'static str {
    match value {
        toml::Value::String(_) => "string",
        toml::Value::Integer(_) => "integer",
        toml::Value::Float(_) => "float",
        toml::Value::Boolean(_) => "boolean",
        toml::Value::Datetime(_) => "datetime",
        toml::Value::Array(_) => "array",
        toml::Value::Table(_) => "table",
    }
}
