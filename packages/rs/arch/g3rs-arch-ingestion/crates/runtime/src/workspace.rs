use glob::Pattern;
use toml::Value;

use g3rs_arch_types::types::{
    G3RsArchCrateNode, G3RsArchCrateStructure, G3RsArchDependencyCounts, G3RsArchFeatureContract,
};

use crate::error::G3RsArchIngestionError;
use crate::view::CrawlView;

pub(crate) fn collect_crate_nodes(
    view: &CrawlView<'_>,
) -> Result<Vec<G3RsArchCrateNode>, G3RsArchIngestionError> {
    let mut cargo_dirs = discover_crate_dirs(view)?;
    cargo_dirs.sort();
    cargo_dirs.dedup();
    let crate_dirs = cargo_dirs.iter().map(String::as_str).collect::<Vec<_>>();

    let mut nodes = cargo_dirs
        .iter()
        .map(|dir| build_crate_node(view, dir, &crate_dirs))
        .collect::<Result<Vec<_>, _>>()?;

    let rel_dirs = nodes
        .iter()
        .map(|node| node.rel_dir.clone())
        .collect::<Vec<_>>();
    for rel_dir in rel_dirs {
        let parent = find_parent_dir(&rel_dir, &nodes);
        if let Some(node) = nodes.iter_mut().find(|node| node.rel_dir == rel_dir) {
            node.parent_rel_dir = parent;
        }
    }

    Ok(nodes)
}

