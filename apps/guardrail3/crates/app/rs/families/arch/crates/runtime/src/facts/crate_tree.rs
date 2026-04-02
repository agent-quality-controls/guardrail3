use std::collections::BTreeMap;

use guardrail3_app_rs_family_mapper::RsProjectSurface as ProjectTree;

/// A node in the crate containment tree.
/// Each node is a directory with a Cargo.toml.
#[derive(Debug, Clone)]
#[allow(dead_code)] // Fields collected for rule expansion and diagnostics.
pub(crate) struct CrateNode {
    /// Repo-relative directory of this crate (e.g., "packages/my-lib").
    pub rel_dir: String,
    /// Repo-relative path to Cargo.toml.
    pub cargo_rel_path: String,
    /// Package name from Cargo.toml, if present.
    pub package_name: Option<String>,
    /// Whether Cargo.toml has [package].
    pub has_package: bool,
    /// Whether Cargo.toml has [workspace].
    pub has_workspace: bool,
    /// Whether src/lib.rs (or custom lib path) exists.
    pub has_lib_rs: bool,
    /// Whether src/main.rs (or custom bin path) exists.
    pub has_main_rs: bool,
    /// Repo-relative path to lib.rs if it exists.
    pub lib_rs_rel: Option<String>,
    /// Parent crate rel_dir (the nearest ancestor directory with Cargo.toml), if any.
    pub parent_rel_dir: Option<String>,
    /// Whether this crate has `shared = true` in [package.metadata.guardrail3].
    pub shared: bool,
    /// [features] section keys from Cargo.toml.
    pub feature_names: Vec<String>,
    /// Whether a `default` feature exists.
    pub has_default_feature: bool,
    /// Whether an `all` feature exists.
    pub has_all_feature: bool,
    /// Dependencies of the `all` feature (feature names it enables).
    pub all_feature_deps: Vec<String>,
    /// Dependencies of the `default` feature.
    pub default_feature_deps: Vec<String>,
    /// Complexity: number of direct dependencies in Cargo.toml.
    pub dependency_count: usize,
    /// Complexity: sibling .rs file count in src/ (or crate root if no src/).
    pub sibling_rs_file_count: usize,
    /// Complexity: sibling directory count in src/.
    pub sibling_dir_count: usize,
    /// Complexity: max module nesting depth.
    pub max_module_depth: usize,
    /// Parse error if Cargo.toml failed to parse.
    pub cargo_parse_error: Option<String>,
}

/// The full crate containment tree: maps rel_dir → CrateNode.
#[derive(Debug, Default)]
pub(crate) struct CrateTree {
    pub nodes: BTreeMap<String, CrateNode>,
}

impl CrateTree {
    /// Find the nearest ancestor crate of a given rel_dir.
    /// Walks up the directory tree looking for a Cargo.toml.
    pub fn parent_of(&self, rel_dir: &str) -> Option<&CrateNode> {
        let mut current = rel_dir;
        loop {
            let Some((parent, _)) = current.rsplit_once('/') else {
                return self.nodes.get("").filter(|_| !rel_dir.is_empty());
            };
            if let Some(node) = self.nodes.get(parent) {
                return Some(node);
            }
            current = parent;
        }
    }

    /// Check if `inner` is inside `outer`'s directory tree.
    pub fn is_inside(&self, inner: &str, outer: &str) -> bool {
        if outer.is_empty() {
            return !inner.is_empty();
        }
        inner.starts_with(outer) && inner.as_bytes().get(outer.len()) == Some(&b'/')
    }

    /// Find all intermediate crate boundaries between target and project root
    /// that the source is NOT inside of. Returns the first such boundary if any.
    pub fn boundary_violation(&self, source_rel: &str, target_rel: &str) -> Option<String> {
        let mut current = target_rel;
        loop {
            let Some((parent, _)) = current.rsplit_once('/') else {
                // Reached project root.
                if let Some(root_node) = self.nodes.get("") {
                    if root_node.rel_dir != target_rel
                        && root_node.rel_dir != source_rel
                        && !self.is_inside(source_rel, "")
                    {
                        return Some(String::new());
                    }
                }
                return None;
            };
            if let Some(boundary) = self.nodes.get(parent) {
                // Skip if source IS the boundary crate (parent accessing its own subtree).
                if boundary.rel_dir != target_rel
                    && boundary.rel_dir != source_rel
                    && !self.is_inside(source_rel, &boundary.rel_dir)
                {
                    return Some(boundary.rel_dir.clone());
                }
            }
            current = parent;
        }
    }

