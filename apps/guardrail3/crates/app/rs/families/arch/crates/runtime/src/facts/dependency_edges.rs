use std::path::PathBuf;

use guardrail3_app_rs_family_view::FamilyView as ProjectTree;

use super::crate_tree::CrateTree;

#[derive(Debug, Clone)]
#[allow(dead_code)] // Fields collected for rule expansion and diagnostics.
pub(crate) struct DependencyEdge {
    /// Source crate rel_dir.
    pub source_rel_dir: String,
    /// Source Cargo.toml rel path.
    pub source_cargo_rel: String,
    /// Dependency alias as declared in Cargo.toml.
    pub dep_alias: String,
    /// The raw `path = "..."` value from Cargo.toml.
    pub raw_path: String,
    /// Resolved target directory (normalized, repo-relative).
    pub resolved_target_rel: Option<String>,
    /// Whether the resolved target is a known crate (has Cargo.toml).
    pub target_is_crate: bool,
    /// The dependency section: "dependencies", "dev-dependencies", "build-dependencies".
    pub section: String,
}

#[derive(Debug, Default)]
pub(crate) struct DependencyEdges {
    pub edges: Vec<DependencyEdge>,
}

pub(super) fn collect(tree: &ProjectTree, crate_tree: &CrateTree) -> DependencyEdges {
    let mut edges = Vec::new();

    for node in crate_tree.nodes.values() {
        if node.cargo_parse_error.is_some() {
            continue;
        }
        let Some(content) = tree.file_content(&node.cargo_rel_path) else {
            continue;
        };
        let Ok(parsed) = toml::from_str::<toml::Value>(content) else {
            continue;
        };

        for section in &["dependencies", "dev-dependencies", "build-dependencies"] {
            if let Some(deps) = parsed.get(section).and_then(toml::Value::as_table) {
                collect_section_edges(
                    &node.rel_dir,
                    &node.cargo_rel_path,
                    section,
                    deps,
                    crate_tree,
                    &mut edges,
                );
            }
        }

        // Target-specific dependencies.
        if let Some(targets) = parsed.get("target").and_then(toml::Value::as_table) {
            for target_cfg in targets.values() {
                let Some(target_table) = target_cfg.as_table() else {
                    continue;
                };
                for section in &["dependencies", "dev-dependencies", "build-dependencies"] {
                    if let Some(deps) = target_table.get(*section).and_then(toml::Value::as_table) {
                        collect_section_edges(
                            &node.rel_dir,
                            &node.cargo_rel_path,
                            section,
                            deps,
                            crate_tree,
                            &mut edges,
                        );
                    }
                }
            }
        }
    }

    DependencyEdges { edges }
}

fn collect_section_edges(
    source_rel_dir: &str,
    source_cargo_rel: &str,
    section: &str,
    deps: &toml::map::Map<String, toml::Value>,
    crate_tree: &CrateTree,
    edges: &mut Vec<DependencyEdge>,
) {
    for (alias, value) in deps {
        let raw_path = extract_path(value);
        let Some(raw_path) = raw_path else {
            continue; // Not a path dependency — skip.
        };

        let resolved = resolve_dep_path(source_rel_dir, &raw_path);
        let target_is_crate = resolved
            .as_ref()
            .is_some_and(|r| crate_tree.nodes.contains_key(r));

        edges.push(DependencyEdge {
            source_rel_dir: source_rel_dir.to_owned(),
            source_cargo_rel: source_cargo_rel.to_owned(),
            dep_alias: alias.clone(),
            raw_path,
            resolved_target_rel: resolved,
            target_is_crate,
            section: section.to_owned(),
        });
    }
}

fn extract_path(value: &toml::Value) -> Option<String> {
    match value {
        toml::Value::Table(table) => table
            .get("path")
            .and_then(toml::Value::as_str)
            .map(str::to_owned),
        _ => None,
    }
}

fn resolve_dep_path(source_rel_dir: &str, raw_path: &str) -> Option<String> {
    // Build an absolute-ish path by joining source dir with the dependency path,
    // then normalize it to remove .. segments.
    let base = if source_rel_dir.is_empty() {
        PathBuf::new()
    } else {
        PathBuf::from(source_rel_dir)
    };
    let joined = base.join(raw_path);
    let normalized = normalize_path(&joined);
    let result = normalized.to_string_lossy().to_string();
    // Avoid empty string for root — use empty string convention.
    Some(result)
}

fn normalize_path(path: &std::path::Path) -> PathBuf {
    let mut components = Vec::new();
    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                let _ = components.pop();
            }
            std::path::Component::Normal(c) => {
                components.push(c.to_owned());
            }
            std::path::Component::CurDir => {}
            _ => {}
        }
    }
    components
        .iter()
        .collect::<PathBuf>()
}