fn discover_crate_dirs(view: &CrawlView<'_>) -> Result<Vec<String>, G3RsArchIngestionError> {
    let Some(root_entry) = view.entry("Cargo.toml") else {
        return Ok(Vec::new());
    };
    if !root_entry.readable {
        return Err(G3RsArchIngestionError::Unreadable {
            path: root_entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    let content =
        view.read_file("Cargo.toml")
            .map_err(|err| G3RsArchIngestionError::Unreadable {
                path: root_entry.path.abs_path.clone(),
                reason: err.to_string(),
            })?;
    let parsed =
        toml::from_str::<Value>(&content).map_err(|err| G3RsArchIngestionError::ParseFailed {
            path: root_entry.path.abs_path.clone(),
            reason: err.to_string(),
        })?;

    let mut dirs = Vec::new();
    if parsed.get("package").is_some() {
        dirs.push(String::new());
    }
    dirs.extend(select_workspace_member_dirs(view, &parsed)?);

    Ok(dirs)
}

fn select_workspace_member_dirs(
    view: &CrawlView<'_>,
    root_manifest: &Value,
) -> Result<Vec<String>, G3RsArchIngestionError> {
    let Some(workspace) = root_manifest.get("workspace").and_then(Value::as_table) else {
        return Ok(Vec::new());
    };

    let member_patterns = workspace
        .get("members")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(|pattern| {
            Pattern::new(pattern).map_err(|err| G3RsArchIngestionError::ParseFailed {
                path: view.root_abs_path().join("Cargo.toml"),
                reason: format!("invalid workspace member pattern `{pattern}`: {err}"),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let exclude_patterns = workspace
        .get("exclude")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(|pattern| {
            Pattern::new(pattern).map_err(|err| G3RsArchIngestionError::ParseFailed {
                path: view.root_abs_path().join("Cargo.toml"),
                reason: format!("invalid workspace exclude pattern `{pattern}`: {err}"),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let all_matching_member_dirs = view
        .all_dir_rels()
        .filter(|rel_dir| !rel_dir.is_empty())
        .filter(|rel_dir| view.file_exists(&CrawlView::join_rel(rel_dir, "Cargo.toml")))
        .filter(|rel_dir| {
            member_patterns
                .iter()
                .any(|pattern| pattern.matches(rel_dir))
        })
        .map(str::to_owned)
        .collect::<Vec<_>>();

    for pattern in workspace
        .get("members")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
    {
        let parsed_pattern =
            Pattern::new(pattern).map_err(|err| G3RsArchIngestionError::ParseFailed {
                path: view.root_abs_path().join("Cargo.toml"),
                reason: format!("invalid workspace member pattern `{pattern}`: {err}"),
            })?;
        if !all_matching_member_dirs
            .iter()
            .any(|member_dir| parsed_pattern.matches(member_dir))
        {
            return Err(G3RsArchIngestionError::ParseFailed {
                path: view.root_abs_path().join("Cargo.toml"),
                reason: format!(
                    "workspace member pattern `{pattern}` did not resolve to any Cargo.toml"
                ),
            });
        }
    }

    Ok(all_matching_member_dirs
        .into_iter()
        .filter(|rel_dir| {
            !exclude_patterns
                .iter()
                .any(|pattern| pattern.matches(rel_dir))
        })
        .collect())
}

fn build_crate_node(
    view: &CrawlView<'_>,
    dir: &str,
    crate_dirs: &[&str],
) -> Result<G3RsArchCrateNode, G3RsArchIngestionError> {
    let cargo_rel_path = CrawlView::join_rel(dir, "Cargo.toml");
    let entry = view
        .entry(&cargo_rel_path)
        .ok_or_else(|| G3RsArchIngestionError::Unreadable {
            path: view.root_abs_path().join(&cargo_rel_path),
            reason: "selected Cargo.toml missing from crawl".to_owned(),
        })?;
    if !entry.readable {
        return Err(G3RsArchIngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    let content =
        view.read_file(&cargo_rel_path)
            .map_err(|err| G3RsArchIngestionError::Unreadable {
                path: entry.path.abs_path.clone(),
                reason: err.to_string(),
            })?;

    let parsed = toml::from_str::<Value>(&content).ok();
    let parse_error = toml::from_str::<Value>(&content)
        .err()
        .map(|err| err.to_string());

    let has_package = parsed
        .as_ref()
        .and_then(|value| value.get("package"))
        .is_some();
    let has_workspace = parsed
        .as_ref()
        .and_then(|value| value.get("workspace"))
        .is_some();
    let package_name = parsed
        .as_ref()
        .and_then(|value| value.get("package"))
        .and_then(|package| package.get("name"))
        .and_then(Value::as_str)
        .map(str::to_owned);
    let shared = parsed
        .as_ref()
        .and_then(|value| value.get("package"))
        .and_then(|package| package.get("metadata"))
        .and_then(|metadata| metadata.get("guardrail3"))
        .and_then(|guardrail| guardrail.get("shared"))
        .and_then(Value::as_bool)
        .unwrap_or(false);

    let custom_lib_path = parsed
        .as_ref()
        .and_then(|value| value.get("lib"))
        .and_then(|lib| lib.get("path"))
        .and_then(Value::as_str);
    let default_lib = CrawlView::join_rel(dir, "src/lib.rs");
    let lib_rs_rel = if let Some(custom) = custom_lib_path {
        let full = CrawlView::join_rel(dir, custom);
        if view.file_exists(&full) {
            Some(full)
        } else {
            None
        }
    } else if view.file_exists(&default_lib) {
        Some(default_lib)
    } else {
        None
    };
    let has_lib_rs = lib_rs_rel.is_some();
    let has_main_rs = view.file_exists(&CrawlView::join_rel(dir, "src/main.rs"));

    let features = parsed
        .as_ref()
        .and_then(|value| value.get("features"))
        .and_then(Value::as_table);
    let has_default_feature = features.is_some_and(|table| table.contains_key("default"));
    let has_all_feature = features.is_some_and(|table| table.contains_key("all"));
    let all_feature_deps = feature_list(features.and_then(|table| table.get("all")));
    let default_feature_deps = feature_list(features.and_then(|table| table.get("default")));
    let (production_dependency_count, dev_dependency_count) =
        parsed.as_ref().map_or((0, 0), count_dependencies);
    let src_dir = CrawlView::join_rel(dir, "src");
    let (max_sibling_rs_file_count, max_sibling_dir_count) =
        if view.dir_contents(&src_dir).is_some() {
            measure_max_sibling_counts(view, &src_dir, dir, crate_dirs)
        } else {
            measure_max_sibling_counts(view, dir, dir, crate_dirs)
        };
    let max_module_depth = measure_module_depth(view, dir, crate_dirs);

    Ok(G3RsArchCrateNode {
        rel_dir: dir.to_owned(),
        cargo_rel_path,
        package_name,
        has_package,
        has_workspace,
        has_lib_rs,
        has_main_rs,
        lib_rs_rel,
        parent_rel_dir: None,
        shared,
        feature_contract: G3RsArchFeatureContract {
            has_default_feature,
            has_all_feature,
            all_feature_deps,
            default_feature_deps,
        },
        dependency_counts: G3RsArchDependencyCounts {
            production: production_dependency_count,
            dev: dev_dependency_count,
        },
        structure: G3RsArchCrateStructure {
            max_sibling_rs_file_count,
            max_sibling_dir_count,
            max_module_depth,
        },
        cargo_parse_error: parse_error,
    })
}

pub(crate) fn collect_rs_files_recursive(
    view: &CrawlView<'_>,
    root_dir: &str,
    dir: &str,
    crate_dirs: &[&str],
    rel_paths: &mut Vec<String>,
) {
    let Some(entry) = view.dir_contents(dir) else {
        return;
    };
    for file in entry.files() {
        if file.ends_with(".rs") {
            rel_paths.push(CrawlView::join_rel(dir, file));
        }
    }
    for subdir in entry.dirs() {
        let child_dir = CrawlView::join_rel(dir, subdir);
        if should_stop_at_nested_crate(view, root_dir, &child_dir, crate_dirs) {
            continue;
        }
        collect_rs_files_recursive(view, root_dir, &child_dir, crate_dirs, rel_paths);
    }
}

pub(crate) fn collect_dirs_recursive(
    view: &CrawlView<'_>,
    root_dir: &str,
    dir: &str,
    crate_dirs: &[&str],
    result: &mut Vec<String>,
) {
    result.push(dir.to_owned());
    let Some(entry) = view.dir_contents(dir) else {
        return;
    };
    for subdir in entry.dirs() {
        let child_dir = CrawlView::join_rel(dir, subdir);
        if should_stop_at_nested_crate(view, root_dir, &child_dir, crate_dirs) {
            continue;
        }
        collect_dirs_recursive(view, root_dir, &child_dir, crate_dirs, result);
    }
}

pub(crate) fn should_stop_at_nested_crate(
    view: &CrawlView<'_>,
    root_dir: &str,
    child_dir: &str,
    crate_dirs: &[&str],
) -> bool {
    if child_dir == root_dir {
        return false;
    }
    crate_dirs.iter().any(|crate_dir| *crate_dir == child_dir)
        || view.file_exists(&CrawlView::join_rel(child_dir, "Cargo.toml"))
}

pub(crate) fn find_parent_dir(rel_dir: &str, crate_nodes: &[G3RsArchCrateNode]) -> Option<String> {
    parent_of(crate_nodes, rel_dir).map(str::to_owned)
}

pub(crate) fn parent_of<'a>(
    crate_nodes: &'a [G3RsArchCrateNode],
    rel_dir: &str,
) -> Option<&'a str> {
    let mut current = rel_dir;
    loop {
        let Some((parent, _)) = current.rsplit_once('/') else {
            return crate_nodes
                .iter()
                .find(|node| node.rel_dir.is_empty() && !rel_dir.is_empty())
                .map(|node| node.rel_dir.as_str());
        };
        if let Some(node) = crate_nodes.iter().find(|node| node.rel_dir == parent) {
            return Some(node.rel_dir.as_str());
        }
        current = parent;
    }
}

pub(crate) fn is_inside(inner: &str, outer: &str) -> bool {
    if outer.is_empty() {
        return !inner.is_empty();
    }
    inner.starts_with(outer) && inner.as_bytes().get(outer.len()) == Some(&b'/')
}

pub(crate) fn normalize_path(base: &str, rel: &str) -> String {
    let mut parts = if base.is_empty() {
        Vec::new()
    } else {
        base.split('/').collect::<Vec<_>>()
    };
    for segment in rel.split('/') {
        match segment {
            ".." => {
                let _ = parts.pop();
            }
            "." | "" => {}
            value => parts.push(value),
        }
    }
    parts.join("/")
}

fn count_dependencies(parsed: &Value) -> (usize, usize) {
    let mut production_count = 0;
    let mut dev_count = 0;
    if let Some(deps) = parsed.get("dependencies").and_then(Value::as_table) {
        production_count += deps.len();
    }
    if let Some(deps) = parsed.get("build-dependencies").and_then(Value::as_table) {
        production_count += deps.len();
    }
    if let Some(deps) = parsed.get("dev-dependencies").and_then(Value::as_table) {
        dev_count += deps.len();
    }
    (production_count, dev_count)
}

fn measure_max_sibling_counts(
    view: &CrawlView<'_>,
    dir: &str,
    root_dir: &str,
    crate_dirs: &[&str],
) -> (usize, usize) {
    let Some(entry) = view.dir_contents(dir) else {
        return (0, 0);
    };
    let sibling_rs_file_count = entry
        .files()
        .iter()
        .filter(|file| file.ends_with(".rs"))
        .count();
    let sibling_dir_count = entry
        .dirs()
        .iter()
        .filter(|subdir| {
            let child_dir = CrawlView::join_rel(dir, subdir);
            !should_stop_at_nested_crate(view, root_dir, &child_dir, crate_dirs)
        })
        .count();

    entry.dirs().iter().fold(
        (sibling_rs_file_count, sibling_dir_count),
        |(max_rs, max_dirs), subdir| {
            let child_dir = CrawlView::join_rel(dir, subdir);
            if should_stop_at_nested_crate(view, root_dir, &child_dir, crate_dirs) {
                return (max_rs, max_dirs);
            }

            let (child_max_rs, child_max_dirs) =
                measure_max_sibling_counts(view, &child_dir, root_dir, crate_dirs);
            (max_rs.max(child_max_rs), max_dirs.max(child_max_dirs))
        },
    )
}

fn measure_module_depth(view: &CrawlView<'_>, crate_dir: &str, crate_dirs: &[&str]) -> usize {
    let src_dir = CrawlView::join_rel(crate_dir, "src");
    let base_dir = if view.dir_contents(&src_dir).is_some() {
        src_dir
    } else {
        crate_dir.to_owned()
    };
    measure_depth_recursive(view, crate_dir, &base_dir, crate_dirs, 0)
}

fn measure_depth_recursive(
    view: &CrawlView<'_>,
    root_dir: &str,
    dir: &str,
    crate_dirs: &[&str],
    depth: usize,
) -> usize {
    let Some(entry) = view.dir_contents(dir) else {
        return depth;
    };
    let has_rs = entry.files().iter().any(|file| file.ends_with(".rs"));
    let current = if has_rs { depth } else { 0 };
    let max_child = entry
        .dirs()
        .iter()
        .map(|subdir| {
            let child_dir = CrawlView::join_rel(dir, subdir);
            if should_stop_at_nested_crate(view, root_dir, &child_dir, crate_dirs) {
                0
            } else {
                measure_depth_recursive(view, root_dir, &child_dir, crate_dirs, depth + 1)
            }
        })
        .max()
        .unwrap_or(0);
    current.max(max_child)
}

pub(crate) fn is_test_or_example_path(rel_path: &str) -> bool {
    rel_path
        .split('/')
        .any(|segment| matches!(segment, "tests" | "examples" | "benches" | "target"))
}

pub(crate) fn is_under_crate_src(dir: &str, crate_nodes: &[G3RsArchCrateNode]) -> bool {
    crate_nodes.iter().any(|node| {
        let src_prefix = if node.rel_dir.is_empty() {
            "src".to_owned()
        } else {
            format!("{}/src", node.rel_dir)
        };
        dir.starts_with(&src_prefix)
            && (dir.len() == src_prefix.len()
                || dir.as_bytes().get(src_prefix.len()) == Some(&b'/'))
    })
}

fn feature_list(value: Option<&Value>) -> Vec<String> {
    value
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default()
}