    /// Check if target is a direct child of source (source's Cargo.toml is an
    /// ancestor directory of target with no intermediate crate boundary).
    pub fn is_direct_child(&self, parent_rel: &str, child_rel: &str) -> bool {
        if !self.is_inside(child_rel, parent_rel) {
            return false;
        }
        // Walk up from child; the first crate boundary we hit should be parent.
        self.parent_of(child_rel)
            .is_some_and(|p| p.rel_dir == parent_rel)
    }
}

pub(super) fn collect(tree: &ProjectTree, root_dirs: &[String]) -> CrateTree {
    // Find Cargo.toml files within route roots by recursive dir_contents walk.
    let cargo_dirs = find_cargo_dirs(tree, root_dirs);

    let mut nodes = BTreeMap::new();
    for dir in &cargo_dirs {
        let cargo_rel = if dir.is_empty() {
            "Cargo.toml".to_owned()
        } else {
            format!("{dir}/Cargo.toml")
        };
        let Some(content) = tree.file_content(&cargo_rel) else {
            continue;
        };
        let parsed = match toml::from_str::<toml::Value>(content) {
            Ok(v) => Some(v),
            Err(e) => {
                let _ = nodes.insert(
                    dir.clone(),
                    make_error_node(dir, &cargo_rel, e.to_string()),
                );
                continue;
            }
        };
        let parsed = parsed.as_ref().unwrap();
        let node = build_node(tree, dir, &cargo_rel, parsed);
        let _ = nodes.insert(dir.clone(), node);
    }

    // Resolve parent pointers.
    let dirs: Vec<String> = nodes.keys().cloned().collect();
    for dir in &dirs {
        let parent = find_parent_dir(dir, &nodes);
        if let Some(node) = nodes.get_mut(dir) {
            node.parent_rel_dir = parent;
        }
    }

    CrateTree { nodes }
}

