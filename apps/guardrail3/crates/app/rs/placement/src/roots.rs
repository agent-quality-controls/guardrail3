use std::collections::BTreeSet;

use guardrail3_domain_project_tree::ProjectTree;
use toml::Value;

use crate::classification::{RustArchRole, RustRootPlacementRootFacts, classify_root};
use crate::overlap::{RustZoneOverlapFacts, collect_overlaps};

#[derive(Debug, Clone)]
pub struct RustRootPlacementInputFailureFacts {
    pub rel_path: String,
    pub message: String,
}

#[derive(Debug, Clone, Default)]
pub struct RustRootPlacementFacts {
    pub roots: Vec<RustRootPlacementRootFacts>,
    pub overlaps: Vec<RustZoneOverlapFacts>,
    pub input_failures: Vec<RustRootPlacementInputFailureFacts>,
}

#[must_use]
pub fn collect(tree: &ProjectTree) -> RustRootPlacementFacts {
    let mut root_dirs = BTreeSet::new();
    if tree.file_exists("Cargo.toml") && !is_excluded_live_root_dir("") {
        let _ = root_dirs.insert(String::new());
    }
    root_dirs.extend(
        tree.dirs_with_file("Cargo.toml")
            .into_iter()
            .filter(|rel_dir| !is_excluded_live_root_dir(rel_dir)),
    );

    let mut roots = Vec::new();
    let mut input_failures = Vec::new();
    let zone_context_prefix = zone_context_prefix(tree);

    for rel_dir in root_dirs {
        let cargo_rel_path = if rel_dir.is_empty() {
            "Cargo.toml".to_owned()
        } else {
            ProjectTree::join_rel(&rel_dir, "Cargo.toml")
        };
        if tree.file_exists(&cargo_rel_path) && tree.file_content(&cargo_rel_path).is_none() {
            input_failures.push(RustRootPlacementInputFailureFacts {
                rel_path: cargo_rel_path.clone(),
                message:
                    "Failed to read eligible live Cargo.toml for Rust root placement discovery."
                        .to_owned(),
            });
        }

        let arch_role = resolve_arch_role(tree, &cargo_rel_path, &mut input_failures);
        let placement_rel_dir = contextual_rel_dir(zone_context_prefix.as_deref(), &rel_dir);
        roots.push(classify_root(
            rel_dir,
            cargo_rel_path,
            &placement_rel_dir,
            arch_role,
        ));
    }

    roots.sort_by(|left, right| left.cargo_rel_path.cmp(&right.cargo_rel_path));
    let overlaps = collect_overlaps(&roots);

    RustRootPlacementFacts {
        roots,
        overlaps,
        input_failures,
    }
}

#[must_use]
pub fn is_excluded_live_root_dir(rel_dir: &str) -> bool {
    let segments: Vec<_> = rel_dir
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect();

    if segments.is_empty() {
        return false;
    }

    if segments.first() == Some(&"target") {
        return true;
    }

    if segments.len() >= 3 && segments[0] == ".claude" && segments[1] == "worktrees" {
        return true;
    }

    segments
        .windows(2)
        .any(|window| matches!(window, ["tests", "fixtures"] | ["tests", "snapshots"]))
}

fn resolve_arch_role(
    tree: &ProjectTree,
    cargo_rel_path: &str,
    input_failures: &mut Vec<RustRootPlacementInputFailureFacts>,
) -> Option<RustArchRole> {
    let Some(content) = tree.file_content(cargo_rel_path) else {
        return None;
    };

    let parsed = match toml::from_str::<Value>(content) {
        Ok(parsed) => parsed,
        Err(parse_error) => {
            input_failures.push(RustRootPlacementInputFailureFacts {
                rel_path: cargo_rel_path.to_owned(),
                message: format!(
                    "Failed to parse eligible live Cargo.toml for Rust root placement discovery: {parse_error}"
                ),
            });
            return None;
        }
    };

    arch_role_from_toml(&parsed)
}

fn arch_role_from_toml(parsed: &Value) -> Option<RustArchRole> {
    let value = parsed
        .get("package")
        .and_then(|value| value.get("metadata"))
        .and_then(|value| value.get("guardrail3"))
        .and_then(|value| value.get("arch_role"))
        .or_else(|| {
            parsed
                .get("workspace")
                .and_then(|value| value.get("metadata"))
                .and_then(|value| value.get("guardrail3"))
                .and_then(|value| value.get("arch_role"))
        })
        .and_then(Value::as_str)?;

    match value {
        "auxiliary" => Some(RustArchRole::Auxiliary),
        _ => None,
    }
}

fn zone_context_prefix(tree: &ProjectTree) -> Option<String> {
    let segments: Vec<_> = tree
        .root
        .iter()
        .filter_map(|segment| segment.to_str())
        .collect();

    segments
        .iter()
        .enumerate()
        .rev()
        .find_map(|(index, segment)| {
            if (*segment == "apps" || *segment == "packages") && index + 1 < segments.len() {
                Some(format!("{}/{}", segment, segments[index + 1]))
            } else {
                None
            }
        })
}

fn contextual_rel_dir(zone_context_prefix: Option<&str>, rel_dir: &str) -> String {
    match zone_context_prefix {
        Some(prefix) if rel_dir.is_empty() => prefix.to_owned(),
        Some(prefix) => format!("{prefix}/{rel_dir}"),
        None => rel_dir.to_owned(),
    }
}