fn build_node(tree: &ProjectTree, dir: &str, cargo_rel: &str, parsed: &toml::Value) -> CrateNode {
    let has_package = parsed.get("package").is_some();
    let has_workspace = parsed.get("workspace").is_some();

    let package_name = parsed
        .get("package")
        .and_then(|p| p.get("name"))
        .and_then(toml::Value::as_str)
        .map(str::to_owned);

    let shared = parsed
        .get("package")
        .and_then(|p| p.get("metadata"))
        .and_then(|m| m.get("guardrail3"))
        .and_then(|g| g.get("shared"))
        .and_then(toml::Value::as_bool)
        .unwrap_or(false);

    // Detect lib.rs.
    let custom_lib_path = parsed
        .get("lib")
        .and_then(|l| l.get("path"))
        .and_then(toml::Value::as_str);
    let default_lib = ProjectTree::join_rel(dir, "src/lib.rs");
    let lib_rs_rel = if let Some(custom) = custom_lib_path {
        let full = ProjectTree::join_rel(dir, custom);
        if tree.file_exists(&full) {
            Some(full)
        } else {
            None
        }
    } else if tree.file_exists(&default_lib) {
        Some(default_lib)
    } else {
        None
    };
    let has_lib_rs = lib_rs_rel.is_some();

    // Detect main.rs.
    let default_main = ProjectTree::join_rel(dir, "src/main.rs");
    let has_main_rs = tree.file_exists(&default_main);

    // Features.
    let features_table = parsed.get("features").and_then(toml::Value::as_table);
    let feature_names: Vec<String> = features_table
        .map(|t| t.keys().cloned().collect())
        .unwrap_or_default();
    let has_default_feature = features_table
        .is_some_and(|t| t.contains_key("default"));
    let has_all_feature = features_table
        .is_some_and(|t| t.contains_key("all"));
    let all_feature_deps: Vec<String> = features_table
        .and_then(|t| t.get("all"))
        .and_then(toml::Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(toml::Value::as_str)
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default();
    let default_feature_deps: Vec<String> = features_table
        .and_then(|t| t.get("default"))
        .and_then(toml::Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(toml::Value::as_str)
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default();

    // Complexity measurements.
    let dependency_count = count_dependencies(parsed);
    let src_dir = ProjectTree::join_rel(dir, "src");
    let (sibling_rs_file_count, sibling_dir_count) = if tree.dir_exists(&src_dir) {
        count_siblings(tree, &src_dir)
    } else {
        count_siblings(tree, dir)
    };
    let max_module_depth = measure_module_depth(tree, dir, 0);

    CrateNode {
        rel_dir: dir.to_owned(),
        cargo_rel_path: cargo_rel.to_owned(),
        package_name,
        has_package,
        has_workspace,
        has_lib_rs,
        has_main_rs,
        lib_rs_rel,
        parent_rel_dir: None, // Filled in after all nodes collected.
        shared,
        feature_names,
        has_default_feature,
        has_all_feature,
        all_feature_deps,
        default_feature_deps,
        dependency_count,
        sibling_rs_file_count,
        sibling_dir_count,
        max_module_depth,
        cargo_parse_error: None,
    }
}

fn make_error_node(dir: &str, cargo_rel: &str, error: String) -> CrateNode {
    CrateNode {
        rel_dir: dir.to_owned(),
        cargo_rel_path: cargo_rel.to_owned(),
        package_name: None,
        has_package: false,
        has_workspace: false,
        has_lib_rs: false,
        has_main_rs: false,
        lib_rs_rel: None,
        parent_rel_dir: None,
        shared: false,
        feature_names: Vec::new(),
        has_default_feature: false,
        has_all_feature: false,
        all_feature_deps: Vec::new(),
        default_feature_deps: Vec::new(),
        dependency_count: 0,
        sibling_rs_file_count: 0,
        sibling_dir_count: 0,
        max_module_depth: 0,
        cargo_parse_error: Some(error),
    }
}

fn find_parent_dir(dir: &str, nodes: &BTreeMap<String, CrateNode>) -> Option<String> {
    let mut current = dir;
    loop {
        let Some((parent, _)) = current.rsplit_once('/') else {
            return nodes
                .get("")
                .filter(|_| !dir.is_empty())
                .map(|n| n.rel_dir.clone());
        };
        if nodes.contains_key(parent) {
            return Some(parent.to_owned());
        }
        current = parent;
    }
}

fn count_dependencies(parsed: &toml::Value) -> usize {
    let mut count = 0;
    if let Some(deps) = parsed.get("dependencies").and_then(toml::Value::as_table) {
        count += deps.len();
    }
    if let Some(deps) = parsed
        .get("build-dependencies")
        .and_then(toml::Value::as_table)
    {
        count += deps.len();
    }
    if let Some(deps) = parsed
        .get("dev-dependencies")
        .and_then(toml::Value::as_table)
    {
        count += deps.len();
    }
    count
}

fn count_siblings(tree: &ProjectTree, dir: &str) -> (usize, usize) {
    let Some(entry) = tree.dir_contents(dir) else {
        return (0, 0);
    };
    let rs_files = entry
        .files()
        .iter()
        .filter(|f| f.ends_with(".rs"))
        .count();
    let dirs = entry.dirs().len();
    (rs_files, dirs)
}

fn measure_module_depth(tree: &ProjectTree, dir: &str, _current_depth: usize) -> usize {
    let src_dir = ProjectTree::join_rel(dir, "src");
    let base = if tree.dir_exists(&src_dir) {
        &src_dir
    } else {
        dir
    };
    measure_depth_recursive(tree, base, 0)
}

fn measure_depth_recursive(tree: &ProjectTree, dir: &str, depth: usize) -> usize {
    let Some(entry) = tree.dir_contents(dir) else {
        return depth;
    };
    let has_rs = entry.files().iter().any(|f| f.ends_with(".rs"));
    let current = if has_rs { depth } else { 0 };
    let max_child = entry
        .dirs()
        .iter()
        .map(|d| {
            let child = ProjectTree::join_rel(dir, d);
            measure_depth_recursive(tree, &child, depth + 1)
        })
        .max()
        .unwrap_or(0);
    current.max(max_child)
}

/// Find all directories with Cargo.toml within the given root dirs.
/// Uses dir_contents recursion (known-path lookup) instead of dirs_with_file (discovery).
fn find_cargo_dirs(tree: &ProjectTree, root_dirs: &[String]) -> Vec<String> {
    let mut dirs = Vec::new();
    for root in root_dirs {
        find_cargo_dirs_recursive(tree, root, &mut dirs);
    }
    dirs.sort();
    dirs.dedup();
    dirs
}

fn find_cargo_dirs_recursive(tree: &ProjectTree, dir: &str, result: &mut Vec<String>) {
    let Some(entry) = tree.dir_contents(dir) else {
        return;
    };
    if entry.files().iter().any(|f| f == "Cargo.toml") {
        result.push(dir.to_owned());
    }
    for subdir in entry.dirs() {
        let child = ProjectTree::join_rel(dir, subdir);
        find_cargo_dirs_recursive(tree, &child, result);
    }
}
